use crate::engine::types::{GraphLoadingFn, PositionUpdateFn};

pub struct NoneGraphLoadingFn;
pub struct SomeGraphLoadingFn<Vertex, Edge, Context>(pub GraphLoadingFn<Vertex, Edge, Context>);
impl<Vertex, Edge, Context> From<GraphLoadingFn<Vertex, Edge, Context>>
    for SomeGraphLoadingFn<Vertex, Edge, Context>
{
    fn from(value: GraphLoadingFn<Vertex, Edge, Context>) -> Self {
        Self(value)
    }
}

pub struct NonePositionUpdateFn;
pub struct SomePositionUpdateFn<Vertex, Context>(pub PositionUpdateFn<Vertex, Context>);
impl<Vertex, Context> From<PositionUpdateFn<Vertex, Context>>
    for SomePositionUpdateFn<Vertex, Context>
{
    fn from(value: PositionUpdateFn<Vertex, Context>) -> Self {
        Self(value)
    }
}
