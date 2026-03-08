use crate::{
    layout::{
        Layout,
        types::{Displacements, Position, SomeContext},
    },
    prelude::Parameter,
    visualizer::layout_trait::{DebugLayoutAlgorithm, Parameterized},
};
use bevy::log::info;
use bevy_math::Vec2;

impl<Vertex: Position, Edge, Context> Parameterized
    for Layout<Vertex, Edge, Context, SomeContext<Context>>
where
    Context: Parameterized,
{
    fn get_parameters(&self) -> Vec<(String, Parameter)> {
        self.context.get_parameters()
    }

    fn update_parameters(&mut self, parameters: &[(String, Parameter)]) {
        self.context.update_parameters(parameters);
    }
}

impl<Vertex: Position, Edge, Context> DebugLayoutAlgorithm
    for Layout<Vertex, Edge, Context, SomeContext<Context>>
{
    fn iterate_debug(&mut self) -> Vec<Vec<Vec2>> {
        let mut forces: Vec<Vec<Vec2>> = Vec::with_capacity(self.forces.len());
        self.displacements = vec![Vec2::ZERO; self.graph.vertices.len()].into();
        for step in &self.forces.0 {
            let mut debug_displacements =
                Displacements::from(vec![Vec2::ZERO; self.graph.vertices.len()]);
            step.apply(
                &self.graph.vertices,
                &self.graph.edges,
                &self.context,
                &mut debug_displacements,
            );
            for (idx, debug_displacement) in debug_displacements.0.iter().enumerate() {
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
