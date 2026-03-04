use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::visualizer::layout::{DebugState, LayoutMode, LayoutState, NormalState};

pub struct VisualizerControlUI;
impl Plugin for VisualizerControlUI {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, visualizer_control_ui);
    }
}

fn visualizer_control_ui(mut contexts: EguiContexts, mut mode: ResMut<LayoutMode>) {
    let Ok(context) = contexts.ctx_mut() else {
        return;
    };
    egui::Window::new("Layout Controls").show(context, |ui| {
        ui.vertical(|ui| {
            ui.heading("Mode Selection");
            ui.horizontal(|ui| {
                if ui
                    .radio(matches!(mode.state, LayoutState::Normal(_)), "Normal Mode")
                    .clicked()
                {
                    info!("Normal Mode pressed");
                    mode.set_mode_changed();
                    mode.state = LayoutState::Normal(NormalState::Stop);
                };
                if ui
                    .radio(matches!(mode.state, LayoutState::Debug(_)), "Debug Mode")
                    .clicked()
                {
                    mode.set_mode_changed();
                    mode.state = LayoutState::Debug(DebugState::Stop);
                }
            });
            match &mut mode.state {
                LayoutState::Normal(normal_state) => {
                    ui.heading("Normal Mode");
                    if matches!(normal_state, NormalState::Run) && ui.button("⏹").clicked() {
                        *normal_state = NormalState::PlaceDestinations;
                    }
                    if matches!(normal_state, NormalState::Stop) && ui.button("▶").clicked() {
                        *normal_state = NormalState::Run;
                    }
                }
                LayoutState::Debug(debug_state) => {
                    ui.heading("Debug Mode");
                    if matches!(debug_state, DebugState::Stop) {
                        ui.horizontal(|ui| {
                            ui.label("Compute Forces");
                            if ui.button("⏭").clicked() {
                                *debug_state = DebugState::Compute;
                            }
                        });
                    }
                    if matches!(debug_state, DebugState::StopBeforeUpdate) {
                        ui.horizontal(|ui| {
                            ui.label("Update Positions");
                            if ui.button("⏭").clicked() {
                                *debug_state = DebugState::RemoveForces;
                            }
                        });
                    }
                }
            }
        });
    });
}
