use crate::visualizer2::{
    graph_visualizer::config::GraphVisualizerPluginConfig, layout::config::LayoutPluginConfig,
};

#[derive(Default)]
pub struct VisualizerConfiguration {
    pub layout: LayoutPluginConfig,
    pub graph_visualization: GraphVisualizerPluginConfig,
}
