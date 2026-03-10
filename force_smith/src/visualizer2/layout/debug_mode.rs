use bevy::prelude::*;

use crate::visualizer2::{
    global_schedule::VisualizerStates,
    layout::{
        layout_mode::{LayoutMode, in_layout_state},
        resource::LayoutResource,
    },
};

pub struct DebugModePlugin;
impl Plugin for DebugModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                dbg_iterate_layout,
                attach_destinations.after(dbg_iterate_layout),
            )
                .run_if(in_layout_state(DebugState::Compute.into()))
                .in_set(VisualizerStates::Iteration),
        );
    }
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
    StopBeforePositions,
    RemoveForces,
    UpdatePositions,
}

fn dbg_iterate_layout(mut layout: NonSendMut<LayoutResource>, mut mode: ResMut<LayoutMode>) {
    let forces = layout.dbg_iterate();
    let destinations = layout.get_positions();
    mode.state = DebugState::ShowForces {
        forces,
        destinations,
    }
    .into();
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
