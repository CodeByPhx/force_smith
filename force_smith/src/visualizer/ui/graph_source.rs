use std::time::Duration;

use bevy::{ecs::error::info, prelude::*, time::common_conditions::on_timer};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::{
    layout::types::BaseGraph,
    visualizer::{
        VisualizerStates, global_resources::GraphResource, ui::helpers::up_down_arrow_buttons_usize,
    },
};

pub struct GraphSourceUI;
impl Plugin for GraphSourceUI {
    fn build(&self, app: &mut App) {
        app.insert_resource(GraphSourceUIContext::default())
            .add_message::<LoadGraph>()
            .add_systems(
                Update,
                (
                    load_graph.in_set(VisualizerStates::BeforeIteration),
                    update_file_list.run_if(on_timer(Duration::from_secs(1))),
                ),
            )
            .add_systems(EguiPrimaryContextPass, graph_source_ui);
    }
}

#[derive(Resource, Default)]
struct GraphSourceUIContext {
    graph_from_file: GraphFromFile,
    graph_from_config: GraphFromConfig,
    selection: GraphSourceSelection,
}

#[derive(PartialEq, Default)]
enum GraphSourceSelection {
    #[default]
    FromFile,
    FromConfig,
}

struct GraphFromFile {
    source_dir_path: String,
    files: Vec<String>,
    selected_file: Option<String>,
}
impl Default for GraphFromFile {
    fn default() -> Self {
        Self {
            source_dir_path: "./graphs".to_string(),
            files: Default::default(),
            selected_file: Default::default(),
        }
    }
}

struct GraphFromConfig {
    vertices: usize,
    edges: usize,
    connected: bool,
}
impl Default for GraphFromConfig {
    fn default() -> Self {
        Self {
            vertices: 2,
            edges: 1,
            connected: false,
        }
    }
}

fn graph_source_ui(
    mut context: EguiContexts,
    mut ctx: ResMut<GraphSourceUIContext>,
    mut send_load_graph: MessageWriter<LoadGraph>,
) {
    let Ok(context) = context.ctx_mut() else {
        return;
    };
    egui::Window::new("Graph Configuration").show(context, |ui| {
        // Selection Bar
        ui.vertical(|ui| {
            if ui
                .radio(
                    matches!(ctx.selection, GraphSourceSelection::FromConfig),
                    "From Config",
                )
                .clicked()
            {
                ctx.selection = GraphSourceSelection::FromConfig;
            }
            if ui
                .radio(
                    matches!(ctx.selection, GraphSourceSelection::FromFile),
                    "From File",
                )
                .clicked()
            {
                ctx.selection = GraphSourceSelection::FromFile;
            }
        });
        // GraphFromFile
        if ctx.selection == GraphSourceSelection::FromFile {
            let ctx = &mut ctx.graph_from_file;
            ui.heading("Graph from File");
            ui.horizontal(|ui| {
                ui.label("Path to graphs: ");
                ui.text_edit_singleline(&mut ctx.source_dir_path);
            });
            for file in ctx.files.iter() {
                ui.horizontal(|ui| {
                    if ui
                        .radio(
                            ctx.selected_file.as_ref().is_some_and(|f| f == file),
                            file.clone(),
                        )
                        .clicked()
                    {
                        ctx.selected_file = Some(file.clone());
                    };
                });
            }
            if let Some(selected_file) = &ctx.selected_file
                && ui.button("Apply").clicked()
            {
                send_load_graph.write(LoadGraph::from(format!(
                    "{}/{}",
                    ctx.source_dir_path, selected_file
                )));
            }
        }
        // GraphFromConfig
        if ctx.selection == GraphSourceSelection::FromConfig {
            let ctx = &mut ctx.graph_from_config;
            ui.heading("Graph from Config");
            ui.horizontal(|ui| {
                ui.label("Vertices: ");
                up_down_arrow_buttons_usize(&mut ctx.vertices, ui, 1);
            });
            ui.horizontal(|ui| {
                ui.label("Edges: ");
                up_down_arrow_buttons_usize(&mut ctx.edges, ui, 1);
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut ctx.connected, "Connected");
            });
            if ui.button("Apply").clicked() {
                todo!("Submit Event");
            };
        }
    });
}

fn update_file_list(mut ctx: ResMut<GraphSourceUIContext>) {
    let ctx = &mut ctx.graph_from_file;
    let Ok(entries) = std::fs::read_dir(ctx.source_dir_path.clone()) else {
        ctx.files.clear();
        return;
    };
    let mut file_names = Vec::with_capacity(ctx.files.len());
    let mut saw_selected_file = false;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if ctx.selected_file.as_ref().is_some_and(|f| f == file_name) {
                saw_selected_file = true;
            }
            file_names.push(file_name.to_string());
        }
    }
    if !saw_selected_file {
        ctx.selected_file = None;
    }
    ctx.files = file_names;
}

#[derive(Message)]
struct LoadGraph {
    path: String,
}
impl From<String> for LoadGraph {
    fn from(value: String) -> Self {
        Self { path: value }
    }
}

fn load_graph(mut read_load_graph: MessageReader<LoadGraph>, mut commands: Commands) {
    let Some(LoadGraph { path }) = read_load_graph.read().last() else {
        return;
    };
    info!("2");
    let Ok(raw_content) = std::fs::read_to_string(path) else {
        return;
    };
    info!("3");
    let Ok(graph) = serde_json::from_str::<BaseGraph>(&raw_content) else {
        return;
    };
    info!("4");
    commands.insert_resource(GraphResource::from(graph));
    info!("Loaded graph");
}

#[derive(Message)]
struct GenerateGraph {
    vertices: usize,
    edges: usize,
    connected: bool,
}
