use crate::prelude::*;

pub trait DebugLayoutAlgorithm: LayoutAlgorithm {
    fn iterate_dbg(&mut self) -> Vec<Vec<Vec2>>;
}
