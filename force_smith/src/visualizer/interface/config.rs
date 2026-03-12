use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::visualizer::simulation::resource::{LayoutParameterResource, LayoutResource};

pub struct ConfigUI;
impl Plugin for ConfigUI {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            (sync_parameters_from_layout, config_ui).chain(),
        );
    }
}

/// Sync parameter resource with actual layout parameters
fn sync_parameters_from_layout(
    layout: Option<NonSend<LayoutResource>>,
    mut params: ResMut<LayoutParameterResource>,
) {
    let Some(layout) = layout else {
        return;
    };

    let current_params = layout.get_parameters();

    // Update the parameter resource if values changed in the layout
    for (resource_param, layout_param) in params.iter_mut().zip(current_params.iter()) {
        if resource_param.is_same_parameter(layout_param)
            && !resource_param.is_same_parameter_value(layout_param)
        {
            resource_param.overwrite_value(layout_param);
        }
    }
}

fn config_ui(mut contexts: EguiContexts, mut params: ResMut<LayoutParameterResource>) {
    let Ok(context) = contexts.ctx_mut() else {
        return;
    };

    egui::Window::new("Parameter Configuration").show(context, |ui| {
        egui::Grid::new("parameter_grid").show(ui, |ui| {
            for param in params.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(&param.name);
                    param.add_ui_element(ui);
                });
                ui.end_row();
            }
        });
    });
}
