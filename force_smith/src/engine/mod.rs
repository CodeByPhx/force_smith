use bevy_math::Vec2;

use crate::graph::{Edge, Graph};

pub mod layout_engine;
pub mod types;

pub trait LayoutAlgorithm {
    fn load_graph(&mut self, graph: &Graph);
    fn iterate(&mut self);
    fn get_positions(&self) -> Vec<Vec2>;
    fn get_edges(&self) -> Vec<Edge>;
}

pub trait AsVec2 {
    fn as_ref_vec2(&self) -> &Vec2;
    fn as_ref_mut_vec2(&mut self) -> &mut Vec2;
    fn as_copy_vec2(&self) -> Vec2;
}
impl AsVec2 for Vec2 {
    fn as_ref_vec2(&self) -> &Vec2 {
        self
    }

    fn as_ref_mut_vec2(&mut self) -> &mut Vec2 {
        self
    }

    fn as_copy_vec2(&self) -> Vec2 {
        *self
    }
}

pub trait AsEdge {
    fn as_edge(&self) -> Edge;
}
impl AsEdge for Edge {
    fn as_edge(&self) -> Edge {
        *self
    }
}
