use bevy_math::Vec2;
use force_smith::builder::LayoutBuilder;
use force_smith::layout::types::{Force, VertexPair};
use force_smith::utils::applicators::{linear_attraction_applicator, linear_repulsion_applicator};
use force_smith::visualizer::visualize_debug;
use force_smith_macros::Parameterized;

#[derive(Parameterized)]
pub struct Config {
    #[parameter(name = "Hello world")]
    temperature: f32,
    #[parameter]
    ideal_edge_len: f32,
    cooling_fn: fn(temperature: f32) -> f32,
}

impl Config {
    pub fn new(
        temperature: f32,
        ideal_edge_len: f32,
        cooling_fn: fn(temperature: f32) -> f32,
    ) -> Self {
        Self {
            temperature,
            ideal_edge_len,
            cooling_fn,
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            temperature: 100.0,
            ideal_edge_len: 10.0,
            cooling_fn: |t| t * 0.99,
        }
    }
}

fn main() {
    let layout = LayoutBuilder::build()
        .with_context_type::<Config>()
        .with_graph_transformation_fn(|g| g.into())
        .with_force(Force {
            force_fn: |pair: VertexPair<Vec2>, ctx| {
                let direction = pair.to - pair.from;
                let force = direction.length().powi(2).max(0.01) / ctx.ideal_edge_len;
                let norm_dir = direction.normalize_or(Vec2::ZERO);
                norm_dir * force
            },
            applicator_fn: linear_attraction_applicator,
        })
        .with_force(Force {
            force_fn: |pair: VertexPair<Vec2>, ctx| {
                let direction = pair.to - pair.from;
                let force = -ctx.ideal_edge_len.powi(2) / direction.length().max(0.01);
                let norm_direction = direction.normalize_or(Vec2::ZERO);
                norm_direction * force
            },
            applicator_fn: linear_repulsion_applicator,
        })
        .with_position_update_fn(|displacements, vertices, ctx| {
            for idx in 0..vertices.len() {
                let displacement = displacements[idx];
                let clamped_displacement = displacement.clamp_length(0.0, ctx.temperature);
                vertices[idx] += clamped_displacement;
            }
            ctx.temperature = (ctx.cooling_fn)(ctx.temperature);
        })
        .to_layout()
        .set_default_context();
    visualize_debug(Box::new(layout));
}
