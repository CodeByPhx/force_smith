use bevy_math::Vec2;

#[derive(Default)]
#[cfg_attr(feature = "visualizer", derive(serde::Serialize, serde::Deserialize))]
pub struct Graph {
    pub vertices: usize,
    pub edges: Vec<Edge>,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "visualizer", derive(serde::Serialize, serde::Deserialize))]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

#[derive(Clone)]
pub struct SpecializedGraph<Vertex, Edge> {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
}

impl From<&Graph> for SpecializedGraph<Vec2, Edge> {
    fn from(value: &Graph) -> Self {
        use rand::RngExt;
        let mut rng = rand::rng();

        let vertices: Vec<Vec2> = (0..value.vertices)
            .map(|_| {
                let angle = rng.random_range(0.0..std::f32::consts::TAU);
                let radius = rng.random_range(50.0..200.0);
                Vec2::new(angle.cos() * radius, angle.sin() * radius)
            })
            .collect();

        Self {
            vertices,
            edges: value.edges.clone(),
        }
    }
}
