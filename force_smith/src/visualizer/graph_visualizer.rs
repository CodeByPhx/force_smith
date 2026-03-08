use bevy::prelude::*;

use crate::visualizer::{
    VisualizerStates,
    global_resources::GraphResource,
    layout::{DebugState, LayoutMode, LayoutState, NormalState, in_layout_state},
};

pub struct GraphVisualizerPlugin;
impl Plugin for GraphVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GraphVisualizerPluginConfig::default());
        app.add_systems(
            Update,
            (
                (cleanup_debug_mode).run_if(layout_mode_changed_to_normal),
                (spawn_graph.run_if(resource_changed::<GraphResource>),)
                    .in_set(VisualizerStates::BeforeIteration),
                (move_nodes)
                    .run_if(in_layout_state(LayoutState::Normal(NormalState::Run)))
                    .in_set(VisualizerStates::AfterIteration),
                (debug_place_nodes)
                    .run_if(in_layout_state(LayoutState::Debug(
                        DebugState::UpdatePositions,
                    )))
                    .in_set(VisualizerStates::AfterIteration),
                (place_nodes)
                    .run_if(in_layout_state(LayoutState::Normal(
                        NormalState::PlaceDestinations,
                    )))
                    .in_set(VisualizerStates::AfterIteration),
                (debug_show_forces).in_set(VisualizerStates::AfterIteration),
                (debug_remove_forces)
                    .run_if(in_layout_state(LayoutState::Debug(
                        DebugState::RemoveForces,
                    )))
                    .in_set(VisualizerStates::AfterIteration),
            ),
        );
    }
}

fn layout_mode_changed_to_normal(mode: Res<LayoutMode>) -> bool {
    mode.mode_change && matches!(mode.state, LayoutState::Normal(_))
}

fn cleanup_debug_mode(
    mut mode: ResMut<LayoutMode>,
    forces: Query<Entity, With<ArrowMarker>>,
    nodes: Query<(&mut Transform, &Destination, Entity), With<NodeMarker>>,
    mut commands: Commands,
) {
    mode.mode_change = false;

    forces.iter().for_each(|e| commands.entity(e).despawn());

    for (mut transform, destination, entity) in nodes {
        transform.translation = destination.extend(0.0);

        commands.entity(entity).remove::<Destination>();
    }
}

#[derive(Resource)]
pub struct GraphVisualizerPluginConfig {
    node_radius: f32,
    node_color: Color,
    node_movement_speed: f32,
}
impl Default for GraphVisualizerPluginConfig {
    fn default() -> Self {
        Self {
            node_radius: 10.0,
            node_color: Color::srgb(1.0, 0.0, 0.0),
            node_movement_speed: 100.0,
        }
    }
}

#[derive(Component)]
pub struct NodeMarker;
#[derive(Component, Deref, DerefMut)]
pub struct Index(pub usize);
impl From<usize> for Index {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
#[derive(Bundle)]
pub struct NodeBundle {
    pub marker: NodeMarker,
    pub idx: Index,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}
#[derive(Component, Deref, DerefMut)]
pub struct Destination(pub Vec2);
impl From<Vec2> for Destination {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

fn spawn_graph(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    old_nodes: Query<Entity, With<NodeMarker>>,
    graph: Res<GraphResource>,
    config: Res<GraphVisualizerPluginConfig>,
) {
    old_nodes.iter().for_each(|e| commands.entity(e).despawn());

    let mut nodes: Vec<NodeBundle> = Vec::with_capacity(graph.vertices.len());
    let mesh = meshes.add(Circle::new(config.node_radius));
    let material = material.add(config.node_color);
    for (idx, position) in graph.vertices.iter().enumerate() {
        nodes.push(NodeBundle {
            marker: NodeMarker,
            idx: idx.into(),
            mesh: Mesh2d(mesh.clone()),
            material: MeshMaterial2d(material.clone()),
            transform: Transform::from_translation(position.extend(0.0)),
        });
    }
    commands.spawn_batch(nodes);
}

fn debug_place_nodes(
    mut mode: ResMut<LayoutMode>,
    nodes: Query<(&mut Transform, &Destination, Entity), With<NodeMarker>>,
    mut commands: Commands,
) {
    for (mut transform, destination, entity) in nodes {
        info!(
            "Pos old : {}, pos new : {}",
            transform.translation, destination.0
        );
        transform.translation = destination.extend(0.0);

        commands.entity(entity).remove::<Destination>();
    }
    mode.state = LayoutState::Debug(DebugState::Stop);
}

fn place_nodes(
    mut mode: ResMut<LayoutMode>,
    nodes: Query<(&mut Transform, &Destination, Entity), With<NodeMarker>>,
    mut commands: Commands,
) {
    for (mut transform, destination, entity) in nodes {
        transform.translation = destination.extend(0.0);
        commands.entity(entity).remove::<Destination>();
    }
    mode.state = LayoutState::Normal(NormalState::Stop);
}

#[derive(Component)]
pub struct ArrowMarker;

#[derive(Bundle)]
pub struct ArrowShaftBundle {
    pub marker: ArrowMarker,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}

#[derive(Bundle)]
pub struct ArrowTipBundle {
    pub marker: ArrowMarker,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}

fn debug_show_forces(
    nodes: Query<(&Index, &Transform), With<NodeMarker>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mode: ResMut<LayoutMode>,
) {
    let LayoutState::Debug(DebugState::ShowForces {
        forces,
        destinations,
    }) = &mode.state
    else {
        return;
    };

    for (&Index(idx), transform) in nodes.iter() {
        let origin = transform.translation;
        let shaft_thickness = 2.0;
        let tip_thickness = shaft_thickness * 10.0;

        for force in forces {
            let displacement = force[idx].extend(0.0);
            let end = origin + displacement;
            draw_arrow2d(
                &mut commands,
                &mut meshes,
                &mut materials,
                origin,
                end,
                shaft_thickness,
                tip_thickness,
                Color::srgb(1.0, 0.0, 0.0),
                0.9,
            );
        }
        let destination = destinations[idx].extend(0.0);
        draw_arrow2d(
            &mut commands,
            &mut meshes,
            &mut materials,
            origin,
            destination,
            shaft_thickness,
            tip_thickness,
            Color::srgb(0.0, 1.0, 0.0),
            0.9,
        );
    }
    mode.state = LayoutState::Debug(DebugState::StopBeforeUpdate);
}

#[allow(clippy::too_many_arguments)]
fn draw_arrow2d(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    start: Vec3,
    end: Vec3,
    shaft_thickness: f32,
    tip_thickness: f32,
    color: Color,
    shaft_tip_ratio: f32,
) {
    let direction = end - start;
    let shaft_start = start;
    let shaft_end = start + direction * shaft_tip_ratio;
    let tip_start = shaft_end;
    let tip_end = end;

    draw_arrow_shaft2d(
        commands,
        meshes,
        materials,
        shaft_start,
        shaft_end,
        shaft_thickness,
        color,
    );
    draw_arrow_tip2d(
        commands,
        meshes,
        materials,
        tip_start,
        tip_end,
        tip_thickness,
        color,
    );
}

fn draw_arrow_tip2d(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    start: Vec3,
    end: Vec3,
    thickness: f32,
    color: Color,
) {
    let start2d = start.truncate();
    let end2d = end.truncate();
    let norm_direction2d = (end2d - start2d).normalize();

    let perp = Vec2::new(-norm_direction2d.y, norm_direction2d.x) * thickness / 2.0;

    let triangle = Triangle2d::new(start2d + perp, end2d, start2d - perp);
    let mesh = meshes.add(triangle);
    let material = materials.add(color);

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        ArrowMarker,
        Transform::default(),
    ));
}

fn draw_arrow_shaft2d(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    start: Vec3,
    end: Vec3,
    thickness: f32,
    color: Color,
) {
    let direction = end - start;
    let midpoint = start + direction / 2.0;
    let angle = direction.truncate().to_angle();
    let mesh = meshes.add(Rectangle::new(1.0, 1.0));
    let material = materials.add(color);
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

fn debug_remove_forces(
    mut mode: ResMut<LayoutMode>,
    forces: Query<Entity, With<ArrowMarker>>,
    mut commands: Commands,
) {
    forces.iter().for_each(|e| commands.entity(e).despawn());
    mode.state = LayoutState::Debug(DebugState::UpdatePositions);
}

fn move_nodes(
    nodes: Query<(&mut Transform, &Destination, Entity), With<NodeMarker>>,
    time: Res<Time>,
    config: Res<GraphVisualizerPluginConfig>,
    mut commands: Commands,
) {
    for (mut transform, destination, entity) in nodes {
        info!("Inside move nodes");
        let origin_pos = transform.translation;
        let target_pos = destination.extend(0.0);

        let direction = target_pos - origin_pos;
        let (norm_dir, distance) = direction.normalize_and_length();

        let movement = norm_dir * config.node_movement_speed * time.delta_secs();

        if movement.length() > distance || norm_dir.is_nan() {
            info!("Inside here");
            transform.translation = target_pos;
            commands.entity(entity).remove::<Destination>();
        } else {
            info!("Inside else, with len {}", movement.length());
            transform.translation += movement;
        }
    }
}
