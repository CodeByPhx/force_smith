use crate::{
    layout::{
        Layout,
        types::{Displacements, Position, SomeContext},
    },
    visualizer::layout_trait::{DebugLayoutAlgorithm, Parameterized},
};
use bevy_math::Vec2;

impl<Vertex: Position, Edge, Context> Parameterized
    for Layout<Vertex, Edge, Context, SomeContext<Context>>
where
    Context: Parameterized,
{
    fn get_parameters(
        &self,
    ) -> std::collections::HashMap<String, crate::visualizer::layout_trait::Parameter> {
        self.context.get_parameters()
    }

    fn update_parameters(
        &mut self,
        parameters: &std::collections::HashMap<String, crate::visualizer::layout_trait::Parameter>,
    ) {
        self.context.update_parameters(parameters);
    }
}

impl<Vertex: Position, Edge, Context> DebugLayoutAlgorithm
    for Layout<Vertex, Edge, Context, SomeContext<Context>>
{
    fn iterate_debug(&mut self) -> Vec<Vec<Vec2>> {
        let mut forces: Vec<Vec<Vec2>> = Vec::with_capacity(self.forces.len());
        for step in &self.forces.0 {
            let mut debug_displacements =
                Displacements::from(Vec::with_capacity(self.graph.vertices.len()));
            step.apply(
                &self.graph.vertices,
                &self.graph.edges,
                &self.context,
                &mut debug_displacements,
            );
            for debug_displacement in &debug_displacements.0 {
                self.displacements.push(*debug_displacement);
            }
            forces.push(debug_displacements.0);
        }
        forces
    }
}
