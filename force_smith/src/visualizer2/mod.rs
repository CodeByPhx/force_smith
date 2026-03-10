use crate::visualizer2::{
    graph_visualizer::config::GraphVisualizerPluginConfig,
    layout::{
        config::LayoutPluginConfig,
        resource::{LayoutParameterResource, LayoutResource},
    },
    layout_trait::ParameterizedDebugLayoutAlgorithm,
    visualizer_configuration::VisualizerConfiguration,
};
use bevy::prelude::*;

pub mod global_assets;
pub mod global_schedule;
mod graph_visualizer;
mod layout;
pub mod layout_trait;
mod ui;
pub mod visualizer_configuration;
mod world;

pub fn visualize_dbg(
    layout: Box<dyn ParameterizedDebugLayoutAlgorithm>,
    config: VisualizerConfiguration,
) {
    let layout_res = LayoutResource::from(layout);
    let layout_parameter_res = LayoutParameterResource::from(layout_res.get_parameters());

    App::new()
        .insert_non_send_resource(layout_res)
        .insert_resource(layout_parameter_res)
        .insert_resource(LayoutPluginConfig::from(&config))
        .insert_resource(GraphVisualizerPluginConfig::from(&config))
        .add_plugins((
            DefaultPlugins,
            world::WorldPlugin,
            // ui::UiPlugin,
            // layout::LayoutPlugin,
            // graph_visualizer::GraphVisualizerPlugin,
            // global_schedule::VisualizerSchedulePlugin,
        ))
        .run();
}
