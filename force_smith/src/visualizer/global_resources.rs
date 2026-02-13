use bevy::prelude::*;

use crate::layout::types::BaseGraph;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct GraphResource(pub BaseGraph);
impl From<BaseGraph> for GraphResource {
    fn from(value: BaseGraph) -> Self {
        Self(value)
    }
}

pub struct Edge {
    from: usize,
    to: usize,
}
impl From<&(usize, usize)> for Edge {
    fn from(value: &(usize, usize)) -> Self {
        Self {
            from: value.0,
            to: value.1,
        }
    }
}
