use crate::layout::types::{Displacements, ForceFn, Position, ToVertexPair};
use bevy_math::Vec2;

pub fn linear_repulsion_applicator<Vertex, Edge, Context>(
    vertices: &[Vertex],
    _: &[Edge],
    context: &Context,
    displacements: &mut Displacements,
    force_fn: ForceFn<Vertex, Context>,
) {
    for from_idx in 0..vertices.len() {
        for to_idx in from_idx + 1..vertices.len() {
            let displacement = force_fn(vertices.to_vertex_pair(from_idx, to_idx), context);
            displacements[from_idx] += displacement;
            displacements[to_idx] -= displacement;
        }
    }
}

pub trait ToIndexPair {
    fn to_index_pair(&self) -> (usize, usize);
}
impl ToIndexPair for (usize, usize) {
    fn to_index_pair(&self) -> (usize, usize) {
        *self
    }
}
pub fn linear_attraction_applicator<Vertex, Edge, Context>(
    vertices: &[Vertex],
    edges: &[Edge],
    context: &Context,
    displacements: &mut Displacements,
    force_fn: ForceFn<Vertex, Context>,
) where
    Edge: ToIndexPair,
{
    for e in edges {
        let (from_idx, to_idx) = e.to_index_pair();
        let displacement = force_fn(vertices.to_vertex_pair(from_idx, to_idx), context);
        displacements[from_idx] += displacement;
        displacements[to_idx] -= displacement;
    }
}

pub trait PositionMut {
    fn position_mut(&mut self) -> &mut Vec2;
}
impl PositionMut for Vec2 {
    fn position_mut(&mut self) -> &mut Vec2 {
        self
    }
}
pub fn linear_position_update<Vertex: Position, Context>(
    displacements: &Displacements,
    vertices: &mut [Vertex],
    _: &mut Context,
) {
    for idx in 0..vertices.len() {
        *vertices[idx].vec2_as_ref_mut() += displacements[idx];
    }
}
