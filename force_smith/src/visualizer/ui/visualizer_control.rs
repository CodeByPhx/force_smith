use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::visualizer::layout::LayoutMode;

pub struct VisualizerControlUI;
impl Plugin for VisualizerControlUI {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, visualizer_control_ui);
    }
}

pub struct VisualizerControlUiContext {
    mode: ControlMode,
}

pub enum ControlMode {
    Normal,
    Debug,
}

fn visualizer_control_ui(mut contexts: EguiContexts, mut layout_mode: ResMut<LayoutMode>) {
    let Ok(context) = contexts.ctx_mut() else {
        return;
    };
    egui::Window::new("Layout Controls").show(context, |ui| {
        // Selection Bar
        ui.vertical(|ui| {
            ui.heading("Mode Selection");
            ui.horizontal(|ui| {
                if ui
                    .radio(layout_mode.is_normal_mode(), "Normal Mode")
                    .clicked()
                {
                    info!("Normal Mode pressed");
                    // send cleanup debug mode message
                    *layout_mode = LayoutMode::Stop;
                };
                if ui
                    .radio(layout_mode.is_debug_mode(), "Debug Mode")
                    .clicked()
                {
                    // send cleanup normal mode message
                    *layout_mode = LayoutMode::DebugStop;
                }
            });
            if layout_mode.is_normal_mode() {
                ui.heading("Normal Mode");
                if ui.radio(layout_mode.is_run(), "▶").clicked() {
                    *layout_mode = LayoutMode::Run;
                }
                if ui.radio(layout_mode.is_stop(), "⏸").clicked() {
                    *layout_mode = LayoutMode::Stop;
                }
            } else if layout_mode.is_debug_mode() {
                ui.heading("Debug Mode");
                if layout_mode.is_debug_stop() {
                    ui.horizontal(|ui| {
                        ui.label("Compute Forces");
                        if ui.button("⏭").clicked() {
                            *layout_mode = LayoutMode::DebugComputeForces;
                        }
                    });
                }
                if layout_mode.is_debug_stop_before_update() {
                    ui.horizontal(|ui| {
                        ui.label("Update Graph");
                        if ui.button("⏭").clicked() {
                            *layout_mode = LayoutMode::DebugUpdateGraph;
                        }
                    });
                }
            }
        });
    });
}
