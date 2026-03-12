use bevy::prelude::*;

use crate::visualizer::{
    global_schedule::{NormalExecutionState, VisualizerMode, VisualizerStates},
    rendering::bundles::{Destination, Index, NodeMarker},
    simulation::resource::LayoutResource,
};

pub struct NormalModePlugin;
impl Plugin for NormalModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // Run iteration when Running
                (iterate_layout, attach_destinations)
                    .chain()
                    .run_if(in_state(VisualizerMode::Normal))
                    .run_if(in_state(NormalExecutionState::Running))
                    .in_set(VisualizerStates::Iteration),
                // Place nodes instantly when stopping
                place_nodes
                    .run_if(in_state(VisualizerMode::Normal))
                    .run_if(in_state(NormalExecutionState::PlacingDestinations))
                    .in_set(VisualizerStates::AfterIteration),
            ),
        );
    }
}

fn iterate_layout(mut layout: NonSendMut<LayoutResource>) {
    info!("Normal mode: iterating layout");
    layout.iterate();
}

fn attach_destinations(
    layout: NonSend<LayoutResource>,
    nodes: Query<(&Index, Entity), With<NodeMarker>>,
    mut commands: Commands,
) {
    let positions = layout.get_positions();
    info!("Attaching destinations to {} nodes", nodes.iter().count());
    for (&Index(idx), entity) in nodes.iter() {
        if let Some(&position) = positions.get(idx) {
            commands.entity(entity).insert(Destination(position));
        }
    }
}

/// Instantly place all nodes at their destination and transition to Stopped
fn place_nodes(
    mut nodes: Query<(&mut Transform, &Destination, Entity), With<NodeMarker>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<NormalExecutionState>>,
) {
    for (mut transform, destination, entity) in nodes.iter_mut() {
        transform.translation = destination.extend(0.0);
        commands.entity(entity).remove::<Destination>();
    }
    next_state.set(NormalExecutionState::Stopped);
}
