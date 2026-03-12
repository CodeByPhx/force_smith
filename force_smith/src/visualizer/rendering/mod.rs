use bevy::prelude::*;

use crate::{
    graph::Edge,
    visualizer::{
        global_assets::{GlobalColorAsset, GlobalShapeAssets},
        global_schedule::VisualizerStates,
        interface::visualizer_control::SmoothMovementSetting,
        rendering::{
            bundles::{
                Destination, EdgeBundle, EdgeIndices, EdgeMarker, Index, NodeBundle, NodeMarker,
                calculate_unit_rectangle_transform,
            },
            config::RenderingConfig,
        },
        simulation::resource::at_least_one_message,
    },
};

mod arrows;
pub mod bundles;
pub mod config;
mod mode_cleanup;

pub struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((arrows::ArrowsPlugin, mode_cleanup::ModeCleanupPlugin))
            .add_message::<SetInitialGraph>()
            .add_systems(
                Update,
                (
                    set_initial_graph
                        .run_if(at_least_one_message::<SetInitialGraph>)
                        .in_set(VisualizerStates::BeforeIteration),
                    (redraw_edges, move_nodes)
                        .chain()
                        .in_set(VisualizerStates::AfterIteration),
                ),
            );
    }
}

type ExclusiveEdgeSelector = (With<EdgeMarker>, Without<NodeMarker>);
fn redraw_edges(
    nodes: Query<(&Index, &Transform), With<NodeMarker>>,
    mut edges: Query<(&EdgeIndices, &mut Transform), ExclusiveEdgeSelector>,
    config: Res<RenderingConfig>,
) {
    let mut node_positions: Vec<(usize, Vec3)> = nodes
        .iter()
        .map(|(index, transform)| (**index, transform.translation))
        .collect();
    node_positions.sort_by_key(|v| v.0);

    for (&EdgeIndices { from, to }, mut transform) in edges.iter_mut() {
        *transform = calculate_unit_rectangle_transform(
            node_positions[from].1,
            node_positions[to].1,
            config.edge_width,
        );
    }
}

#[derive(Message)]
pub struct SetInitialGraph {
    pub vertices: Vec<Vec2>,
    pub edges: Vec<Edge>,
}

type GraphEntitySelector = Or<(With<NodeMarker>, With<EdgeMarker>)>;
fn set_initial_graph(
    mut set_initial_graph: MessageReader<SetInitialGraph>,
    mut commands: Commands,
    old_graph_entities: Query<Entity, GraphEntitySelector>,
    config: Res<RenderingConfig>,
    global_meshes: Res<GlobalShapeAssets>,
    global_colors: Res<GlobalColorAsset>,
) {
    let Some(SetInitialGraph { vertices, edges }) = set_initial_graph.read().last() else {
        return;
    };
    old_graph_entities
        .iter()
        .for_each(|e| commands.entity(e).despawn());

    let mut edge_bundles: Vec<EdgeBundle> = Vec::with_capacity(edges.len());
    for &Edge {
        from: from_idx,
        to: to_idx,
    } in edges
    {
        edge_bundles.push(EdgeBundle::new(
            from_idx,
            to_idx,
            vertices[from_idx].extend(0.0),
            vertices[to_idx].extend(0.0),
            config.edge_width,
            global_meshes.unit_rectangle.clone(),
            global_colors[&config.edge_color].clone(),
        ));
    }
    commands.spawn_batch(edge_bundles);

    let mut node_bundles: Vec<NodeBundle> = Vec::with_capacity(vertices.len());
    for (idx, position) in vertices.iter().enumerate() {
        node_bundles.push(NodeBundle::new(
            idx,
            position.extend(0.0),
            config.node_radius,
            global_meshes.unit_circle.clone(),
            global_colors[&config.node_color].clone(),
        ));
    }
    commands.spawn_batch(node_bundles);
}

/// Move nodes toward their destination positions (smoothly or instantly based on setting)
fn move_nodes(
    mut nodes: Query<(&mut Transform, &Destination, Entity), With<NodeMarker>>,
    time: Res<Time>,
    config: Res<RenderingConfig>,
    smooth_movement: Res<SmoothMovementSetting>,
    mut commands: Commands,
) {
    for (mut transform, destination, entity) in nodes.iter_mut() {
        let current_pos = transform.translation;
        let target_pos = destination.extend(0.0);

        if smooth_movement.enabled {
            let direction = target_pos - current_pos;
            let distance = direction.length();

            let movement_amount = config.node_movement_speed * time.delta_secs();

            if movement_amount >= distance {
                // Close enough - snap to destination
                transform.translation = target_pos;
                commands.entity(entity).remove::<Destination>();
            } else {
                // Move toward destination
                let norm_dir = direction / distance;
                transform.translation += norm_dir * movement_amount;
            }
        } else {
            // Instant snap
            transform.translation = target_pos;
            commands.entity(entity).remove::<Destination>();
        }
    }
}
