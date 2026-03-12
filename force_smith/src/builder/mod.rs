use std::marker::PhantomData;

use crate::builder::types::{
    NoneGraphLoadingFn, NonePositionUpdateFn, SomeGraphLoadingFn, SomePositionUpdateFn,
};
use crate::prelude::*;

mod types;

pub struct LayoutBuilder<Vertex, Edge, Context, G, P, CtxType = Context>
where
    Context: Default,
    Vertex: AsVec2,
{
    _ctx_type: std::marker::PhantomData<CtxType>,
    graph_loading_fn: G,
    position_update_fn: P,
    forces: Vec<Force<Vertex, Edge, Context>>,
}

impl<Vertex, Edge, Context>
    LayoutBuilder<Vertex, Edge, Context, NoneGraphLoadingFn, NonePositionUpdateFn>
where
    Context: Default,
    Vertex: AsVec2,
{
    pub fn build() -> Self {
        Self {
            _ctx_type: PhantomData,
            graph_loading_fn: NoneGraphLoadingFn,
            position_update_fn: NonePositionUpdateFn,
            forces: Vec::new(),
        }
    }
}

impl<Vertex, Edge, Context, G, P> LayoutBuilder<Vertex, Edge, Context, G, P>
where
    Context: Default,
    Vertex: AsVec2,
{
    pub fn with_context_type<Ctx>(self) -> LayoutBuilder<Vertex, Edge, Context, G, P, Ctx> {
        LayoutBuilder {
            _ctx_type: PhantomData,
            graph_loading_fn: self.graph_loading_fn,
            position_update_fn: self.position_update_fn,
            forces: self.forces,
        }
    }
}

impl<Vertex, Edge, Context, P> LayoutBuilder<Vertex, Edge, Context, NoneGraphLoadingFn, P>
where
    Context: Default,
    Vertex: AsVec2,
{
    pub fn with_graph_loading_fn(
        self,
        graph_loading_fn: GraphLoadingFn<Vertex, Edge, Context>,
    ) -> LayoutBuilder<Vertex, Edge, Context, SomeGraphLoadingFn<Vertex, Edge, Context>, P> {
        LayoutBuilder {
            _ctx_type: self._ctx_type,
            graph_loading_fn: graph_loading_fn.into(),
            position_update_fn: self.position_update_fn,
            forces: self.forces,
        }
    }
}

impl<Vertex, Edge, Context, G> LayoutBuilder<Vertex, Edge, Context, G, NonePositionUpdateFn>
where
    Context: Default,
    Vertex: AsVec2,
{
    pub fn with_position_update_fn(
        self,
        position_update_fn: PositionUpdateFn<Vertex, Context>,
    ) -> LayoutBuilder<Vertex, Edge, Context, G, SomePositionUpdateFn<Vertex, Context>> {
        LayoutBuilder {
            _ctx_type: self._ctx_type,
            graph_loading_fn: self.graph_loading_fn,
            position_update_fn: position_update_fn.into(),
            forces: self.forces,
        }
    }
}

impl<Vertex, Edge, Context, G, P> LayoutBuilder<Vertex, Edge, Context, G, P>
where
    Context: Default,
    Vertex: AsVec2,
{
    pub fn with_force(mut self, force: Force<Vertex, Edge, Context>) -> Self {
        self.forces.push(force);
        self
    }
}

impl<Vertex, Edge, Context>
    LayoutBuilder<
        Vertex,
        Edge,
        Context,
        SomeGraphLoadingFn<Vertex, Edge, Context>,
        SomePositionUpdateFn<Vertex, Context>,
    >
where
    Context: Default,
    Vertex: AsVec2,
{
    pub fn to_layout(self) -> LayoutEngine<Vertex, Edge, Context> {
        LayoutEngine::new(
            self.graph_loading_fn.0,
            self.forces,
            self.position_update_fn.0,
        )
    }
}
