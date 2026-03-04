use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::visualizer::layout::{LayoutConfigResource, LayoutResource};

pub struct ConfigUI;
impl Plugin for ConfigUI {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            (config_ui, update_config.before(config_ui)),
        );
    }
}

fn update_config(mut config: ResMut<LayoutConfigResource>, layout: NonSend<LayoutResource>) {
    *config = layout.get_parameters().into();
}

fn config_ui(mut contexts: EguiContexts, mut config: ResMut<LayoutConfigResource>) {
    let Ok(context) = contexts.ctx_mut() else {
        return;
    };
    egui::Window::new("Parameter Configuration").show(context, |ui| {
        egui::Grid::new("parameter_grid").show(ui, |ui| {
            for (name, parameter) in config.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(name.clone());
                    parameter.add_ui_element(ui);
                });
                ui.end_row();
            }
        });
    });
}
