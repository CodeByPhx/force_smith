use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GraphResource {
    node_count: usize,
    edges: Vec<Edge>,
}

pub struct Edge {
    from: usize,
    to: usize,
}
