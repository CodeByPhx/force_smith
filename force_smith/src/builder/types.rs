use crate::layout::types::{GraphTransformationFn, Position, PositionUpdateFn};

/// Marker structs of the Type-State Pattern indicating whether a graph transformation function was set or not.
pub struct NoneGraphTransformationFn;
pub struct SomeGraphTransformationFn<Vertex: Position, Edge>(
    pub GraphTransformationFn<Vertex, Edge>,
);
impl<Vertex: Position, Edge> From<GraphTransformationFn<Vertex, Edge>>
    for SomeGraphTransformationFn<Vertex, Edge>
{
    fn from(value: GraphTransformationFn<Vertex, Edge>) -> Self {
        Self(value)
    }
}

/// Marker structs of the Type-State Pattern indicating whether a position update function was set or not.
pub struct NonePositionUpdateFn;
pub struct SomePositionUpdateFn<Vertex: Position, Context>(pub PositionUpdateFn<Vertex, Context>);
impl<Vertex: Position, Context> From<PositionUpdateFn<Vertex, Context>>
    for SomePositionUpdateFn<Vertex, Context>
{
    fn from(value: PositionUpdateFn<Vertex, Context>) -> Self {
        Self(value)
    }
}
