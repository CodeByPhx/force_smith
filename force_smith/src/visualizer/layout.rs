use crate::visualizer::{
    VisualizerStates,
    global_resources::GraphResource,
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
                    iterate_layout.run_if(LayoutMode::is_run),
                    iterate_layout_debug.run_if(LayoutMode::is_debug),
                )
                    .in_set(CoreIteration),
            )
                .in_set(VisualizerStates::Iteration),
        );
    }
}

#[derive(SystemSet, Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct CoreIteration;

#[derive(Resource, Default)]
pub enum LayoutMode {
    Run,
    #[default]
    Stop,
    Debug,
}
impl LayoutMode {
    fn is_run(mode: Res<LayoutMode>) -> bool {
        matches!(*mode, LayoutMode::Run)
    }
    fn is_stop(mode: Res<LayoutMode>) -> bool {
        matches!(*mode, LayoutMode::Stop)
    }
    fn is_debug(mode: Res<LayoutMode>) -> bool {
        matches!(*mode, LayoutMode::Debug)
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

fn update_layout_graph() {
    todo!()
}

fn iterate_layout(mut layout: NonSendMut<LayoutResource>) {
    layout.iterate();
    todo!("Send messages");
}

fn iterate_layout_debug(
    mut layout: NonSendMut<LayoutResource>,
    mut layout_mode: ResMut<LayoutMode>,
) {
    *layout_mode = LayoutMode::Stop;
    todo!("Iterate debug and send messages");
}
