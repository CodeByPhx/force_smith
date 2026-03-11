use crate::visualizer::{
    camera::CameraConfig,
    rendering::config::RenderingConfig,
    simulation::{
        config::SimulationConfig,
        resource::{LayoutParameterResource, LayoutResource},
    },
    layout_trait::ParameterizedDebugLayoutAlgorithm,
    visualizer_configuration::VisualizerConfiguration,
};
use bevy::prelude::*;

pub mod global_assets;
pub mod global_schedule;
mod rendering;
mod simulation;
pub mod layout_trait;
mod interface;
pub mod visualizer_configuration;
mod camera;

pub fn visualize_dbg(
    layout: Box<dyn ParameterizedDebugLayoutAlgorithm>,
    config: VisualizerConfiguration,
) {
    let layout_res = LayoutResource::from(layout);
    let layout_parameter_res = LayoutParameterResource::from(layout_res.get_parameters());

    App::new()
        .insert_non_send_resource(layout_res)
        .insert_resource(layout_parameter_res)
        .insert_resource(SimulationConfig::from(&config))
        .insert_resource(RenderingConfig::from(&config))
        .insert_resource(CameraConfig::from(&config))
        .add_plugins((
            DefaultPlugins,
            // Order matters! Schedule must be first to define the system sets and states
            global_schedule::VisualizerSchedulePlugin,
            // Global assets must be initialized early (in Startup)
            global_assets::GlobalAssetPlugin,
            // Camera plugin sets up camera controls and scene
            camera::CameraPlugin,
            // Simulation plugin manages the layout computation and execution modes
            simulation::SimulationPlugin,
            // Rendering plugin handles visual representation of graphs
            rendering::RenderingPlugin,
            // Interface plugin for controls and UI
            interface::InterfacePlugin,
        ))
        .run();
}
