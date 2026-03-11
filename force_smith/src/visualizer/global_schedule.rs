use bevy::prelude::*;

use crate::visualizer::{simulation::config::SimulationConfig, visualizer_configuration::VisualizerMode as ConfigVisualizerMode};

pub struct VisualizerSchedulePlugin;
impl Plugin for VisualizerSchedulePlugin {
    fn build(&self, app: &mut App) {
        // Configure system sets to run in strict sequence each frame
        // This ensures: parameters/graph updates → layout computation → visual updates
        app.configure_sets(
            Update,
            (
                VisualizerStates::BeforeIteration,
                VisualizerStates::Iteration,
                VisualizerStates::AfterIteration,
            )
                .chain(), // CRITICAL: Forces sequential execution
        );

        // Initialize Bevy States for mode management
        app.init_state::<VisualizerMode>()
            .init_state::<NormalExecutionState>()
            .init_state::<DebugExecutionState>();

        // Set initial mode based on configuration
        app.add_systems(Startup, set_initial_visualizer_mode);
    }
}

/// Set the initial visualizer mode based on configuration
fn set_initial_visualizer_mode(
    config: Res<SimulationConfig>,
    mut next_mode: ResMut<NextState<VisualizerMode>>,
) {
    match config.initial_mode {
        ConfigVisualizerMode::Debug => next_mode.set(VisualizerMode::Debug),
        ConfigVisualizerMode::Normal => next_mode.set(VisualizerMode::Normal),
    }
}

#[derive(SystemSet, Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VisualizerStates {
    /// First: Handle parameter changes and graph loading
    BeforeIteration,
    /// Second: Compute layout step (only when running)
    Iteration,
    /// Third: Update visual representation
    AfterIteration,
}

// ============================================================================
// Bevy State Management for Visualizer Modes
// ============================================================================

/// Top-level mode: Normal (automatic) vs Debug (step-through)
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualizerMode {
    #[default]
    Normal,
    Debug,
}

/// Execution state within Normal mode
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum NormalExecutionState {
    #[default]
    Stopped,
    Running,
    PlacingDestinations, // Transition state when stopping - instantly places nodes
}

/// Execution state within Debug mode (more granular control)
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DebugExecutionState {
    #[default]
    Stopped,
    Computing,
    ShowingForces,
    StoppedBeforePositionUpdate,
    RemovingForces,
}
