use bevy::prelude::*;

use crate::visualizer::{
    global_schedule::{DebugExecutionState, VisualizerMode, VisualizerStates},
    rendering::bundles::{ArrowMarker, Destination, Index, NodeMarker},
    simulation::resource::LayoutResource,
};

pub struct DebugModePlugin;
impl Plugin for DebugModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugVisualizationData>()
            .add_systems(
                Update,
                (
                    // Computing state: snap nodes to destinations, then run layout iteration
                    (snap_nodes_to_destinations, dbg_iterate_layout)
                        .chain()
                        .run_if(in_state(VisualizerMode::Debug))
                        .run_if(in_state(DebugExecutionState::Computing))
                        .in_set(VisualizerStates::Iteration),
                    // RemovingForces state: clean up arrow entities, attach destinations
                    (debug_remove_forces, attach_destinations)
                        .chain()
                        .run_if(in_state(VisualizerMode::Debug))
                        .run_if(in_state(DebugExecutionState::RemovingForces))
                        .in_set(VisualizerStates::AfterIteration),
                ),
            );
    }
}

/// Resource to store intermediate debug visualization data
#[derive(Resource, Default)]
pub struct DebugVisualizationData {
    pub forces: Vec<Vec<Vec2>>,
    pub destinations: Vec<Vec2>,
}

/// Snap any nodes that are still moving to their destinations before computing new forces
fn snap_nodes_to_destinations(
    mut nodes: Query<(&mut Transform, &Destination, Entity), With<NodeMarker>>,
    mut commands: Commands,
) {
    for (mut transform, destination, entity) in nodes.iter_mut() {
        info!("Debug: Snapping node to destination before computing new forces");
        transform.translation = destination.extend(0.0);
        commands.entity(entity).remove::<Destination>();
    }
}

fn dbg_iterate_layout(
    mut layout: NonSendMut<LayoutResource>,
    mut debug_data: ResMut<DebugVisualizationData>,
    mut next_state: ResMut<NextState<DebugExecutionState>>,
) {
    let forces = layout.dbg_iterate();
    let destinations = layout.get_positions();

    // Store the computed data for visualization
    debug_data.forces = forces;
    debug_data.destinations = destinations;

    // Transition to ShowingForces state
    next_state.set(DebugExecutionState::ShowingForces);
}

fn attach_destinations(
    layout: NonSend<LayoutResource>,
    nodes: Query<(&Index, Entity), With<NodeMarker>>,
    mut commands: Commands,
) {
    let positions = layout.get_positions();
    info!("Debug: Attaching destinations to {} nodes for smooth movement", nodes.iter().count());
    for (&Index(idx), entity) in nodes.iter() {
        if let Some(&position) = positions.get(idx) {
            commands.entity(entity).insert(Destination(position));
        }
    }
}

/// Remove all arrow entities and transition to Stopped (nodes will move smoothly via move_nodes system)
fn debug_remove_forces(
    arrows: Query<Entity, With<ArrowMarker>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<DebugExecutionState>>,
) {
    info!("Debug: Removing forces, transitioning to Stopped");
    for entity in arrows.iter() {
        commands.entity(entity).despawn();
    }
    next_state.set(DebugExecutionState::Stopped);
}
