use bevy_egui::egui::Ui;

pub fn up_down_arrow_buttons_usize(value: &mut usize, ui: &mut Ui, step_size: usize) {
    ui.horizontal(|ui| {
        if *value >= step_size && ui.button("-").clicked() {
            *value -= step_size;
        }
        ui.label(value.to_string());
        if *value <= usize::MAX - step_size && ui.button("+").clicked() {
            *value += step_size;
        }
    });
}
