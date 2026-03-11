use crate::graph::Graph;
use crate::visualizer::rendering::SetInitialGraph;
use crate::visualizer::{
    global_schedule::VisualizerStates,
    layout_trait::{Parameter, ParameterizedDebugLayoutAlgorithm},
};
use bevy::prelude::*;

pub struct ResourcePlugin;
impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LoadGraph>().add_systems(
            Update,
            (
                // Chain these systems to ensure parameters update before graph
                update_layout_parameters.run_if(any_parameter_changed),
                update_layout_graph.run_if(at_least_one_message::<LoadGraph>),
            )
                .chain() // Ensures sequential execution within BeforeIteration
                .in_set(VisualizerStates::BeforeIteration),
        );
    }
}

fn any_parameter_changed(parameters: Res<LayoutParameterResource>) -> bool {
    parameters.iter().any(|p| p.check_changed())
}

fn update_layout_parameters(
    mut layout: NonSendMut<LayoutResource>,
    mut parameters: ResMut<LayoutParameterResource>,
) {
    let changed_parameters: Vec<Parameter> = parameters
        .iter_mut()
        .filter(|p| p.check_changed())
        .map(|p| {
            p.set_unchanged();
            p.clone()
        })
        .collect();
    layout.update_parameters(&changed_parameters);
}

pub fn at_least_one_message<M: Message>(message_reader: MessageReader<M>) -> bool {
    !message_reader.is_empty()
}

fn update_layout_graph(
    mut load_graph: MessageReader<LoadGraph>,
    mut layout: NonSendMut<LayoutResource>,
    mut set_initial_graph: MessageWriter<SetInitialGraph>,
) {
    let Some(LoadGraph(graph)) = load_graph.read().last() else {
        return;
    };
    info!("Loading graph with {} vertices and {} edges", graph.vertices, graph.edges.len());
    layout.load_graph(graph);
    let positions = layout.get_positions();
    let edges = layout.get_edges();
    info!("Graph loaded. First position: {:?}", positions.first());
    set_initial_graph.write(SetInitialGraph {
        vertices: positions,
        edges,
    });
}

#[derive(Message)]
pub struct LoadGraph(pub Graph);

#[derive(Deref, DerefMut)]
pub struct LayoutResource(pub Box<dyn ParameterizedDebugLayoutAlgorithm>);
impl From<Box<dyn ParameterizedDebugLayoutAlgorithm>> for LayoutResource {
    fn from(value: Box<dyn ParameterizedDebugLayoutAlgorithm>) -> Self {
        Self(value)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct LayoutParameterResource(pub Vec<DeltaParameter>);
impl From<Vec<Parameter>> for LayoutParameterResource {
    fn from(value: Vec<Parameter>) -> Self {
        Self(value.into_iter().map(DeltaParameter::from).collect())
    }
}

pub struct DeltaParameter {
    pub current: Parameter,
    pub previous: Parameter,
}
impl DeltaParameter {
    pub fn check_changed(&self) -> bool {
        self.current != self.previous
    }
    pub fn set_unchanged(&mut self) {
        self.previous = self.current.clone();
    }

    /// Add UI element for this parameter using egui
    #[cfg(feature = "visualizer")]
    pub fn add_ui_element(&mut self, ui: &mut bevy_egui::egui::Ui) {
        self.current.value.add_ui_element(ui);
    }
}
impl From<Parameter> for DeltaParameter {
    fn from(value: Parameter) -> Self {
        Self {
            current: value.clone(),
            previous: value.clone(),
        }
    }
}
impl From<DeltaParameter> for Parameter {
    fn from(value: DeltaParameter) -> Self {
        value.current
    }
}
impl std::ops::Deref for DeltaParameter {
    type Target = Parameter;

    fn deref(&self) -> &Self::Target {
        &self.current
    }
}
impl std::ops::DerefMut for DeltaParameter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.current
    }
}
