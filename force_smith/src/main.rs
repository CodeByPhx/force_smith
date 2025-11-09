use force_smith::prelude::*;
use force_smith::prelude::{LayoutBuilder, Step, Vec2};
use force_smith::utils::applicators::{linear_attraction_applicator, linear_repulsion_applicator};

#[derive(Parameterized)]
pub struct Config {
    #[parameter]
    temperature: f32,
    #[parameter]
    ideal_edge_len: f32,
    cooling_fn: fn(temperature: f32) -> f32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            temperature: 100.0,
            ideal_edge_len: 10.0,
            cooling_fn: |t| t * 0.9,
        }
    }
}

fn main() {
    let layout = LayoutBuilder::build()
        .with_context_type::<Config>()
        .with_graph_transformation_fn(|g| g.into())
        .with_step(Step {
            force_fn: |pair, ctx| {
                let direction = pair.from.direction(pair.to);
                let force = direction.length().powi(2).max(0.01) / ctx.ideal_edge_len;
                let norm_dir = direction.normalize().unwrap_or(Vec2::ZERO);
                norm_dir * force
            },
            applicator_fn: linear_attraction_applicator,
        })
        .with_step(Step {
            force_fn: |pair, ctx| {
                let direction = pair.from.direction(pair.to);
                let force = -ctx.ideal_edge_len.powi(2) / direction.length().max(0.01);
                let norm_direction = direction.normalize().unwrap_or(Vec2::ZERO);
                norm_direction * force
            },
            applicator_fn: linear_repulsion_applicator,
        })
        .with_position_update_fn(|displacements, vertices, ctx| {
            for idx in 0..vertices.len() {
                let displacement = displacements[idx];
                let clamped_displacement = displacement
                    .clamp_length(..=ctx.temperature)
                    .unwrap_or(Vec2::ZERO);
                vertices[idx] += clamped_displacement;
            }
            ctx.temperature = (ctx.cooling_fn)(ctx.temperature);
        })
        .to_layout()
        .set_default_context();
    visualize(Box::new(layout));
}
