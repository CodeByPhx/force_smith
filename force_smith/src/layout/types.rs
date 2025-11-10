use crate::utils::vec2::Vec2;

/// A collection of layout steps executed in sequence during simulation.
///
/// Each [`Step`] defines a single force and its application behavior.
/// This wrapper provides deref access to the underlying `Vec`.
pub struct Steps<Vertex, Edge, Context>(pub Vec<Step<Vertex, Edge, Context>>);
impl<Vertex, Edge, Context> std::ops::Deref for Steps<Vertex, Edge, Context> {
    type Target = Vec<Step<Vertex, Edge, Context>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<Vertex, Edge, Context> std::ops::DerefMut for Steps<Vertex, Edge, Context> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<Vertex, Edge, Context> From<Vec<Step<Vertex, Edge, Context>>>
    for Steps<Vertex, Edge, Context>
{
    fn from(value: Vec<Step<Vertex, Edge, Context>>) -> Self {
        Self(value)
    }
}

/// A single step in the layout simulation.
///
/// Each step defines:
/// - A **force function** that computes directional displacement between vertices.
/// - An **applicator function** that applies those displacements.
pub struct Step<Vertex, Edge, Context> {
    pub force_fn: ForceFn<Vertex, Context>,
    pub applicator_fn: ApplicatorFn<Vertex, Edge, Context>,
}
impl<Vertex, Edge, Context> Step<Vertex, Edge, Context> {
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
impl<Vertex, Edge, Context>
    From<(
        ApplicatorFn<Vertex, Edge, Context>,
        ForceFn<Vertex, Context>,
    )> for Step<Vertex, Edge, Context>
{
    fn from(
        value: (
            ApplicatorFn<Vertex, Edge, Context>,
            ForceFn<Vertex, Context>,
        ),
    ) -> Self {
        Self {
            applicator_fn: value.0,
            force_fn: value.1,
        }
    }
}

/// Stores intermediate displacement vectors computed during layout iteration.
#[derive(Default)]
pub struct Displacements(pub Vec<Vec2>);
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
impl Displacements {
    pub fn fill_zero_sized(len: usize) -> Self {
        Self(vec![Vec2::ZERO; len])
    }
}
impl From<Vec<Vec2>> for Displacements {
    fn from(value: Vec<Vec2>) -> Self {
        Self(value)
    }
}

/// The graph structure specialized for simulation.
///
/// This represents the transformed form of the [`BaseGraph`] used internally
/// during layout computations.
pub struct SpecializedGraph<Vertex, Edge> {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
}
impl<Vertex, Edge> Default for SpecializedGraph<Vertex, Edge> {
    fn default() -> Self {
        Self {
            vertices: Vec::default(),
            edges: Vec::default(),
        }
    }
}
impl From<&BaseGraph> for SpecializedGraph<Vec2, (usize, usize)> {
    fn from(value: &BaseGraph) -> Self {
        SpecializedGraph {
            vertices: value.vertices.clone(),
            edges: value.edges.clone(),
        }
    }
}

/// Trait implemented by vertex types that can return a position vector.
pub trait Position {
    fn position(&self) -> Vec2;
}

/// The raw graph input used before transformation.
///
/// This is the structure passed to the layout before any specialization.
#[cfg_attr(feature = "visualizer", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default)]
pub struct BaseGraph {
    pub vertices: Vec<Vec2>,
    pub edges: Vec<(usize, usize)>,
}
impl BaseGraph {
    pub fn mock() -> Self {
        Self {
            vertices: vec![Vec2::ZERO, Vec2::ONE, Vec2::splat(2.0)],
            edges: vec![(0, 1), (1, 2)],
        }
    }
}

/// Represents a pair of vertices.
///
/// Typically passed to force functions for computing directional forces.
pub struct VertexPair<'a, Vertex> {
    pub from: &'a Vertex,
    pub to: &'a Vertex,
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

/// Marker structs of the Type-State Pattern indicating whether a context instance was set or not.
pub struct NoneContext;
pub struct SomeContext<Context>(pub Context);
impl<Context> std::ops::Deref for SomeContext<Context> {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<Context> std::ops::DerefMut for SomeContext<Context> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<Context> From<Context> for SomeContext<Context> {
    fn from(value: Context) -> Self {
        Self(value)
    }
}

/// Marker structs of the Type-State Pattern indicating whether a graph instance was set or not.
pub struct NoneGraph;
pub struct SomeGraph<Vertex, Edge>(pub SpecializedGraph<Vertex, Edge>);
impl<Vertex, Edge> std::ops::Deref for SomeGraph<Vertex, Edge> {
    type Target = SpecializedGraph<Vertex, Edge>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<Vertex, Edge> std::ops::DerefMut for SomeGraph<Vertex, Edge> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<Vertex, Edge> From<SpecializedGraph<Vertex, Edge>> for SomeGraph<Vertex, Edge> {
    fn from(value: SpecializedGraph<Vertex, Edge>) -> Self {
        SomeGraph(value)
    }
}

///// Function type aliases used for dependency injection within the layout system.
/// Transforms a base graph into a specialized graph for the layout.
pub type GraphTransformationFn<Vertex, Edge> =
    fn(graph: &BaseGraph) -> SpecializedGraph<Vertex, Edge>;

/// Computes the directional force vector between a pair of vertices.
pub type ForceFn<Vertex, Context> = fn(vertex_pair: VertexPair<Vertex>, context: &Context) -> Vec2;

/// Applies computed forces.
pub type ApplicatorFn<Vertex, Edge, Context> = fn(
    vertices: &[Vertex],
    edges: &[Edge],
    context: &Context,
    displacements: &mut Displacements,
    force_fn: ForceFn<Vertex, Context>,
);

/// Updates vertex positions based on accumulated displacements after a simulation step.
pub type PositionUpdateFn<Vertex, Context> =
    fn(displacements: &Displacements, vertices: &mut [Vertex], context: &mut Context);
