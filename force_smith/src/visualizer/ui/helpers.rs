use bevy_egui::egui::Ui;

pub fn up_down_arrow_buttons_usize(value: &mut usize, ui: &mut Ui, step_size: usize) {
    ui.horizontal(|ui| {
        if ui.button("-").clicked() {
            if *value >= step_size {
                *value -= step_size;
            }
        }
        if ui.button("+").clicked() {
            if *value <= usize::MAX - step_size {
                *value += step_size;
            }
        }
    });
}
