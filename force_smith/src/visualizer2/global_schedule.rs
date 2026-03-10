use bevy::prelude::*;

pub struct VisualizerSchedulePlugin;
impl Plugin for VisualizerSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                VisualizerStates::BeforeIteration,
                VisualizerStates::Iteration,
                VisualizerStates::AfterIteration,
            ),
        );
    }
}

#[derive(SystemSet, Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VisualizerStates {
    BeforeIteration,
    Iteration,
    AfterIteration,
}
