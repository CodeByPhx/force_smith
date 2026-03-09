use bevy_math::Vec2;

#[derive(Default)]
pub struct Graph {
    pub vertices: usize,
    pub edges: Vec<Edge>,
}

#[derive(Clone, Copy)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

pub struct SpecializedGraph<Vertex, Edge> {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
}
impl From<&Graph> for SpecializedGraph<Vec2, Edge> {
    fn from(value: &Graph) -> Self {
        Self {
            vertices: vec![Vec2::ZERO; value.vertices],
            edges: value.edges.clone(),
        }
    }
}
