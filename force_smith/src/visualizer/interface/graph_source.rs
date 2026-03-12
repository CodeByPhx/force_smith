use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use std::time::Duration;

use crate::{
    graph::{Edge, Graph},
    visualizer::{
        global_schedule::VisualizerStates, interface::helpers::up_down_arrow_buttons_usize,
        simulation::resource::LoadGraph,
    },
};

pub struct GraphSourceUI;
impl Plugin for GraphSourceUI {
    fn build(&self, app: &mut App) {
        app.insert_resource(GraphSourceUIContext::default())
            .add_message::<LoadGraphFromFile>()
            .add_message::<GenerateGraphMessage>()
            .add_systems(
                Update,
                (
                    load_graph_from_file.in_set(VisualizerStates::BeforeIteration),
                    generate_graph.in_set(VisualizerStates::BeforeIteration),
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
            connected: true,
        }
    }
}

fn graph_source_ui(
    mut context: EguiContexts,
    mut ctx: ResMut<GraphSourceUIContext>,
    mut send_load_graph: MessageWriter<LoadGraphFromFile>,
    mut send_generate_graph: MessageWriter<GenerateGraphMessage>,
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
                send_load_graph.write(LoadGraphFromFile::from(format!(
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
                send_generate_graph.write(GenerateGraphMessage {
                    vertices: ctx.vertices,
                    edges: ctx.edges,
                    connected: ctx.connected,
                });
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
struct LoadGraphFromFile {
    path: String,
}
impl From<String> for LoadGraphFromFile {
    fn from(value: String) -> Self {
        Self { path: value }
    }
}

fn load_graph_from_file(
    mut read_load_graph: MessageReader<LoadGraphFromFile>,
    mut send_load_graph: MessageWriter<LoadGraph>,
) {
    let Some(LoadGraphFromFile { path }) = read_load_graph.read().last() else {
        return;
    };
    let Ok(raw_content) = std::fs::read_to_string(path) else {
        error!("Failed to read graph file: {}", path);
        return;
    };
    let Ok(graph) = serde_json::from_str::<Graph>(&raw_content) else {
        error!("Failed to parse graph from JSON");
        return;
    };
    info!("Loaded graph from file");
    send_load_graph.write(LoadGraph(graph));
}

#[derive(Message)]
struct GenerateGraphMessage {
    vertices: usize,
    edges: usize,
    connected: bool,
}

fn generate_graph(
    mut read_generate: MessageReader<GenerateGraphMessage>,
    mut send_load_graph: MessageWriter<LoadGraph>,
) {
    let Some(GenerateGraphMessage {
        vertices,
        edges,
        connected,
    }) = read_generate.read().last()
    else {
        return;
    };

    use rand::RngExt;
    let mut rng = rand::rng();

    let mut graph_edges: Vec<Edge> = Vec::with_capacity(*edges);

    // If connected, create a spanning tree first
    if *connected && *vertices > 1 {
        let mut connected_vertices: Vec<usize> = vec![0];
        let mut disconnected_vertices: Vec<usize> = (1..*vertices).collect();

        while !disconnected_vertices.is_empty() {
            let disconnected_idx = rng.random_range(0..disconnected_vertices.len());
            let connected_idx = rng.random_range(0..connected_vertices.len());

            let new_vertex = disconnected_vertices.swap_remove(disconnected_idx);
            let existing_vertex = connected_vertices[connected_idx];

            graph_edges.push(Edge {
                from: existing_vertex,
                to: new_vertex,
            });

            connected_vertices.push(new_vertex);
        }
    }

    // Add remaining edges randomly
    while graph_edges.len() < *edges && *vertices > 1 {
        let from = rng.random_range(0..*vertices);
        let mut to = rng.random_range(0..*vertices);

        // Avoid self-loops
        if to == from {
            to = (to + 1) % vertices;
        }

        // Check for duplicate edges (optional, for simple graphs)
        let edge = Edge { from, to };
        if !graph_edges.iter().any(|e| {
            (e.from == edge.from && e.to == edge.to) || (e.from == edge.to && e.to == edge.from)
        }) {
            graph_edges.push(edge);
        }
    }

    let graph = Graph {
        vertices: *vertices,
        edges: graph_edges,
    };

    info!(
        "Generated graph with {} vertices and {} edges{}",
        vertices,
        graph.edges.len(),
        if *connected { " (connected)" } else { "" }
    );

    send_load_graph.write(LoadGraph(graph));
}
