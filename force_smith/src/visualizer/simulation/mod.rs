use bevy::prelude::*;

pub mod config;
pub mod debug;
pub mod normal;
pub mod resource;

pub struct SimulationPlugin;
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            debug::DebugModePlugin,
            normal::NormalModePlugin,
            resource::ResourcePlugin,
        ));
    }
}
