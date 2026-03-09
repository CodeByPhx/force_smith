use crate::prelude::*;
use bevy::prelude::*;

#[derive(Deref, DerefMut)]
pub struct LayoutResource(pub Box<dyn DebugLayoutAlgorithm>);
impl From<Box<dyn DebugLayoutAlgorithm>> for LayoutResource {
    fn from(value: Box<dyn DebugLayoutAlgorithm>) -> Self {
        Self(value)
    }
}
