use crate::visualizer::global_assets::GlobalColor;

/// Comprehensive configuration for the Force Smith visualizer.
/// This includes all visual settings, animation controls, and initial layout mode.
#[derive(Clone, Copy)]
pub struct VisualizerConfiguration {
    // === Scene ===
    /// Background color of the visualization scene
    pub background_color: GlobalColor,

    // === Camera ===
    /// Speed of camera panning with mouse/touchpad (pixels per pixel of mouse movement)
    pub camera_pan_speed: f32,
    /// Speed of camera zoom (multiplier per scroll unit)
    pub camera_zoom_speed: f32,
    /// Speed of keyboard camera movement (pixels per second)
    pub camera_keyboard_speed: f32,

    // === Graph Visualization ===
    /// Radius of node circles in pixels
    pub node_radius: f32,
    /// Color of nodes
    pub node_color: GlobalColor,
    /// Speed at which nodes move toward their destination (pixels per second)
    pub node_movement_speed: f32,
    /// Width of edges in pixels
    pub edge_width: f32,
    /// Color of edges
    pub edge_color: GlobalColor,
    /// Color of force arrows
    pub force_arrow_color: GlobalColor,
    /// Color of final force arrows
    pub final_force_arrow_color: GlobalColor,
    /// Width of arrows shafts in pixels
    pub arrow_shaft_width: f32,
    /// Width of arrows tips in pixels
    pub arrow_tip_width: f32,
    /// Ratio of arrow shaft length to its tip
    pub arrow_shaft_tip_ratio: f32,

    // === Initial Mode ===
    /// Which mode to start in (Normal or Debug)
    pub initial_mode: VisualizerMode,
    /// Whether smooth movement is enabled by default
    pub smooth_movement_enabled: bool,
}

impl Default for VisualizerConfiguration {
    fn default() -> Self {
        Self {
            // Scene defaults - soft grey background
            background_color: GlobalColor::DarkGrey,

            // Camera defaults
            camera_pan_speed: 1.0,
            camera_zoom_speed: 0.1,
            camera_keyboard_speed: 500.0,

            // Graph visualization defaults - harmonious color scheme
            node_radius: 10.0,
            node_color: GlobalColor::Cyan,
            node_movement_speed: 100.0,
            edge_width: 3.0,
            edge_color: GlobalColor::LightGrey,

            // Initial mode defaults
            initial_mode: VisualizerMode::Normal,
            smooth_movement_enabled: true,
            force_arrow_color: GlobalColor::Red,
            final_force_arrow_color: GlobalColor::Green,
            arrow_shaft_width: 2.0,
            arrow_tip_width: 2.0 * 10.0,
            arrow_shaft_tip_ratio: 0.9,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisualizerMode {
    Debug,
    Normal,
}
