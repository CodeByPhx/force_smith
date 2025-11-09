pub mod types;

use crate::layout::{
    Layout,
    types::{GraphTransformationFn, NoneContext, PositionUpdateFn, Step, Steps},
};
use std::marker::PhantomData;
use types::*;

pub struct LayoutBuilder<Vertex, Edge, Context, G, P, CtxType = Context> {
    _ctx_type: PhantomData<CtxType>,
    graph_transformation_fn: G,
    position_update_fn: P,
    steps: Steps<Vertex, Edge, Context>,
}

/// Type-state: **Initial**
///
/// This state represents a newly created `LayoutBuilder` before any configuration has
/// been applied. Both the graph transformation and position update functions are
/// absent, represented by the marker types [`NoneGraphTransformationFn`] and
/// [`NonePositionUpdateFn`].
///
/// The builder starts here when you call [`LayoutBuilder::build`], and from this state
/// you can chain configuration methods such as:
///
/// - [`with_context_type`] to explicitly set the compile-time context type for better LSP support
/// - [`with_graph_transformation_fn`] to supply a graph transformation function
/// - [`with_position_update_fn`] to define how vertex positions are updated
///
/// # Example
///
/// ```rust, ignore
/// let builder = LayoutBuilder::build()
///     .with_context_type::<MyContext>()
///     .with_graph_transformation_fn(|g| g.into())
///     .with_position_update_fn(|disp, verts, ctx| { /* ... */ });
/// ```
impl<Vertex, Edge, Context>
    LayoutBuilder<Vertex, Edge, Context, NoneGraphTransformationFn, NonePositionUpdateFn>
{
    pub fn build()
    -> LayoutBuilder<Vertex, Edge, Context, NoneGraphTransformationFn, NonePositionUpdateFn> {
        LayoutBuilder {
            graph_transformation_fn: NoneGraphTransformationFn,
            position_update_fn: NonePositionUpdateFn,
            steps: Vec::new().into(),
            _ctx_type: PhantomData,
        }
    }
}

/// Type-state: **Intermediate**
///
/// This state represents a partially configured builder. Depending on which methods
/// have been called, the type parameters `G` and `P` will indicate whether the graph
/// transformation or position update functions have been provided.
///
/// The builder in this state supports method chaining to set additional functions or
/// steps. It also includes [`with_context_type`] to refine the compile-time `Context`
/// type for better inference and autocompletion in downstream function closures.
///
/// - `G` can be [`NoneGraphTransformationFn`] or [`SomeGraphTransformationFn`]
/// - `P` can be [`NonePositionUpdateFn`] or [`SomePositionUpdateFn`]
///
/// You can stay in this state until all required configuration pieces are provided.
/// Once both `G` and `P` are in the `Some*` state, the builder transitions to the
/// final state, enabling [`to_layout`].
impl<Vertex, Edge, Context, G, P> LayoutBuilder<Vertex, Edge, Context, G, P> {
    pub fn with_context_type<Ctx>(self) -> LayoutBuilder<Vertex, Edge, Context, G, P, Ctx> {
        LayoutBuilder {
            graph_transformation_fn: self.graph_transformation_fn,
            position_update_fn: self.position_update_fn,
            steps: self.steps,
            _ctx_type: PhantomData,
        }
    }

    pub fn with_graph_transformation_fn(
        self,
        graph_transformation_fn: GraphTransformationFn<Vertex, Edge>,
    ) -> LayoutBuilder<Vertex, Edge, Context, SomeGraphTransformationFn<Vertex, Edge>, P> {
        LayoutBuilder {
            graph_transformation_fn: graph_transformation_fn.into(),
            position_update_fn: self.position_update_fn,
            steps: self.steps,
            _ctx_type: self._ctx_type,
        }
    }

    pub fn with_position_update_fn(
        self,
        position_update_fn: PositionUpdateFn<Vertex, Context>,
    ) -> LayoutBuilder<Vertex, Edge, Context, G, SomePositionUpdateFn<Vertex, Context>> {
        LayoutBuilder {
            graph_transformation_fn: self.graph_transformation_fn,
            position_update_fn: position_update_fn.into(),
            steps: self.steps,
            _ctx_type: self._ctx_type,
        }
    }

    pub fn with_step(mut self, step: Step<Vertex, Edge, Context>) -> Self {
        self.steps.push(step);
        self
    }
}

/// Type-state: **Final**
///
/// This is the final state of the builder pattern, where both the graph
/// transformation function (`G`) and position update function (`P`) have been set.
/// The generics `G` and `P` are concrete marker types:
///
/// - `G = SomeGraphTransformationFn<Vertex, Edge>`
/// - `P = SomePositionUpdateFn<Vertex, Context>`
///
/// From this state, the builder can produce a finalized [`Layout`] instance by
/// calling [`to_layout`]. After this point, the builder is consumed and no further
/// configuration can be performed.
///
/// # Example
///
/// ```rust, ignore
/// let layout = LayoutBuilder::build()
///     .with_context_type::<MyContext>()
///     .with_graph_transformation_fn(|g| g.into())
///     .with_position_update_fn(|disp, verts, ctx| { /* ... */ })
///     .with_step(my_step)
///     .to_layout();
/// ```
impl<Vertex, Edge, Context>
    LayoutBuilder<
        Vertex,
        Edge,
        Context,
        SomeGraphTransformationFn<Vertex, Edge>,
        SomePositionUpdateFn<Vertex, Context>,
    >
{
    pub fn to_layout(self) -> Layout<Vertex, Edge, Context, NoneContext> {
        Layout::new(
            self.graph_transformation_fn.0,
            self.steps,
            self.position_update_fn.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        layout::{
            LayoutAlgorithm,
            types::{BaseGraph, ToVertexPair},
        },
        utils::vec2::Vec2,
    };

    fn setup_simple_fdl() -> Layout<Vec2, (usize, usize), (), NoneContext> {
        LayoutBuilder::build()
            .with_context_type::<()>()
            .with_graph_transformation_fn(|g| g.into())
            .with_position_update_fn(|displacements, vertices, _| {
                for idx in 0..vertices.len() {
                    vertices[idx] += displacements[idx];
                }
            })
            .with_step(Step {
                force_fn: |pair, _| pair.from.direction(pair.to) * pair.from.distance(pair.to),
                applicator_fn: |vertices, edges, ctx, displacements, force_fn| {
                    for (from, to) in edges {
                        let displacement = force_fn(vertices.to_vertex_pair(*from, *to), ctx);
                        displacements[*from] += displacement;
                        displacements[*to] -= displacement;
                    }
                },
            })
            .to_layout()
    }

    #[test]
    fn simple_fdl_runs() {
        let layout = setup_simple_fdl();

        let base_graph = BaseGraph {
            vertices: vec![
                Vec2::from_xy(0.0, 0.0),
                Vec2::from_xy(1.0, 0.0),
                Vec2::from_xy(2.0, 0.0),
            ],
            edges: vec![(0, 1)],
        };

        let mut layout = layout.set_default_context();
        layout.set_graph(&base_graph);
        layout.iterate();

        let result = layout.get_positions();
        println!("{:?}", result);

        assert_eq!(result[0], Vec2::from_xy(1.0, 0.0));
        assert_eq!(result[1], Vec2::from_xy(0.0, 0.0));
        assert_eq!(result[2], Vec2::from_xy(2.0, 0.0));
    }
}
