use bevy::prelude::*;

use crate::prelude::VisualizerConfiguration;
use crate::visualizer::visualizer_configuration::VisualizerMode;

#[derive(Resource, Clone, Copy)]
pub struct SimulationConfig {
    pub initial_mode: VisualizerMode,
    pub smooth_movement_enabled: bool,
}

impl From<VisualizerConfiguration> for SimulationConfig {
    fn from(config: VisualizerConfiguration) -> Self {
        Self {
            initial_mode: config.initial_mode,
            smooth_movement_enabled: config.smooth_movement_enabled,
        }
    }
}

impl From<&VisualizerConfiguration> for SimulationConfig {
    fn from(config: &VisualizerConfiguration) -> Self {
        (*config).into()
    }
}
