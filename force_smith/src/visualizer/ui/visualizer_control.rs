use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::visualizer::layout::LayoutMode;

pub struct VisualizerControlUI;
impl Plugin for VisualizerControlUI {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, visualizer_control_ui);
    }
}

fn visualizer_control_ui(mut contexts: EguiContexts, mut layout_mode: ResMut<LayoutMode>) {
    let Ok(context) = contexts.ctx_mut() else {
        return;
    };
    egui::Window::new("Layout Controls").show(context, |ui| {
        if ui
            .radio(matches!(*layout_mode, LayoutMode::Stop), "Stop")
            .clicked()
        {
            *layout_mode = LayoutMode::Stop;
        }
        if ui
            .radio(matches!(*layout_mode, LayoutMode::Run), "Run")
            .clicked()
        {
            *layout_mode = LayoutMode::Run;
        }
        if ui
            .radio(matches!(*layout_mode, LayoutMode::Debug), "Debug")
            .clicked()
        {
            *layout_mode = LayoutMode::Debug;
        }
    });
}
