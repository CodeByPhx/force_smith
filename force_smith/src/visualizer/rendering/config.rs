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
    pub force_arrow_color: GlobalColor,
    pub final_force_arrow_color: GlobalColor,
    pub arrow_shaft_width: f32,
    pub arrow_tip_width: f32,
    pub arrow_shaft_tip_ratio: f32,
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
            force_arrow_color: config.force_arrow_color,
            final_force_arrow_color: config.final_force_arrow_color,
            arrow_shaft_width: config.arrow_shaft_width,
            arrow_tip_width: config.arrow_tip_width,
            arrow_shaft_tip_ratio: config.arrow_shaft_tip_ratio,
        }
    }
}

impl From<&VisualizerConfiguration> for RenderingConfig {
    fn from(config: &VisualizerConfiguration) -> Self {
        (*config).into()
    }
}
