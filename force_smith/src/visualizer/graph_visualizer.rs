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
            draw_arrow(
                &mut commands,
                &mut meshes,
                &mut materials,
                origin,
                displacement,
                shaft_thickness,
                tip_thickness,
                Color::srgb(1.0, 0.0, 0.0),
                0.9,
            );
        }
        let destination = destinations[idx].extend(0.0);
        draw_arrow(
            &mut commands,
            &mut meshes,
            &mut materials,
            origin,
            destination - origin,
            shaft_thickness,
            tip_thickness,
            Color::srgb(0.0, 1.0, 0.0),
            0.9,
        );
    }
    mode.state = LayoutState::Debug(DebugState::StopBeforeUpdate);
}

#[allow(clippy::too_many_arguments)]
fn draw_arrow(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    origin: Vec3,
    displacement: Vec3,
    shaft_thickness: f32,
    tip_thickness: f32,
    color: Color,
    shaft_tip_ratio: f32,
) {
    let direction = displacement - origin;
    let (norm_dir, length) = direction.normalize_and_length();
    let shaft = direction * shaft_tip_ratio;
    let tip = direction * (1.0 - shaft_tip_ratio);
    draw_arrow_shaft(
        commands,
        meshes,
        materials,
        origin,
        shaft,
        shaft_thickness,
        color,
    );
    draw_arrow_tip(
        commands,
        meshes,
        materials,
        origin + shaft,
        tip,
        tip_thickness,
        color,
    );
}

#[allow(clippy::too_many_arguments)]
fn draw_arrow_shaft(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    origin: Vec3,
    dir: Vec3,
    thickness: f32,
    color: Color,
) {
    let midpoint = origin + dir / 2.0;
    let angle = dir.y.atan2(dir.x);
    let mesh = meshes.add(Rectangle::new(dir.length(), thickness));
    let color = materials.add(color);
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(color),
        ArrowMarker,
        Transform {
            translation: midpoint,
            rotation: Quat::from_rotation_z(angle),
            ..Default::default()
        },
    ));
}

#[allow(clippy::too_many_arguments)]
fn draw_arrow_tip(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    origin: Vec3,
    direction: Vec3,
    thickness: f32,
    color: Color,
) {
    let (norm_dir, length) = direction.normalize_and_length();
    // 2. Compute perpendicular vector for base width
    let perp = Vec3::new(-norm_dir.y, norm_dir.x, 0.0); // perpendicular in XY plane

    // 3. Define triangle points
    let tip = origin + norm_dir * length; // arrow tip point
    let base_left = origin + perp * (thickness / 2.0); // left base
    let base_right = origin - perp * (thickness / 2.0); // right base

    // 4. Create the triangle mesh
    let mesh_handle = meshes.add(Triangle2d::new(
        tip.truncate(), // convert Vec3 to Vec2
        base_left.truncate(),
        base_right.truncate(),
    ));
    let color = materials.add(color);

    // 5. Spawn the triangle as a 2D mesh
    commands.spawn((
        ArrowMarker,
        Mesh2d(mesh_handle),
        MeshMaterial2d(color),
        Transform::default(),
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
