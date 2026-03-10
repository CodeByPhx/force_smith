use bevy::prelude::*;

use crate::prelude::VisualizerConfiguration;

#[derive(Resource, Clone, Copy)]
pub struct LayoutPluginConfig {
    pub initial_layout_mode: LayoutModeSetting,
}
impl Default for LayoutPluginConfig {
    fn default() -> Self {
        Self {
            initial_layout_mode: LayoutModeSetting::Normal,
        }
    }
}

#[derive(Clone, Copy)]
pub enum LayoutModeSetting {
    Debug,
    Normal,
}

impl From<&VisualizerConfiguration> for LayoutPluginConfig {
    fn from(value: &VisualizerConfiguration) -> Self {
        value.layout
    }
}
