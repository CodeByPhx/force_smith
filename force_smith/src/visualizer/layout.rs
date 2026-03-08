use crate::visualizer::{
    VisualizerStates,
    global_resources::GraphResource,
    graph_visualizer::{Destination, Index, NodeMarker},
    layout_trait::{Parameter, VisualizableDebugLayout},
};
use bevy::prelude::*;

pub struct LayoutPlugin;
impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LayoutMode::default());
        app.add_plugins(LayoutPluginNormalMode);
        app.add_plugins(LayoutPluginDebugMode);
        app.add_systems(
            Update,
            ((
                update_layout_config.run_if(resource_changed::<LayoutConfigResource>),
                update_layout_graph.run_if(resource_changed::<GraphResource>),
            )
                .in_set(VisualizerStates::BeforeIteration),),
        );
    }
}

#[derive(Resource, Default)]
pub struct LayoutMode {
    pub state: LayoutState,
    pub mode_change: bool,
}
impl LayoutMode {
    pub fn set_mode_changed(&mut self) {
        self.mode_change = true;
    }
}
#[derive(PartialEq)]
pub enum LayoutState {
    Normal(NormalState),
    Debug(DebugState),
}
pub fn in_layout_state(expected: LayoutState) -> impl Fn(Res<LayoutMode>) -> bool {
    move |res: Res<LayoutMode>| res.state == expected
}
impl Default for LayoutState {
    fn default() -> Self {
        Self::Normal(NormalState::default())
    }
}
#[derive(Default, PartialEq)]
pub enum NormalState {
    Run,
    #[default]
    Stop,
    PlaceDestinations,
}
#[derive(Default, PartialEq)]
pub enum DebugState {
    #[default]
    Stop,
    Compute,
    ShowForces {
        forces: Vec<Vec<Vec2>>,
        destinations: Vec<Vec2>,
    },
    StopBeforeUpdate,
    RemoveForces,
    UpdatePositions,
}

pub struct LayoutPluginNormalMode;
impl Plugin for LayoutPluginNormalMode {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (iterate_layout, attach_destinations.after(iterate_layout))
                .run_if(in_layout_state(LayoutState::Normal(NormalState::Run)))
                .in_set(VisualizerStates::Iteration),
        );
    }
}

fn iterate_layout(mut layout: NonSendMut<LayoutResource>) {
    layout.iterate();
}

fn attach_destinations(
    layout: NonSend<LayoutResource>,
    nodes: Query<(&Index, Entity), With<NodeMarker>>,
    mut commands: Commands,
) {
    let positions = layout.get_positions();
    for (&Index(idx), entity) in nodes {
        commands.entity(entity).insert(Destination(positions[idx]));
    }
}

pub struct LayoutPluginDebugMode;
impl Plugin for LayoutPluginDebugMode {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                iterate_layout_debug,
                attach_destinations.after(iterate_layout_debug),
            )
                .run_if(in_layout_state(LayoutState::Debug(DebugState::Compute)))
                .in_set(VisualizerStates::Iteration),
        );
    }
}

fn iterate_layout_debug(mut layout: NonSendMut<LayoutResource>, mut mode: ResMut<LayoutMode>) {
    let forces = layout.iterate_debug();
    let destinations = layout.get_positions();
    mode.state = LayoutState::Debug(DebugState::ShowForces {
        forces,
        destinations,
    });
}

#[derive(Deref, DerefMut)]
pub struct LayoutResource(Box<dyn VisualizableDebugLayout>);
impl From<Box<dyn VisualizableDebugLayout>> for LayoutResource {
    fn from(value: Box<dyn VisualizableDebugLayout>) -> Self {
        Self(value)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct LayoutConfigResource(Vec<(String, DeltaParameter)>);
#[derive(Debug)]
pub struct DeltaParameter {
    current: Parameter,
    previous: Parameter,
}
impl DeltaParameter {
    pub fn changed(&self) -> bool {
        self.current != self.previous
    }
    pub fn uncheck_change(&mut self) {
        self.previous = self.current;
    }
}
impl std::ops::DerefMut for DeltaParameter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.current
    }
}
impl std::ops::Deref for DeltaParameter {
    type Target = Parameter;
    fn deref(&self) -> &Self::Target {
        &self.current
    }
}
impl From<Parameter> for DeltaParameter {
    fn from(value: Parameter) -> Self {
        Self {
            current: value,
            previous: value,
        }
    }
}
impl From<Vec<(String, Parameter)>> for LayoutConfigResource {
    fn from(value: Vec<(String, Parameter)>) -> Self {
        Self(
            value
                .into_iter()
                .map(|(name, value)| (name, DeltaParameter::from(value)))
                .collect(),
        )
    }
}

fn update_layout_config(
    mut config: ResMut<LayoutConfigResource>,
    mut layout: NonSendMut<LayoutResource>,
) {
    let mut changed_parameters: Vec<(String, Parameter)> = Vec::new();
    for (name, value) in config.iter_mut().filter(|(_, param)| param.changed()) {
        value.uncheck_change();
        changed_parameters.push((name.clone(), value.current.clone()));
    }
    layout.update_parameters(&changed_parameters);
}

fn update_layout_graph(graph: Res<GraphResource>, mut layout: NonSendMut<LayoutResource>) {
    layout.set_graph(&graph);
}
