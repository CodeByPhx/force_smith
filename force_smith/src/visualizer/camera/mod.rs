use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_egui::EguiContexts;

use crate::visualizer::{
    rendering::config::RenderingConfig, visualizer_configuration::VisualizerConfiguration,
};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, set_background_color))
            .add_systems(Update, (camera_movement, camera_zoom, camera_pan));
    }
}

#[derive(Resource)]
pub struct CameraConfig {
    pan_speed: f32,
    zoom_speed: f32,
    keyboard_speed: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        VisualizerConfiguration::default().into()
    }
}

impl From<VisualizerConfiguration> for CameraConfig {
    fn from(config: VisualizerConfiguration) -> Self {
        Self {
            pan_speed: config.camera_pan_speed,
            zoom_speed: config.camera_zoom_speed,
            keyboard_speed: config.camera_keyboard_speed,
        }
    }
}

impl From<&VisualizerConfiguration> for CameraConfig {
    fn from(config: &VisualizerConfiguration) -> Self {
        (*config).into()
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn set_background_color(config: Res<RenderingConfig>, mut commands: Commands) {
    commands.insert_resource(ClearColor(config.background_color.to_color()));
}

/// Handle keyboard movement (vim-style: hjkl + arrows)
fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    camera_config: Res<CameraConfig>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        // Vim keybinds: h(left), j(down), k(up), l(right)
        if keyboard_input.pressed(KeyCode::KeyH) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyL) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyK) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyJ) {
            direction.y -= 1.0;
        }

        // Arrow keys as alternative
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            let zoom_factor = transform.scale.x;
            transform.translation +=
                direction * camera_config.keyboard_speed * time.delta_secs() * zoom_factor;
        }
    }
}

/// Handle zoom (mouse wheel and keyboard)
fn camera_zoom(
    mut scroll_events: MessageReader<MouseWheel>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    camera_config: Res<CameraConfig>,
    mut query: Query<&mut Transform, With<Camera2d>>,
    mut egui_contexts: EguiContexts,
) {
    for mut transform in query.iter_mut() {
        let mut zoom_delta = 0.0;

        // Mouse wheel zoom - only if not over UI
        let mouse_over_ui = if let Ok(ctx) = egui_contexts.ctx_mut() {
            ctx.is_pointer_over_area()
        } else {
            false
        };

        if !mouse_over_ui {
            for event in scroll_events.read() {
                let delta = match event.unit {
                    MouseScrollUnit::Line => event.y,
                    MouseScrollUnit::Pixel => event.y * 0.01,
                };
                zoom_delta -= delta * camera_config.zoom_speed;
            }
        } else {
            // Clear scroll events when over UI
            scroll_events.clear();
        }

        // Keyboard zoom: + (or =) to zoom in, - to zoom out
        if keyboard_input.pressed(KeyCode::Equal) || keyboard_input.pressed(KeyCode::NumpadAdd) {
            zoom_delta -= camera_config.zoom_speed * time.delta_secs() * 5.0;
        }
        if keyboard_input.pressed(KeyCode::Minus) || keyboard_input.pressed(KeyCode::NumpadSubtract)
        {
            zoom_delta += camera_config.zoom_speed * time.delta_secs() * 5.0;
        }

        if zoom_delta.abs() > 0.0 {
            let new_scale = (transform.scale.x * (1.0 + zoom_delta)).clamp(0.1, 10.0);
            transform.scale = Vec3::splat(new_scale);
        }
    }
}

/// Handle mouse panning (left-click drag)
fn camera_pan(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    camera_config: Res<CameraConfig>,
    mut query: Query<&mut Transform, With<Camera2d>>,
    mut egui_contexts: EguiContexts,
) {
    // If mouse is over any egui UI, clear motion
    if let Ok(ctx) = egui_contexts.ctx_mut()
        && ctx.is_pointer_over_area()
    {
        mouse_motion.clear();
        return;
    }

    // Pan with left mouse button only
    if mouse_button.pressed(MouseButton::Left) {
        for mut transform in query.iter_mut() {
            for motion in mouse_motion.read() {
                // Invert the delta for natural scrolling feel
                let zoom_factor = transform.scale.x;
                transform.translation.x -= motion.delta.x * camera_config.pan_speed * zoom_factor;
                transform.translation.y += motion.delta.y * camera_config.pan_speed * zoom_factor;
            }
        }
    } else {
        // Clear motion events if not panning
        mouse_motion.clear();
    }
}
