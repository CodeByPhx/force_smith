use std::{collections::HashMap, fs, path::Path, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::layout::{LayoutAlgorithm, types::BaseGraph};

pub trait VisualLayoutAlgorithm: LayoutAlgorithm + Parameterized {}

pub trait Parameterized {
    fn get_parameters(&self) -> HashMap<String, Parameter>;
    fn update_parameters(&mut self, parameters: &HashMap<String, Parameter>);
}

#[derive(Deref, DerefMut)]
pub struct LayoutResource(Box<dyn VisualLayoutAlgorithm>);
impl From<Box<dyn VisualLayoutAlgorithm>> for LayoutResource {
    fn from(value: Box<dyn VisualLayoutAlgorithm>) -> Self {
        Self(value)
    }
}

impl From<crate::utils::vec2::Vec2> for Vec2 {
    fn from(value: crate::utils::vec2::Vec2) -> Self {
        Vec2::new(value.x, value.y)
    }
}

impl From<&crate::utils::vec2::Vec2> for Vec2 {
    fn from(value: &crate::utils::vec2::Vec2) -> Self {
        Vec2::new(value.x, value.y)
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct LayoutConfigResource(HashMap<String, Parameter>);
pub enum Parameter {
    Float(f32),
    Bool(bool),
}
impl Parameter {
    fn add_ui_element(&mut self, ui: &mut egui::Ui) {
        match self {
            Parameter::Float(v) => {
                ui.add(egui::DragValue::new(v).speed(0.01));
            }
            Parameter::Bool(v) => {
                ui.checkbox(v, ());
            }
        }
    }
}
impl From<HashMap<String, Parameter>> for LayoutConfigResource {
    fn from(value: HashMap<String, Parameter>) -> Self {
        Self(value)
    }
}
pub trait FromParameter: Sized {
    fn from_parameter(p: &Parameter) -> Option<Self>;
}
pub trait ToParameter {
    fn to_parameter(&self) -> Parameter;
}
impl FromParameter for bool {
    fn from_parameter(p: &Parameter) -> Option<Self> {
        if let Parameter::Bool(value) = p {
            Some(*value)
        } else {
            None
        }
    }
}
impl ToParameter for bool {
    fn to_parameter(&self) -> Parameter {
        Parameter::Bool(*self)
    }
}
impl FromParameter for f32 {
    fn from_parameter(p: &Parameter) -> Option<Self> {
        if let Parameter::Float(value) = p {
            Some(*value)
        } else {
            None
        }
    }
}
impl ToParameter for f32 {
    fn to_parameter(&self) -> Parameter {
        Parameter::Float(*self)
    }
}

#[derive(Message, Deref, DerefMut)]
pub struct NewGraph(BaseGraph);

#[derive(Resource, Deref, DerefMut)]
pub struct NodeMovementSpeed(f32);

#[derive(Component, Default)]
pub struct NodeMarker;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Id(usize);

#[derive(Component, Deref, DerefMut)]
pub struct Destination(Vec2);

pub fn visualize(layout: Box<dyn VisualLayoutAlgorithm>) {
    let parameters = layout.get_parameters();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .insert_resource(GraphSourceUIContext::default())
        .insert_non_send_resource(LayoutResource::from(layout))
        .insert_resource(LayoutConfigResource::from(parameters))
        .insert_resource(NodeMovementSpeed(10.0))
        .add_message::<NewGraph>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                // load_graph_from_file.run_if(on_timer(Duration::from_secs(5))),
                update_source_graph_dirs.run_if(on_timer(Duration::from_secs(5))),
                load_ui_nodes.after(load_graph_from_file),
                update_layout_config
                    .run_if(resource_changed::<LayoutConfigResource>)
                    .before(iterate_layout),
                update_layout_graph
                    .after(load_graph_from_file)
                    .before(iterate_layout),
                iterate_layout,
                update_destinations.after(iterate_layout),
                move_nodes.after(update_destinations),
            ),
        )
        .add_systems(EguiPrimaryContextPass, (config_ui, graph_source_ui))
        .run();
}

#[derive(Bundle, Default)]
pub struct NodeBundle {
    pub marker: NodeMarker,
    pub id: Id,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
}

pub fn load_ui_nodes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    old_nodes: Query<Entity, With<NodeMarker>>,
    mut ev_new_graph: MessageReader<NewGraph>,
) {
    if let Some(new_graph) = ev_new_graph.read().last() {
        for entity in old_nodes {
            commands.entity(entity).despawn();
        }

        let mesh = meshes.add(Circle::new(10.0));
        let material = materials.add(Color::srgb(1.0, 0.0, 0.0));

        let node_bundles: Vec<NodeBundle> = new_graph
            .vertices
            .iter()
            .enumerate()
            .map(|(id, pos)| NodeBundle {
                marker: NodeMarker,
                id: Id(id),
                mesh: Mesh2d(mesh.clone()),
                material: MeshMaterial2d(material.clone()),
                transform: Transform::from_translation(Vec2::from(pos).extend(0.0)),
            })
            .collect();

        commands.spawn_batch(node_bundles);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn load_graph_from_file(mut ev_new_graph: MessageWriter<NewGraph>) {
    let path = "./graphs/graph1.json";
    let content = std::fs::read_to_string(path).unwrap();
    let new_graph: BaseGraph = serde_json::from_str(&content).unwrap();

    ev_new_graph.write(NewGraph(new_graph));
}

#[derive(Resource)]
pub struct GraphSourceUIContext {
    selected_file: Option<String>,
    files: Vec<String>,
    path: String,
}

impl Default for GraphSourceUIContext {
    fn default() -> Self {
        Self {
            selected_file: Default::default(),
            files: Default::default(),
            path: "./graphs/".to_string(),
        }
    }
}

fn update_source_graph_dirs(mut graph_source_context: ResMut<GraphSourceUIContext>) {
    let mut file_names: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(graph_source_context.path.clone()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            #[allow(clippy::collapsible_if)]
            if path.extension().is_some_and(|ext| ext == "json") {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    file_names.push(file_name.to_string());
                }
            }
        }
    }
    if graph_source_context
        .selected_file
        .as_ref()
        .is_some_and(|f| !file_names.contains(f))
    {
        graph_source_context.selected_file = None;
    }
    graph_source_context.files = file_names;
}

fn graph_source_ui(
    mut contexts: EguiContexts,
    mut graph_source_context: ResMut<GraphSourceUIContext>,
) {
    warn!("Files are {:?}", graph_source_context.files);
    if let Ok(context) = contexts.ctx_mut() {
        egui::Window::new("Source Graph").show(context, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for file in graph_source_context.files.clone() {
                    ui.horizontal(|ui| {
                        let selected = graph_source_context
                            .selected_file
                            .as_ref()
                            .is_some_and(|f| f == &file);
                        if ui.radio(selected, file.clone()).clicked() {
                            graph_source_context.selected_file = Some(file);
                        }
                    });
                }
            })
        });
    }
}

pub fn config_ui(mut contexts: EguiContexts, mut config: ResMut<LayoutConfigResource>) {
    if let Ok(context) = contexts.ctx_mut() {
        egui::Window::new("Parameter Configuration")
            .resizable(false)
            // .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0])
            .show(context, |ui| {
                egui::Grid::new("parameter_grid").show(ui, |ui| {
                    for (name, parameter) in config.iter_mut() {
                        ui.horizontal(|ui| {
                            ui.label(name);
                            parameter.add_ui_element(ui);
                        });
                        ui.end_row();
                    }
                });
            });
    }
}

fn update_layout_config(mut layout: NonSendMut<LayoutResource>, config: Res<LayoutConfigResource>) {
    layout.update_parameters(&config.0);
}

fn update_layout_graph(
    mut layout: NonSendMut<LayoutResource>,
    mut ev_new_graph: MessageReader<NewGraph>,
) {
    if let Some(new_graph) = ev_new_graph.read().last() {
        layout.set_graph(new_graph);
    }
}

fn iterate_layout(mut layout: NonSendMut<LayoutResource>) {
    info!("Iterating");
    layout.iterate()
}

fn update_destinations(
    layout: NonSend<LayoutResource>,
    vertices: Query<(&Id, Entity), With<NodeMarker>>,
    mut commands: Commands,
) {
    let destinations: Vec<Vec2> = layout.get_positions().into_iter().map(Vec2::from).collect();
    for (id, entity) in vertices {
        if let Some(destination) = destinations.get(id.0) {
            commands.entity(entity).insert(Destination(*destination));
        }
    }
}

fn move_nodes(
    mut vertices: Query<(Entity, &mut Transform, &Destination), With<NodeMarker>>,
    time: Res<Time>,
    speed: Res<NodeMovementSpeed>,
    mut commands: Commands,
) {
    let speed = speed.0;
    for (entity, mut transform, destination) in &mut vertices {
        let position_3d = transform.translation;
        let position_2d = Vec2::new(position_3d.x, position_3d.y);

        let direction = destination.0 - position_2d;
        if direction.length() <= speed {
            transform.translation = destination.extend(0.0);
            commands.entity(entity).remove::<Destination>();
        } else {
            let norm_dir = direction.normalize();
            transform.translation += norm_dir.extend(0.0) * speed * time.delta_secs();
        }
    }
}
