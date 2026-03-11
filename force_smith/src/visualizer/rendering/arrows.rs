use bevy::{platform::collections::HashMap, prelude::*};

use crate::visualizer::{
    global_schedule::{DebugExecutionState, VisualizerMode, VisualizerStates},
    rendering::bundles::{ArrowMarker, Index, NodeMarker},
    simulation::debug::DebugVisualizationData,
};

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_arrow_assets).add_systems(
            Update,
            debug_show_forces
                .run_if(in_state(VisualizerMode::Debug))
                .run_if(in_state(DebugExecutionState::ShowingForces))
                .in_set(VisualizerStates::AfterIteration),
        );
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum ArrowColor {
    Red,
    Green,
}

impl ArrowColor {
    pub const ALL: [Self; 2] = [Self::Red, Self::Green];

    fn to_color(self) -> Color {
        match self {
            ArrowColor::Red => Color::srgb(1.0, 0.0, 0.0),
            ArrowColor::Green => Color::srgb(0.0, 1.0, 0.0),
        }
    }
}

#[derive(Resource)]
pub struct ArrowAssets {
    shaft_unit_mesh: Handle<Mesh>,
    tip_unit_mesh: Handle<Mesh>,
    materials: HashMap<ArrowColor, Handle<ColorMaterial>>,
}

fn setup_arrow_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(ArrowAssets {
        shaft_unit_mesh: meshes.add(Rectangle::new(1.0, 1.0)),
        tip_unit_mesh: meshes.add(Triangle2d::new(
            Vec2::new(1.0, 0.0),
            Vec2::new(-1.0, -1.0),
            Vec2::new(-1.0, 1.0),
        )),
        materials: ArrowColor::ALL
            .iter()
            .map(|c| (*c, materials.add(c.to_color())))
            .collect(),
    });
}

/// Show force vectors and destination arrows in debug mode
fn debug_show_forces(
    nodes: Query<(&Index, &Transform), With<NodeMarker>>,
    mut commands: Commands,
    arrow_assets: Res<ArrowAssets>,
    debug_data: Res<DebugVisualizationData>,
    mut next_state: ResMut<NextState<DebugExecutionState>>,
) {
    for (&Index(idx), transform) in nodes.iter() {
        let origin = transform.translation;
        let shaft_thickness = 2.0;
        let tip_thickness = shaft_thickness * 10.0;

        // Draw force vectors in red
        for force_layer in &debug_data.forces {
            let displacement = force_layer[idx].extend(0.0);
            let end = origin + displacement;
            draw_arrow2d(
                &mut commands,
                &arrow_assets,
                origin,
                end,
                shaft_thickness,
                tip_thickness,
                ArrowColor::Red,
                0.9,
            );
        }

        // Draw destination in green
        let destination = debug_data.destinations[idx].extend(0.0);
        draw_arrow2d(
            &mut commands,
            &arrow_assets,
            origin,
            destination,
            shaft_thickness,
            tip_thickness,
            ArrowColor::Green,
            0.9,
        );
    }

    // Transition to StoppedBeforePositionUpdate
    next_state.set(DebugExecutionState::StoppedBeforePositionUpdate);
}

#[allow(clippy::too_many_arguments)]
fn draw_arrow2d(
    commands: &mut Commands,
    arrow_assets: &ArrowAssets,
    start: Vec3,
    end: Vec3,
    shaft_thickness: f32,
    tip_thickness: f32,
    color: ArrowColor,
    shaft_tip_ratio: f32,
) {
    let direction = end - start;
    let shaft_start = start;
    let shaft_end = start + direction * shaft_tip_ratio;
    let tip_start = shaft_end;
    let tip_end = end;

    draw_arrow_shaft(
        commands,
        arrow_assets,
        shaft_start,
        shaft_end,
        shaft_thickness,
        color,
    );
    draw_arrow_tip(
        commands,
        arrow_assets,
        tip_start,
        tip_end,
        tip_thickness,
        color,
    );
}

fn draw_arrow_shaft(
    commands: &mut Commands,
    arrow_assets: &ArrowAssets,
    start: Vec3,
    end: Vec3,
    thickness: f32,
    color: ArrowColor,
) {
    let direction = end - start;
    let midpoint = start + direction / 2.0;
    let angle = direction.truncate().to_angle();
    let mesh = arrow_assets.shaft_unit_mesh.clone();
    let material = arrow_assets.materials.get(&color).unwrap().clone();

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        ArrowMarker,
        Transform {
            translation: midpoint,
            rotation: Quat::from_rotation_z(angle),
            scale: Vec3::new(direction.length(), thickness, 1.0),
        },
    ));
}

fn draw_arrow_tip(
    commands: &mut Commands,
    arrow_assets: &ArrowAssets,
    start: Vec3,
    end: Vec3,
    thickness: f32,
    color: ArrowColor,
) {
    let start2d = start.truncate();
    let end2d = end.truncate();
    let dir = end2d - start2d;
    let length = dir.length();

    if length == 0.0 {
        return;
    }

    let angle = dir.to_angle();
    let mesh = arrow_assets.tip_unit_mesh.clone();
    let material = arrow_assets.materials.get(&color).unwrap().clone();

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform {
            translation: Vec3::new(end.x, end.y, start.z),
            rotation: Quat::from_rotation_z(angle),
            scale: Vec3::new(length, thickness / 2.0, 1.0),
        },
        ArrowMarker,
    ));
}
