pub mod types;
pub mod visualizer_integration;

use bevy_math::Vec2;
use types::*;

pub trait LayoutAlgorithm {
    fn set_graph(&mut self, graph: &BaseGraph);
    fn iterate(&mut self);
    fn write_positions(&self, positions: &mut Vec<Vec2>);
}

pub struct Layout<Vertex: Position, Edge, Context, C> {
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
impl<Vertex: Position, Edge, Context> Layout<Vertex, Edge, Context, NoneContext> {
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
impl<Vertex: Position, Edge, Context, C> Layout<Vertex, Edge, Context, C> {
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
impl<Vertex: Position, Edge, Context> LayoutAlgorithm
    for Layout<Vertex, Edge, Context, SomeContext<Context>>
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

    fn write_positions(&self, positions: &mut Vec<Vec2>) {
        for (idx, position) in self.graph.vertices.iter().enumerate() {
            positions[idx] = *position.vec2_as_ref();
        }
    }
}
