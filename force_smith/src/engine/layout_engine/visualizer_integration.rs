use bevy_math::Vec2;

use crate::{
    engine::{AsEdge, AsVec2, layout_engine::LayoutEngine, types::Displacements},
    prelude::DebugLayoutAlgorithm,
    visualizer2::layout_trait::Parameterized,
};

impl<Vertex, Edge, Context> DebugLayoutAlgorithm for LayoutEngine<Vertex, Edge, Context>
where
    Vertex: AsVec2,
    Context: Default,
    Edge: AsEdge,
{
    fn dbg_iterate(&mut self) -> Vec<Vec<Vec2>> {
        let mut forces: Vec<Vec<Vec2>> = Vec::with_capacity(self.forces.len());
        self.displacements = vec![Vec2::ZERO; self.graph.vertices.len()].into();
        for step in &self.forces {
            let mut debug_displacements =
                Displacements::from(vec![Vec2::ZERO; self.graph.vertices.len()]);
            step.apply(
                &self.graph.vertices,
                &self.graph.edges,
                &self.context,
                &mut debug_displacements,
            );
            for (idx, debug_displacement) in debug_displacements.iter().enumerate() {
                self.displacements[idx] += *debug_displacement;
            }
            forces.push(debug_displacements.0);
        }
        (self.position_update_fn)(
            &self.displacements,
            &mut self.graph.vertices,
            &mut self.context,
        );
        forces
    }
}

impl<Vertex, Edge, Context> Parameterized for LayoutEngine<Vertex, Edge, Context>
where
    Context: Parameterized,
{
    fn get_parameters(&self) -> Vec<crate::visualizer2::layout_trait::Parameter> {
        self.context.get_parameters()
    }

    fn update_parameters(&mut self, parameters: &[crate::visualizer2::layout_trait::Parameter]) {
        self.context.update_parameters(parameters);
    }
}
