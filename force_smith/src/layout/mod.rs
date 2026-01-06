pub mod types;

use std::collections::HashMap;

use crate::{
    utils::vec2::Vec2,
    visualizer::{Parameter, Parameterized, VisualLayoutAlgorithm},
};
use types::*;

pub trait LayoutAlgorithm {
    fn set_graph(&mut self, graph: &BaseGraph);
    fn iterate(&mut self);
    fn get_positions(&self) -> Vec<Vec2>;
}

pub struct Layout<Vertex, Edge, Context, C> {
    graph: SpecializedGraph<Vertex, Edge>,
    graph_transformation_fn: GraphTransformationFn<Vertex, Edge>,
    context: C,
    forces: Forces<Vertex, Edge, Context>,
    displacements: Displacements,
    position_update_fn: PositionUpdateFn<Vertex, Context>,
}

/// Type-state: **Initial**
///
/// This state represents a layout immediately produced by the builder via
/// [`LayoutBuilder::to_layout`](crate::layout::LayoutBuilder::to_layout).
///
/// At this point, no graph or context has been assigned yet.
/// The layout is fully defined in structure but cannot yet be simulated.
///
/// From this state, you can transition into the next type-state by calling
/// [`set_graph`].
impl<Vertex, Edge, Context> Layout<Vertex, Edge, Context, NoneContext> {
    pub fn new(
        graph_transformation_fn: GraphTransformationFn<Vertex, Edge>,
        steps: Forces<Vertex, Edge, Context>,
        position_update_fn: PositionUpdateFn<Vertex, Context>,
    ) -> Self {
        Self {
            graph: SpecializedGraph::default(),
            graph_transformation_fn,
            context: NoneContext,
            forces: steps,
            displacements: Displacements::default(),
            position_update_fn,
        }
    }
}

/// Type-state: **Intermediate**
///
/// This state represents a partially initialized layout where either
/// the graph or context—or both—can still be set.
///
/// This type-state provides transition methods:
///
/// - [`set_context`] — assigns a custom context instance  
/// - [`set_default_context`] — assigns a context using its `Default` implementation
///
/// After both a graph and a context are assigned, the layout transitions to
/// the **Final** state, enabling iteration and position updates.
impl<Vertex, Edge, Context, C> Layout<Vertex, Edge, Context, C> {
    pub fn set_context(
        self,
        context: Context,
    ) -> Layout<Vertex, Edge, Context, SomeContext<Context>> {
        Layout {
            graph: self.graph,
            graph_transformation_fn: self.graph_transformation_fn,
            context: context.into(),
            forces: self.forces,
            displacements: self.displacements,
            position_update_fn: self.position_update_fn,
        }
    }

    pub fn set_default_context(self) -> Layout<Vertex, Edge, Context, SomeContext<Context>>
    where
        Context: Default,
    {
        Layout {
            graph: self.graph,
            graph_transformation_fn: self.graph_transformation_fn,
            context: Context::default().into(),
            forces: self.forces,
            displacements: self.displacements,
            position_update_fn: self.position_update_fn,
        }
    }
}

/// Type-state: **Final**
///
/// This state represents a fully-initialized layout with both a graph and a context
/// bound. The layout is now ready to perform simulation steps via [`iteration`].
///
/// - `C = SomeContext<Context>`
///
/// In this state, the layout is mutable, and you can safely iterate over steps
/// that apply forces, update positions, and evolve the graph state.
impl<Vertex, Edge, Context> LayoutAlgorithm for Layout<Vertex, Edge, Context, SomeContext<Context>>
where
    Vertex: Position,
{
    fn set_graph(&mut self, graph: &BaseGraph) {
        self.graph = (self.graph_transformation_fn)(graph);
        self.displacements = Displacements::fill_zero_sized(self.graph.vertices.len());
    }

    fn iterate(&mut self) {
        for step in self.forces.iter() {
            step.apply(
                &self.graph.vertices,
                &self.graph.edges,
                &self.context,
                &mut self.displacements,
            );
        }
        (self.position_update_fn)(
            &self.displacements,
            &mut self.graph.vertices,
            &mut self.context,
        )
    }

    fn get_positions(&self) -> Vec<Vec2> {
        self.graph.vertices.iter().map(|v| v.position()).collect()
    }
}

impl<Vertex, Edge, Context> Parameterized for Layout<Vertex, Edge, Context, SomeContext<Context>>
where
    Context: Parameterized,
{
    fn get_parameters(&self) -> HashMap<String, Parameter> {
        self.context.get_parameters()
    }

    fn update_parameters(&mut self, parameters: &HashMap<String, Parameter>) {
        self.context.update_parameters(parameters);
    }
}
impl<Vertex, Edge, Context> VisualLayoutAlgorithm
    for Layout<Vertex, Edge, Context, SomeContext<Context>>
where
    Context: Parameterized,
    Vertex: Position,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_fdl() -> Layout<Vec2, (usize, usize), (), SomeContext<()>> {
        Layout {
            graph: SpecializedGraph::default(),
            graph_transformation_fn: |g| g.into(),
            context: ().into(),
            forces: vec![Force {
                force_fn: |pair: VertexPair<'_, Vec2>, _: &()| {
                    pair.from.direction(pair.to) * pair.from.distance(pair.to)
                },
                applicator_fn: |vertices, edges, ctx, displacements, force_fn| {
                    for (from, to) in edges {
                        let displacement = force_fn(vertices.to_vertex_pair(*from, *to), ctx);
                        println!("Got here");
                        displacements[*from] += displacement;
                        displacements[*to] -= displacement;
                    }
                },
            }]
            .into(),
            displacements: Displacements::default(),
            position_update_fn: |displacements, vertices, _| {
                for idx in 0..vertices.len() {
                    vertices[idx] += displacements[idx];
                }
            },
        }
    }

    #[test]
    fn one_iteration_predictable_three_node_result() {
        let mut layout = simple_fdl();

        let base_graph = BaseGraph {
            vertices: vec![
                Vec2::new(0.0, 0.0), // node 0
                Vec2::new(1.0, 0.0), // node 1
                Vec2::new(2.0, 0.0), // node 2 (disconnected)
            ],
            edges: vec![(0, 1)],
        };

        layout.set_graph(&base_graph);

        layout.iterate();

        let got = layout.get_positions();

        // Expected:
        // edge (0,1) computes force = direction(0->1) * distance(0,1) = (1,0) * 1 = (1,0)
        // vertex 0 += (1,0) -> (1,0)
        // vertex 1 -= (1,0) -> (0,0)
        // vertex 2 unchanged -> (2,0)
        let expected = vec![
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
        ];

        assert_eq!(got, expected);
    }
}
