use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::visualizer::{
    global_schedule::{DebugExecutionState, NormalExecutionState, VisualizerMode},
    simulation::config::SimulationConfig,
};

pub struct VisualizerControlUI;
impl Plugin for VisualizerControlUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_smooth_movement)
            .add_systems(Update, reset_execution_states_on_mode_change)
            .add_systems(EguiPrimaryContextPass, visualizer_control_ui);
    }
}

/// Resource controlling whether nodes move smoothly or snap to positions
#[derive(Resource)]
pub struct SmoothMovementSetting {
    pub enabled: bool,
}

impl Default for SmoothMovementSetting {
    fn default() -> Self {
        Self { enabled: true }
    }
}

fn initialize_smooth_movement(config: Res<SimulationConfig>, mut commands: Commands) {
    commands.insert_resource(SmoothMovementSetting {
        enabled: config.smooth_movement_enabled,
    });
}

/// Reset both execution states to Stopped when the visualizer mode changes
fn reset_execution_states_on_mode_change(
    mode: Res<State<VisualizerMode>>,
    mut next_normal: ResMut<NextState<NormalExecutionState>>,
    mut next_debug: ResMut<NextState<DebugExecutionState>>,
) {
    if mode.is_changed() {
        // Reset both states to Stopped when mode changes
        next_normal.set(NormalExecutionState::Stopped);
        next_debug.set(DebugExecutionState::Stopped);
        info!("Mode changed - resetting execution states to Stopped");
    }
}

/// Bundle of state resources for the visualizer control UI
#[derive(SystemParam)]
struct VisualizerStates<'w> {
    mode: Res<'w, State<VisualizerMode>>,
    normal_state: Res<'w, State<NormalExecutionState>>,
    debug_state: Res<'w, State<DebugExecutionState>>,
    next_mode: ResMut<'w, NextState<VisualizerMode>>,
    next_normal: ResMut<'w, NextState<NormalExecutionState>>,
    next_debug: ResMut<'w, NextState<DebugExecutionState>>,
}

fn visualizer_control_ui(
    mut contexts: EguiContexts,
    mut states: VisualizerStates,
    mut smooth_movement: ResMut<SmoothMovementSetting>,
) {
    let Ok(context) = contexts.ctx_mut() else {
        return;
    };

    egui::Window::new("Layout Controls").show(context, |ui| {
        ui.vertical(|ui| {
            ui.heading("Mode Selection");
            ui.horizontal(|ui| {
                if ui
                    .radio(matches!(states.mode.get(), VisualizerMode::Normal), "Normal Mode")
                    .clicked()
                {
                    states.next_mode.set(VisualizerMode::Normal);
                }
                if ui
                    .radio(matches!(states.mode.get(), VisualizerMode::Debug), "Debug Mode")
                    .clicked()
                {
                    states.next_mode.set(VisualizerMode::Debug);
                }
            });

            ui.separator();

            // Smooth Movement Toggle
            ui.checkbox(&mut smooth_movement.enabled, "Smooth Movement")
                .on_hover_text("When enabled, nodes smoothly interpolate to destinations.\nWhen disabled, nodes snap instantly.");

            ui.separator();

            match states.mode.get() {
                VisualizerMode::Normal => {
                    ui.heading("Normal Mode");
                    ui.label(format!("State: {:?}", states.normal_state.get()));
                    match states.normal_state.get() {
                        NormalExecutionState::Running => {
                            if ui.button("⏹ Stop").clicked() {
                                info!("Stopping normal mode");
                                states.next_normal.set(NormalExecutionState::PlacingDestinations);
                            }
                        }
                        NormalExecutionState::Stopped | NormalExecutionState::PlacingDestinations => {
                            if ui.button("▶ Run").clicked() {
                                info!("Starting normal mode");
                                states.next_normal.set(NormalExecutionState::Running);
                            }
                        }
                    }
                }
                VisualizerMode::Debug => {
                    ui.heading("Debug Mode");
                    ui.label(format!("State: {:?}", states.debug_state.get()));
                    match states.debug_state.get() {
                        DebugExecutionState::Stopped => {
                            ui.horizontal(|ui| {
                                ui.label("Compute Forces");
                                if ui.button("⏭ Step").clicked() {
                                    states.next_debug.set(DebugExecutionState::Computing);
                                }
                            });
                        }
                        DebugExecutionState::StoppedBeforePositionUpdate => {
                            ui.horizontal(|ui| {
                                ui.label("Update Positions");
                                if ui.button("⏭ Step").clicked() {
                                    states.next_debug.set(DebugExecutionState::RemovingForces);
                                }
                            });
                        }
                        _ => {
                            ui.label("Processing...");
                        }
                    }
                }
            }
        });
    });
}
