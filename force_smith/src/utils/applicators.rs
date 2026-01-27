use std::ops::AddAssign;

use crate::{
    layout::types::{Displacements, ForceFn, ToVertexPair},
    utils::vec2::Vec2,
};

pub fn linear_repulsion_applicator<Vertex, Edge, Context>(
    vertices: &[Vertex],
    _: &[Edge],
    context: &Context,
    displacements: &mut Displacements,
    force_fn: ForceFn<Vertex, Context>,
) {
    for from_idx in 0..vertices.len() {
        for to_idx in from_idx + 1..vertices.len() {
            let displacement = force_fn(vertices.to_vertex_pair(from_idx, to_idx), context);
            displacements[from_idx] += displacement;
            displacements[to_idx] -= displacement;
        }
    }
}

pub trait ToIndexPair {
    fn to_index_pair(&self) -> (usize, usize);
}
impl ToIndexPair for (usize, usize) {
    fn to_index_pair(&self) -> (usize, usize) {
        *self
    }
}
pub fn linear_attraction_applicator<Vertex, Edge, Context>(
    vertices: &[Vertex],
    edges: &[Edge],
    context: &Context,
    displacements: &mut Displacements,
    force_fn: ForceFn<Vertex, Context>,
) where
    Edge: ToIndexPair,
{
    for e in edges {
        let (from_idx, to_idx) = e.to_index_pair();
        let displacement = force_fn(vertices.to_vertex_pair(from_idx, to_idx), context);
        displacements[from_idx] += displacement;
        displacements[to_idx] -= displacement;
    }
}

pub trait PositionMut {
    fn position_mut(&mut self) -> &mut Vec2;
}
impl PositionMut for Vec2 {
    fn position_mut(&mut self) -> &mut Vec2 {
        self
    }
}
pub fn linear_position_update<Vertex, Context>(
    displacements: &Displacements,
    vertices: &mut [Vertex],
    _: &mut Context,
) where
    Vertex: PositionMut,
{
    for idx in 0..vertices.len() {
        vertices[idx].position_mut().add_assign(displacements[idx]);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::LayoutBuilder,
        layout::{
            LayoutAlgorithm,
            types::{BaseGraph, Force, VertexPair},
        },
        utils::{
            applicators::{
                linear_attraction_applicator, linear_position_update, linear_repulsion_applicator,
            },
            vec2::Vec2,
        },
    };

    #[test]
    fn linear_repulsion_applicator_works() {
        let layout = LayoutBuilder::build()
            .with_context_type::<()>()
            .with_graph_transformation_fn(|g| g.into())
            .with_force(Force {
                force_fn: |pair: VertexPair<Vec2>, _| {
                    pair.from.direction(pair.to) * pair.from.distance(pair.to)
                },
                applicator_fn: linear_repulsion_applicator,
            })
            .with_position_update_fn(linear_position_update)
            .to_layout();

        let base_graph = BaseGraph {
            vertices: vec![Vec2::from_xy(0.0, 0.0), Vec2::from_xy(1.0, 0.0)],
            edges: vec![], // repulsion ignores edges
        };

        let mut layout = layout.set_default_context();
        layout.set_graph(&base_graph);
        layout.iterate();

        let result = layout.get_positions();

        assert_eq!(result[0], Vec2::from_xy(1.0, 0.0));
        assert_eq!(result[1], Vec2::from_xy(0.0, 0.0));
    }

    #[test]
    fn linear_attraction_applicator_works() {
        let layout = LayoutBuilder::build()
            .with_context_type::<()>()
            .with_graph_transformation_fn(|g| g.into())
            .with_force(Force {
                force_fn: |pair: VertexPair<Vec2>, _| {
                    pair.from.direction(pair.to) * pair.from.distance(pair.to)
                },
                applicator_fn: linear_attraction_applicator,
            })
            .with_position_update_fn(linear_position_update)
            .to_layout();

        let base_graph = BaseGraph {
            vertices: vec![
                Vec2::from_xy(0.0, 0.0),
                Vec2::from_xy(1.0, 0.0),
                Vec2::from_xy(2.0, 0.0),
            ],
            edges: vec![(0, 1)],
        };

        let mut layout = layout.set_default_context();
        layout.set_graph(&base_graph);
        layout.iterate();

        let result = layout.get_positions();

        assert_eq!(result[0], Vec2::from_xy(1.0, 0.0));
        assert_eq!(result[1], Vec2::from_xy(0.0, 0.0));
        assert_eq!(result[2], Vec2::from_xy(2.0, 0.0));
    }
}
