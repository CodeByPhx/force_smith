use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use config::ConfigUI;
use graph_source::GraphSourceUI;
use visualizer_control::VisualizerControlUI;

mod config;
mod graph_source;
mod helpers;
mod visualizer_control;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(ConfigUI)
            .add_plugins(GraphSourceUI);
        // .add_plugins(VisualizerControlUI);
    }
}
