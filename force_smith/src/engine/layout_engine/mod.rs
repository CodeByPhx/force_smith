use crate::prelude::*;

#[cfg(feature = "visualizer")]
mod visualizer_integration;

pub struct LayoutEngine<Vertex, Edge, Context> {
    graph: SpecializedGraph<Vertex, Edge>,
    graph_loading_fn: GraphLoadingFn<Vertex, Edge, Context>,
    context: Context,
    forces: Vec<Force<Vertex, Edge, Context>>,
    displacements: Displacements,
    position_update_fn: PositionUpdateFn<Vertex, Context>,
}
impl<Vertex, Edge, Context> LayoutEngine<Vertex, Edge, Context>
where
    Context: Default,
{
    pub fn new(
        graph_loading_fn: GraphLoadingFn<Vertex, Edge, Context>,
        forces: Vec<Force<Vertex, Edge, Context>>,
        position_update_fn: PositionUpdateFn<Vertex, Context>,
    ) -> Self {
        let mut ctx = Context::default();
        let graph = Graph::default();
        Self {
            graph: (graph_loading_fn)(&graph, &mut ctx),
            graph_loading_fn,
            context: ctx,
            forces,
            displacements: vec![Vec2::ZERO; graph.vertices].into(),
            position_update_fn,
        }
    }
}

impl<Vertex, Edge, Context> LayoutAlgorithm for LayoutEngine<Vertex, Edge, Context>
where
    Vertex: AsVec2,
    Context: Default,
    Edge: AsEdge,
{
    fn load_graph(&mut self, graph: &Graph) {
        self.context = Context::default();
        self.graph = (self.graph_loading_fn)(graph, &mut self.context);
        self.displacements = vec![Vec2::ZERO; self.graph.vertices.len()].into();
    }

    fn iterate(&mut self) {
        for step in &self.forces {
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
        );
    }

    fn get_positions(&self) -> Vec<bevy_math::Vec2> {
        self.graph
            .vertices
            .iter()
            .map(AsVec2::as_copy_vec2)
            .collect()
    }

    fn get_edges(&self) -> Vec<crate::graph::Edge> {
        self.graph.edges.iter().map(AsEdge::as_edge).collect()
    }
}
