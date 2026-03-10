use bevy::prelude::*;

use crate::{prelude::VisualizerConfiguration, visualizer2::global_assets::GlobalColor};

#[derive(Resource, Clone, Copy)]
pub struct GraphVisualizerPluginConfig {
    pub node_radius: f32,
    pub node_color: GlobalColor,
    pub edge_width: f32,
    pub edge_color: GlobalColor,
}
impl Default for GraphVisualizerPluginConfig {
    fn default() -> Self {
        Self {
            node_radius: 10.0,
            node_color: GlobalColor::Red,
            edge_width: 5.0,
            edge_color: GlobalColor::Green,
        }
    }
}
impl From<&VisualizerConfiguration> for GraphVisualizerPluginConfig {
    fn from(value: &VisualizerConfiguration) -> Self {
        value.graph_visualization
    }
}
