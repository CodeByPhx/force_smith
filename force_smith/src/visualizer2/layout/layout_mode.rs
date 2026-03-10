use bevy::prelude::*;

use crate::visualizer2::layout::{
    config::{LayoutModeSetting, LayoutPluginConfig},
    debug_mode::DebugState,
    normal_mode::NormalState,
};

pub struct LayoutModePlugin;
impl Plugin for LayoutModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_initial_layout_mode);
    }
}

#[derive(Resource)]
pub struct LayoutMode {
    pub state: LayoutState,
    pub mode_changed: bool,
}

#[derive(PartialEq)]
pub enum LayoutState {
    Normal(NormalState),
    Debug(DebugState),
}
impl From<NormalState> for LayoutState {
    fn from(value: NormalState) -> Self {
        Self::Normal(value)
    }
}
impl From<DebugState> for LayoutState {
    fn from(value: DebugState) -> Self {
        Self::Debug(value)
    }
}

pub fn in_layout_state(expected: LayoutState) -> impl Fn(Res<LayoutMode>) -> bool {
    move |res: Res<LayoutMode>| res.state == expected
}

pub fn set_initial_layout_mode(config: Res<LayoutPluginConfig>, mut commands: Commands) {
    match config.initial_layout_mode {
        LayoutModeSetting::Debug => commands.insert_resource(LayoutMode {
            state: DebugState::Stop.into(),
            mode_changed: false,
        }),
        LayoutModeSetting::Normal => commands.insert_resource(LayoutMode {
            state: NormalState::Stop.into(),
            mode_changed: false,
        }),
    }
}
