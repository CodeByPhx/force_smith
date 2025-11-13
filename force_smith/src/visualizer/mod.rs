use std::{collections::HashMap, fmt::format, fs, path::Path, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_egui::{
    EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{self, Ui},
};
use rand::Rng;

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
#[derive(Debug)]
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
        .insert_resource(GraphSourceUiContext::default())
        .insert_non_send_resource(LayoutResource::from(layout))
        .insert_resource(LayoutConfigResource::from(parameters))
        .insert_resource(NodeMovementSpeed(10.0))
        .add_message::<NewGraph>()
        .add_message::<LoadGraph>()
        .add_message::<GenerateGraph>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (generate_graph, load_graph_from_file).in_set(NewGraphSet),
                (load_ui_nodes, update_layout_graph)
                    .in_set(InsertNewGraphSet)
                    .after(NewGraphSet),
                update_source_graph_dirs.run_if(on_timer(Duration::from_secs(1))),
                update_layout_config.run_if(resource_changed::<LayoutConfigResource>),
            )
                .in_set(ConfigurationSet),
        )
        // .add_systems(
        //     Update,
        //     (
        //         iterate_layout,
        //         update_destinations.after(iterate_layout),
        //         move_nodes.after(update_destinations),
        //     )
        //         .in_set(UpdateSet)
        //         .after(ConfigurationSet),
        // )
        .add_systems(EguiPrimaryContextPass, (config_ui, graph_source_ui))
        .run();
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ConfigurationSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct NewGraphSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct InsertNewGraphSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct UpdateSet;

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

fn load_graph_from_file(
    mut ev_load_graph: MessageReader<LoadGraph>,
    mut ev_new_graph: MessageWriter<NewGraph>,
) {
    if let Some(LoadGraph { path }) = ev_load_graph.read().last() {
        let content = std::fs::read_to_string(path).unwrap();
        let new_graph: BaseGraph = serde_json::from_str(&content).unwrap();
        ev_new_graph.write(NewGraph(new_graph));
    }
}

fn generate_graph(
    mut ev_generate_graph: MessageReader<GenerateGraph>,
    mut ev_new_graph: MessageWriter<NewGraph>,
) {
    if let Some(GenerateGraph {
        vertices: vertex_count,
        edges: edge_count,
        connected,
    }) = ev_generate_graph.read().last()
    {
        use crate::utils::vec2::Vec2;
        let vertices: Vec<Vec2> = vec![Vec2::ZERO; *vertex_count];
        let mut edges: Vec<(usize, usize)> = Vec::with_capacity(*edge_count);
        let mut rng = rand::rng();
        if *connected {
            let mut connected_vertices: Vec<usize> = Vec::with_capacity(*vertex_count);
            connected_vertices.push(1);
            let mut disconnected_vertices: Vec<usize> = (1..*vertex_count).collect();
            while !disconnected_vertices.is_empty() {
                let disconnected_idx = rng.random_range(0..disconnected_vertices.len());
                let connected_idx = rng.random_range(0..connected_vertices.len());
                disconnected_vertices.swap_remove(disconnected_idx);
                connected_vertices.push(disconnected_idx);
                edges.push((connected_idx, disconnected_idx));
            }
        }
        while edges.len() < *edge_count {
            let from_idx = rng.random_range(0..vertices.len());
            let mut to_idx = rng.random_range(0..vertices.len());
            if to_idx == from_idx {
                to_idx = (to_idx + 1) % vertices.len();
            }
            edges.push((from_idx, to_idx));
        }
        ev_new_graph.write(NewGraph(BaseGraph { vertices, edges }));
        info!("Generated Graph");
    }
}

fn update_source_graph_dirs(mut graph_source_context: ResMut<GraphSourceUiContext>) {
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

fn up_down_arrow_buttons_usize(
    value: &mut usize,
    ui: &mut Ui,
    min: Option<usize>,
    max: Option<usize>,
) {
    let min = min.unwrap_or(0);
    let max = max.unwrap_or(usize::MAX);
    if *value < min {
        *value = min;
    }
    if *value > max {
        *value = max;
    }
    ui.horizontal(|ui| {
        if ui.button("-").clicked() && *value > min {
            *value -= 1;
        }
        ui.label(value.to_string());
        if ui.button("+").clicked() && *value < max {
            *value += 1;
        }
    });
}

#[derive(Message)]
struct GenerateGraph {
    vertices: usize,
    edges: usize,
    connected: bool,
}
#[derive(Message)]
struct LoadGraph {
    path: String,
}

#[derive(Resource)]
struct GraphSourceUiContext {
    path: String,
    files: Vec<String>,
    selected_file: Option<String>,
    vertices: usize,
    edges: usize,
    connected: bool,
    current_selection: Option<String>,
}
impl Default for GraphSourceUiContext {
    fn default() -> Self {
        Self {
            path: String::from("./graphs/"),
            files: Vec::new(),
            selected_file: None,
            vertices: 3,
            edges: 2,
            connected: true,
            current_selection: None,
        }
    }
}

fn graph_source_ui(
    mut ui_ctx: EguiContexts,
    mut ctx: ResMut<GraphSourceUiContext>,
    mut ev_generate_graph: MessageWriter<GenerateGraph>,
    mut ev_load_graph: MessageWriter<LoadGraph>,
) {
    if let Ok(context) = ui_ctx.ctx_mut() {
        egui::Window::new("Graph Configuration").show(context, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(selection) = &ctx.current_selection {
                    ui.colored_label(egui::Color32::from_rgb(255, 0, 0), selection.clone());
                }
                ui.heading("Generate Graph");
                ui.horizontal(|ui| {
                    ui.label("Vertices");
                    up_down_arrow_buttons_usize(&mut ctx.vertices, ui, None, None);
                });
                ui.horizontal(|ui| {
                    ui.label("Edges");
                    up_down_arrow_buttons_usize(&mut ctx.edges, ui, None, None);
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut ctx.connected, "Connected");
                });
                ui.horizontal(|ui| {
                    if ui.button("Apply").clicked() {
                        ctx.current_selection = Some(format!(
                            "Selected: Generate Graph(vertices {}, edges {}{})",
                            ctx.vertices,
                            ctx.edges,
                            if ctx.connected { ", connected" } else { "" }
                        ));
                        ev_generate_graph.write(GenerateGraph {
                            vertices: ctx.vertices,
                            edges: ctx.edges,
                            connected: ctx.connected,
                        });
                    }
                });
                ui.heading("Graph from File");
                for file in ctx.files.clone() {
                    ui.horizontal(|ui| {
                        let selected = ctx.selected_file.as_ref().is_some_and(|f| f == &file);
                        if ui.radio(selected, file.clone()).clicked() {
                            ctx.selected_file = Some(file);
                        }
                    });
                }
                #[allow(clippy::collapsible_if)]
                if let Some(selected_file) = ctx.selected_file.clone() {
                    if ui.button("Apply").clicked() {
                        ctx.current_selection = Some(format!("Selected: File {}", selected_file));
                        ev_load_graph.write(LoadGraph {
                            path: format!("{}{}", ctx.path, selected_file),
                        });
                    }
                }
            });
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
