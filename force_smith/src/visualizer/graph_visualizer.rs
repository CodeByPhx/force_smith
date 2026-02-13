use bevy::prelude::*;

use crate::visualizer::{VisualizerStates, global_resources::GraphResource, layout::LayoutMode};

pub struct GraphVisualizerPlugin;
impl Plugin for GraphVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GraphVisualizerPluginConfig::default());
        app.add_message::<NodeDestinations>();
        app.add_systems(
            Update,
            spawn_graph
                .run_if(resource_changed::<GraphResource>)
                .in_set(VisualizerStates::BeforeIteration),
        );
        todo!("Add Layout Visualizer Logic");
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
            node_movement_speed: 10.0,
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
pub struct Destination(Vec2);
impl From<Vec2> for Destination {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

#[derive(Message, Deref)]
pub struct NodeDestinations(Vec<Vec2>);

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

fn attach_destinations(
    mut new_destinations: MessageReader<NodeDestinations>,
    nodes: Query<(&Index, Entity), With<NodeMarker>>,
    mut commands: Commands,
) {
    let Some(destinations) = new_destinations.read().last() else {
        return;
    };
    for (idx, entity) in nodes {
        let Some(destination) = destinations.get(**idx) else {
            continue;
        };
        commands
            .entity(entity)
            .insert(Destination::from(*destination));
    }
}

fn debug_place_nodes(
    nodes: Query<(&mut Transform, &Destination, Entity), With<NodeMarker>>,
    mut commands: Commands,
    mut layout_mode: ResMut<LayoutMode>,
) {
    for (mut transform, destination, entity) in nodes {
        transform.translation = destination.extend(0.0);
        commands.entity(entity).remove::<Destination>();
    }
    *layout_mode = LayoutMode::DebugStop;
}

fn debug_show_forces(
    nodes: Query<(&Index, &Transform), With<NodeMarker>>,
    mut commands: Commands,
    mut layout_mode: ResMut<LayoutMode>,
) {
    *layout_mode = LayoutMode::Stop;
}

fn move_nodes(
    nodes: Query<(&mut Transform, &Destination, Entity), With<NodeMarker>>,
    time: Res<Time>,
    config: Res<GraphVisualizerPluginConfig>,
    mut commands: Commands,
) {
    for (mut transform, destination, entity) in nodes {
        let origin_pos = transform.translation;
        let target_pos = destination.extend(0.0);

        let direction = target_pos - origin_pos;
        let (norm_dir, distance) = direction.normalize_and_length();

        let movement = norm_dir * config.node_movement_speed * time.delta_secs();

        if movement.length() > distance || norm_dir.is_nan() {
            transform.translation = target_pos;
            commands.entity(entity).remove::<Destination>();
        } else {
            transform.translation += movement;
        }
    }
}
