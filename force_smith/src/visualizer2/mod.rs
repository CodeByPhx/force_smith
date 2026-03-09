use crate::visualizer2::{
    graph_visualizer::GraphVisualizerPlugin, layout::LayoutPlugin,
    layout_trait::DebugLayoutAlgorithm, ui::UiPlugin,
    visualizer_configuration::VisualizerConfiguration, world::WorldPlugin,
};
use bevy::prelude::*;

mod graph_visualizer;
mod layout;
pub mod layout_trait;
mod ui;
pub mod visualizer_configuration;
mod world;

#[cfg(feature = "visualize_dbg")]
pub fn visualize_dbg(layout: Box<dyn DebugLayoutAlgorithm>, config: VisualizerConfiguration) {
    let layout = layout::LayoutResource::from(layout);

    App::new().add_plugins((
        DefaultPlugins,
        WorldPlugin,
        UiPlugin,
        LayoutPlugin,
        GraphVisualizerPlugin,
    ));
}
