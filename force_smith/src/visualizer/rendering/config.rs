use bevy::prelude::*;

use crate::{prelude::VisualizerConfiguration, visualizer::global_assets::GlobalColor};

#[derive(Resource, Clone, Copy)]
pub struct RenderingConfig {
    pub background_color: GlobalColor,
    pub node_radius: f32,
    pub node_color: GlobalColor,
    pub node_movement_speed: f32,
    pub edge_width: f32,
    pub edge_color: GlobalColor,
}

impl Default for RenderingConfig {
    fn default() -> Self {
        VisualizerConfiguration::default().into()
    }
}

impl From<VisualizerConfiguration> for RenderingConfig {
    fn from(config: VisualizerConfiguration) -> Self {
        Self {
            background_color: config.background_color,
            node_radius: config.node_radius,
            node_color: config.node_color,
            node_movement_speed: config.node_movement_speed,
            edge_width: config.edge_width,
            edge_color: config.edge_color,
        }
    }
}

impl From<&VisualizerConfiguration> for RenderingConfig {
    fn from(config: &VisualizerConfiguration) -> Self {
        (*config).into()
    }
}
