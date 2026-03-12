use bevy::prelude::*;

use crate::visualizer::{
    global_schedule::VisualizerMode,
    rendering::bundles::{ArrowMarker, Destination, NodeMarker},
};

pub struct ModeCleanupPlugin;
impl Plugin for ModeCleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(VisualizerMode::Normal), cleanup_debug_mode);
    }
}

/// Clean up debug artifacts when entering Normal mode
fn cleanup_debug_mode(
    arrows: Query<Entity, With<ArrowMarker>>,
    mut nodes: Query<(&mut Transform, &Destination, Entity), With<NodeMarker>>,
    mut commands: Commands,
) {
    // Despawn all arrow entities
    for entity in arrows.iter() {
        commands.entity(entity).despawn();
    }

    // Instantly place all nodes at their destinations and remove the Destination component
    for (mut transform, destination, entity) in nodes.iter_mut() {
        transform.translation = destination.extend(0.0);
        commands.entity(entity).remove::<Destination>();
    }
}
