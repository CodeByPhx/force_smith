use bevy::prelude::*;

use crate::visualizer2::{
    global_schedule::VisualizerStates,
    layout::{layout_mode::in_layout_state, resource::LayoutResource},
};

pub struct NormalModePlugin;
impl Plugin for NormalModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (iterate_layout, attach_destinations.after(iterate_layout))
                .run_if(in_layout_state(NormalState::Run.into()))
                .in_set(VisualizerStates::Iteration),
        );
    }
}

#[derive(Default, PartialEq)]
pub enum NormalState {
    #[default]
    Stop,
    Run,
}

fn iterate_layout(mut layout: NonSendMut<LayoutResource>) {
    layout.iterate();
}

fn attach_destinations(// layout: NonSend<LayoutResource>,
    // nodes: Query<(&Index, Entity), With<NodeMarker>>,
    // mut commands: Commands,
) {
    // let positions = layout.get_positions();
    // for (&Index(idx), entity) in nodes {
    //     commands.entity(entity).insert(Destination(positions[idx]));
    // }
}
