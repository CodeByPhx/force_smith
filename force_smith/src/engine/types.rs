use bevy_math::Vec2;

use crate::graph::{Graph, SpecializedGraph};

pub type GraphLoadingFn<Vertex, Edge, Context> =
    fn(graph: &Graph, ctx: &mut Context) -> SpecializedGraph<Vertex, Edge>;

#[derive(Clone)]
pub struct Displacements(pub Vec<Vec2>);
impl From<Vec<Vec2>> for Displacements {
    fn from(value: Vec<Vec2>) -> Self {
        Self(value)
    }
}
impl std::ops::Deref for Displacements {
    type Target = Vec<Vec2>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Displacements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct VertexPair<'a, Vertex> {
    pub from: &'a Vertex,
    pub to: &'a Vertex,
}
impl<'a, Vertex> From<&'a [Vertex; 2]> for VertexPair<'a, Vertex> {
    fn from(arr: &'a [Vertex; 2]) -> Self {
        Self {
            from: &arr[0],
            to: &arr[1],
        }
    }
}
pub trait ToVertexPair<'a, Vertex> {
    fn to_vertex_pair(&'a self, from: usize, to: usize) -> VertexPair<'a, Vertex>;
}
impl<'a, Vertex> ToVertexPair<'a, Vertex> for &[Vertex] {
    fn to_vertex_pair(&'a self, from: usize, to: usize) -> VertexPair<'a, Vertex> {
        VertexPair {
            from: &self[from],
            to: &self[to],
        }
    }
}

#[derive(Clone)]
pub struct Force<Vertex, Edge, Context> {
    pub force_fn: ForceFn<Vertex, Context>,
    pub applicator_fn: ApplicatorFn<Vertex, Edge, Context>,
}
impl<Vertex, Edge, Context> Force<Vertex, Edge, Context> {
    pub fn apply(
        &self,
        vertices: &[Vertex],
        edges: &[Edge],
        context: &Context,
        displacements: &mut Displacements,
    ) {
        (self.applicator_fn)(vertices, edges, context, displacements, self.force_fn);
    }
}

pub type ForceFn<Vertex, Context> = fn(vertex_pair: VertexPair<Vertex>, ctx: &Context) -> Vec2;
pub type ApplicatorFn<Vertex, Edge, Context> = fn(
    vertices: &[Vertex],
    edges: &[Edge],
    context: &Context,
    displacements: &mut Displacements,
    force_fn: ForceFn<Vertex, Context>,
);

pub type PositionUpdateFn<Vertex, Context> =
    fn(displacements: &Displacements, vertices: &mut [Vertex], context: &mut Context);
