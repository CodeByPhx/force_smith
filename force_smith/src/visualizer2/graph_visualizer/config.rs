use bevy::prelude::*;

use crate::visualizer2::global_assets::GlobalColor;

#[derive(Resource)]
pub struct GraphVisualizerPluginConfig {
    pub node_radius: f32,
    pub node_color: GlobalColor,
    pub edge_width: f32,
    pub edge_color: GlobalColor,
}
