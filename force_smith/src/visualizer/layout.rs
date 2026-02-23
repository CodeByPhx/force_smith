use crate::visualizer::{
    VisualizerStates,
    global_resources::GraphResource,
    graph_visualizer::{Destination, Index, NodeMarker},
    layout_trait::{Parameter, VisualizableDebugLayout},
};
use bevy::prelude::*;
use std::collections::HashMap;

pub struct LayoutPlugin;
impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LayoutMode::default()).add_systems(
            Update,
            (
                (
                    update_layout_config.run_if(resource_changed::<LayoutConfigResource>),
                    update_layout_graph.run_if(resource_changed::<GraphResource>),
                )
                    .before(CoreIteration),
                (
                    iterate_layout.run_if(deref_res(LayoutMode::is_run)),
                    iterate_layout_debug.run_if(deref_res(LayoutMode::is_debug_compute_forces)),
                    attach_destinations
                        .run_if(
                            deref_res(LayoutMode::is_run)
                                .or(deref_res(LayoutMode::is_debug_update_graph)),
                        )
                        .after(iterate_layout),
                )
                    .in_set(CoreIteration),
            )
                .in_set(VisualizerStates::Iteration),
        );
    }
}

#[derive(SystemSet, Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct CoreIteration;

pub fn deref_res<T, F>(fun: F) -> impl Fn(Res<T>) -> bool
where
    T: Resource,
    F: Fn(&T) -> bool + Copy,
{
    move |res: Res<T>| fun(&*res)
}

#[derive(Resource, Default)]
pub enum LayoutMode {
    Run,
    #[default]
    Stop,
    DebugStop,
    DebugComputeForces,
    DebugShowForces {
        forces: Vec<Vec<Vec2>>,
    },
    DebugStopBeforeUpdate,
    DebugUpdateGraph,
}
impl LayoutMode {
    pub fn is_normal_mode(&self) -> bool {
        matches!(self, LayoutMode::Run | LayoutMode::Stop)
    }
    pub fn is_debug_mode(&self) -> bool {
        match self {
            LayoutMode::DebugStop => true,
            LayoutMode::DebugComputeForces => true,
            LayoutMode::DebugShowForces { forces: _ } => true,
            LayoutMode::DebugStopBeforeUpdate => true,
            LayoutMode::DebugUpdateGraph => todo!(),
            _ => false,
        }
    }
    pub fn is_run(&self) -> bool {
        matches!(self, LayoutMode::Run)
    }
    pub fn is_stop(&self) -> bool {
        matches!(self, LayoutMode::Stop)
    }
    pub fn is_debug_stop(&self) -> bool {
        matches!(self, LayoutMode::DebugStop)
    }
    pub fn is_debug_compute_forces(&self) -> bool {
        matches!(self, LayoutMode::DebugComputeForces)
    }
    pub fn is_debug_show_forces(&self) -> bool {
        matches!(self, LayoutMode::DebugShowForces { forces: _ })
    }
    pub fn is_debug_stop_before_update(&self) -> bool {
        matches!(self, LayoutMode::DebugStopBeforeUpdate)
    }
    pub fn is_debug_update_graph(&self) -> bool {
        matches!(self, LayoutMode::DebugUpdateGraph)
    }
}

#[derive(Deref, DerefMut)]
pub struct LayoutResource(Box<dyn VisualizableDebugLayout>);
impl From<Box<dyn VisualizableDebugLayout>> for LayoutResource {
    fn from(value: Box<dyn VisualizableDebugLayout>) -> Self {
        Self(value)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct LayoutConfigResource(HashMap<String, Parameter>);
impl From<HashMap<String, Parameter>> for LayoutConfigResource {
    fn from(value: HashMap<String, Parameter>) -> Self {
        Self(value)
    }
}

fn update_layout_config(config: Res<LayoutConfigResource>, mut layout: NonSendMut<LayoutResource>) {
    layout.update_parameters(&config);
}

fn update_layout_graph(graph: Res<GraphResource>, mut layout: NonSendMut<LayoutResource>) {
    layout.set_graph(&graph);
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

fn iterate_layout_debug(
    mut layout: NonSendMut<LayoutResource>,
    mut layout_mode: ResMut<LayoutMode>,
) {
    let forces = layout.iterate_debug();
    *layout_mode = LayoutMode::DebugShowForces { forces };
}
