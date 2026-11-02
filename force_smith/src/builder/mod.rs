use std::marker::PhantomData;

use crate::builder::types::{
    NoneConvergedCheckFn, NoneGraphLoadingFn, NonePositionUpdateFn, SomeConvergedCheckFn,
    SomeGraphLoadingFn, SomePositionUpdateFn,
};
use crate::prelude::*;

mod types;

pub struct LayoutBuilder<Vertex, Edge, Context, G, P, C, CtxType = Context>
where
    Context: Default,
    Vertex: AsVec2,
{
    _ctx_type: std::marker::PhantomData<CtxType>,
    graph_loading_fn: G,
    position_update_fn: P,
    converged_check_fn: C,
    forces: Vec<Force<Vertex, Edge, Context>>,
}

impl<Vertex, Edge, Context>
    LayoutBuilder<
        Vertex,
        Edge,
        Context,
        NoneGraphLoadingFn,
        NonePositionUpdateFn,
        NoneConvergedCheckFn,
    >
where
    Context: Default,
    Vertex: AsVec2,
{
    pub fn build() -> Self {
        Self {
            _ctx_type: PhantomData,
            graph_loading_fn: NoneGraphLoadingFn,
            position_update_fn: NonePositionUpdateFn,
            converged_check_fn: NoneConvergedCheckFn,
            forces: Vec::new(),
        }
    }
}

impl<Vertex, Edge, Context, G, P, C> LayoutBuilder<Vertex, Edge, Context, G, P, C>
where
    Context: Default,
    Vertex: AsVec2,
{
    pub fn with_context_type<Ctx>(self) -> LayoutBuilder<Vertex, Edge, Context, G, P, C, Ctx> {
        LayoutBuilder {
            _ctx_type: PhantomData,
            graph_loading_fn: self.graph_loading_fn,
            position_update_fn: self.position_update_fn,
            converged_check_fn: self.converged_check_fn,
            forces: self.forces,
        }
    }
}

impl<Vertex, Edge, Context, P, C> LayoutBuilder<Vertex, Edge, Context, NoneGraphLoadingFn, P, C>
where
    Context: Default,
    Vertex: AsVec2,
{
    pub fn with_graph_loading_fn(
        self,
        graph_loading_fn: GraphLoadingFn<Vertex, Edge, Context>,
    ) -> LayoutBuilder<Vertex, Edge, Context, SomeGraphLoadingFn<Vertex, Edge, Context>, P, C> {
        LayoutBuilder {
            _ctx_type: self._ctx_type,
            graph_loading_fn: graph_loading_fn.into(),
            position_update_fn: self.position_update_fn,
            converged_check_fn: self.converged_check_fn,
            forces: self.forces,
        }
    }
}

impl<Vertex, Edge, Context, G, P> LayoutBuilder<Vertex, Edge, Context, G, P, NoneConvergedCheckFn>
where
    Context: Default,
    Vertex: AsVec2,
{
    pub fn with_converged_check_fn(
        self,
        converged_check_fn: ConvergedCheckFn<Context>,
    ) -> LayoutBuilder<Vertex, Edge, Context, G, P, SomeConvergedCheckFn<Context>> {
        LayoutBuilder {
            _ctx_type: self._ctx_type,
            graph_loading_fn: self.graph_loading_fn,
            position_update_fn: self.position_update_fn,
            converged_check_fn: converged_check_fn.into(),
            forces: self.forces,
        }
    }
}

impl<Vertex, Edge, Context, G, C> LayoutBuilder<Vertex, Edge, Context, G, NonePositionUpdateFn, C>
where
    Context: Default,
    Vertex: AsVec2,
{
    pub fn with_position_update_fn(
        self,
        position_update_fn: PositionUpdateFn<Vertex, Context>,
    ) -> LayoutBuilder<Vertex, Edge, Context, G, SomePositionUpdateFn<Vertex, Context>, C> {
        LayoutBuilder {
            _ctx_type: self._ctx_type,
            graph_loading_fn: self.graph_loading_fn,
            position_update_fn: position_update_fn.into(),
            converged_check_fn: self.converged_check_fn,
            forces: self.forces,
        }
    }
}

impl<Vertex, Edge, Context, G, P, C> LayoutBuilder<Vertex, Edge, Context, G, P, C>
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
        SomeConvergedCheckFn<Context>,
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
            self.converged_check_fn.0,
        )
    }
}
