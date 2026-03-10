use bevy::prelude::*;

pub mod config;
mod debug_mode;
mod layout_mode;
mod normal_mode;
pub mod resource;

pub struct LayoutPlugin;
impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            debug_mode::DebugModePlugin,
            normal_mode::NormalModePlugin,
            resource::ResourcePlugin,
            layout_mode::LayoutModePlugin,
        ));
    }
}
