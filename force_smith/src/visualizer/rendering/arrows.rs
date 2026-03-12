use bevy::prelude::*;

use crate::{
    prelude::GlobalColor,
    visualizer::{
        global_assets::{GlobalColorAsset, GlobalShapeAssets},
        global_schedule::{DebugExecutionState, VisualizerMode, VisualizerStates},
        rendering::{
            bundles::{ArrowMarker, Index, NodeMarker},
            config::RenderingConfig,
        },
        simulation::debug::DebugVisualizationData,
    },
};

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            debug_show_forces
                .run_if(in_state(VisualizerMode::Debug))
                .run_if(in_state(DebugExecutionState::ShowingForces))
                .in_set(VisualizerStates::AfterIteration),
        );
    }
}

/// Show force vectors and destination arrows in debug mode
fn debug_show_forces(
    nodes: Query<(&Index, &Transform), With<NodeMarker>>,
    mut commands: Commands,
    global_meshes: Res<GlobalShapeAssets>,
    global_colors: Res<GlobalColorAsset>,
    config: Res<RenderingConfig>,
    debug_data: Res<DebugVisualizationData>,
    mut next_state: ResMut<NextState<DebugExecutionState>>,
) {
    for (&Index(idx), transform) in nodes.iter() {
        let origin = transform.translation;

        // Draw force vectors in red
        for force_layer in &debug_data.forces {
            let displacement = force_layer[idx].extend(0.0);
            let end = origin + displacement;
            draw_arrow2d(
                &mut commands,
                &global_meshes,
                &global_colors,
                origin,
                end,
                config.arrow_shaft_width,
                config.arrow_tip_width,
                config.force_arrow_color,
                config.arrow_shaft_tip_ratio,
            );
        }

        // Draw destination in green
        let destination = debug_data.destinations[idx].extend(0.0);
        draw_arrow2d(
            &mut commands,
            &global_meshes,
            &global_colors,
            origin,
            destination,
            config.arrow_shaft_width,
            config.arrow_tip_width,
            config.final_force_arrow_color,
            config.arrow_shaft_tip_ratio,
        );
    }

    // Transition to StoppedBeforePositionUpdate
    next_state.set(DebugExecutionState::StoppedBeforePositionUpdate);
}

#[allow(clippy::too_many_arguments)]
fn draw_arrow2d(
    commands: &mut Commands,
    global_meshes: &Res<GlobalShapeAssets>,
    global_colors: &Res<GlobalColorAsset>,
    start: Vec3,
    end: Vec3,
    shaft_thickness: f32,
    tip_thickness: f32,
    color: GlobalColor,
    shaft_tip_ratio: f32,
) {
    let Some(color_handle) = global_colors.get(&color) else {
        return;
    };

    let direction = end - start;
    let shaft_start = start;
    let shaft_end = start + direction * shaft_tip_ratio;
    let tip_start = shaft_end;
    let tip_end = end;

    draw_arrow_shaft2d(
        commands,
        &global_meshes.unit_rectangle,
        color_handle,
        shaft_start,
        shaft_end,
        shaft_thickness,
    );
    draw_arrow_tip2d(
        commands,
        &global_meshes.unit_triangle,
        color_handle,
        tip_start,
        tip_end,
        tip_thickness,
    );
}

fn draw_arrow_shaft2d(
    commands: &mut Commands,
    unit_rectangle_mesh: &Handle<Mesh>,
    color_material: &Handle<ColorMaterial>,
    start: Vec3,
    end: Vec3,
    thickness: f32,
) {
    let direction = end - start;
    let midpoint = start + direction / 2.0;
    let angle = direction.truncate().to_angle();

    commands.spawn((
        Mesh2d(unit_rectangle_mesh.clone()),
        MeshMaterial2d(color_material.clone()),
        ArrowMarker,
        Transform {
            translation: midpoint,
            rotation: Quat::from_rotation_z(angle),
            scale: Vec3::new(direction.length(), thickness, 1.0),
        },
    ));
}

fn draw_arrow_tip2d(
    commands: &mut Commands,
    unit_triangle_mesh: &Handle<Mesh>,
    color_material: &Handle<ColorMaterial>,
    start: Vec3,
    end: Vec3,
    thickness: f32,
) {
    let start2d = start.truncate();
    let end2d = end.truncate();
    let dir = end2d - start2d;
    let length = dir.length();

    if length == 0.0 {
        return;
    }

    let angle = dir.to_angle();

    commands.spawn((
        Mesh2d(unit_triangle_mesh.clone()),
        MeshMaterial2d(color_material.clone()),
        Transform {
            translation: Vec3::new(end.x, end.y, start.z),
            rotation: Quat::from_rotation_z(angle),
            scale: Vec3::new(length, thickness / 2.0, 1.0),
        },
        ArrowMarker,
    ));
}
