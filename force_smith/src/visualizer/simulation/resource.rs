use crate::graph::Graph;
use crate::visualizer::layout_trait::ParameterValue;
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
                update_layout_parameters.run_if(any_parameter_changed),
                update_layout_graph.run_if(at_least_one_message::<LoadGraph>),
            )
                .chain()
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
            Parameter::from(p)
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
    info!(
        "Loading graph with {} vertices and {} edges",
        graph.vertices,
        graph.edges.len()
    );
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

#[derive(Clone)]
pub struct DeltaParameter {
    pub name: String,
    pub current_value: ParameterValue,
    pub previous_value: ParameterValue,
}
impl DeltaParameter {
    pub fn is_same_parameter(&self, parameter: &Parameter) -> bool {
        self.name == parameter.name
    }
    pub fn is_same_parameter_value(&self, parameter: &Parameter) -> bool {
        self.current_value == parameter.value
    }
    pub fn is_same(&self, parameter: &Parameter) -> bool {
        self.is_same_parameter(parameter) && self.is_same_parameter_value(parameter)
    }
    pub fn overwrite_value(&mut self, parameter: &Parameter) {
        self.current_value = parameter.value;
        self.previous_value = parameter.value;
    }

    pub fn check_changed(&self) -> bool {
        self.current_value != self.previous_value
    }

    pub fn set_unchanged(&mut self) {
        self.previous_value = self.current_value;
    }

    pub fn add_ui_element(&mut self, ui: &mut bevy_egui::egui::Ui) {
        self.current_value.add_ui_element(ui);
    }
}
impl From<Parameter> for DeltaParameter {
    fn from(parameter: Parameter) -> Self {
        Self {
            name: parameter.name,
            current_value: parameter.value,
            previous_value: parameter.value,
        }
    }
}
impl From<DeltaParameter> for Parameter {
    fn from(parameter: DeltaParameter) -> Self {
        Parameter {
            name: parameter.name.clone(),
            value: parameter.current_value,
        }
    }
}
impl From<&DeltaParameter> for Parameter {
    fn from(parameter: &DeltaParameter) -> Self {
        Parameter {
            name: parameter.name.clone(),
            value: parameter.current_value,
        }
    }
}
impl From<&mut DeltaParameter> for Parameter {
    fn from(parameter: &mut DeltaParameter) -> Self {
        Parameter {
            name: parameter.name.clone(),
            value: parameter.current_value,
        }
    }
}
