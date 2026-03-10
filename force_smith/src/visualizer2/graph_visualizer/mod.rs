use bevy::{
    platform::collections::HashMap, prelude::*, render::render_resource::encase::private::Length,
};

use crate::{
    graph::Edge,
    visualizer2::{
        global_assets::{GlobalColorAsset, GlobalShapeAssets},
        global_schedule::VisualizerStates,
        graph_visualizer::{
            bundles::{
                EdgeBundle, EdgeIndices, EdgeMarker, GraphEntitySelector, Index, NodeBundle,
                NodeMarker, calculate_unit_rectangle_transform,
            },
            config::GraphVisualizerPluginConfig,
        },
        layout::resource::at_least_one_message,
    },
};

mod bundles;
mod config;

pub struct GraphVisualizerPlugin;
impl Plugin for GraphVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SetInitialGraph>().add_systems(
            Update,
            (
                set_initial_graph
                    .run_if(at_least_one_message::<SetInitialGraph>)
                    .in_set(VisualizerStates::BeforeIteration),
                redraw_edges.in_set(VisualizerStates::AfterIteration),
            ),
        );
    }
}

fn redraw_edges(
    nodes: Query<(&Index, &Transform), With<NodeMarker>>,
    mut edges: Query<(&EdgeIndices, &mut Transform), With<EdgeMarker>>,
    config: Res<GraphVisualizerPluginConfig>,
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

fn set_initial_graph(
    mut set_initial_graph: MessageReader<SetInitialGraph>,
    mut commands: Commands,
    old_graph_entities: Query<Entity, GraphEntitySelector>,
    config: Res<GraphVisualizerPluginConfig>,
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
