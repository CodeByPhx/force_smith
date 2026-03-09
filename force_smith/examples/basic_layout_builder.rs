use force_smith::prelude::*;

pub struct Config {
    temperature: f32,
    ideal_edge_len: f32,
    cooling_fn: fn(temperature: f32) -> f32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            temperature: 100.0,
            ideal_edge_len: 100.0,
            cooling_fn: |t| if t - 1.0 > 0.0 { t - 1.0 } else { 0.0 },
        }
    }
}

fn main() {
    let mut layout = LayoutBuilder::build()
        .with_context_type::<Config>()
        .with_graph_loading_fn(|g, _| g.into())
        .with_position_update_fn(|displacements, vertices, ctx| {
            for idx in 0..vertices.len() {
                let displacement = displacements[idx];
                let clamped_displacement = displacement.clamp_length(0.0, ctx.temperature);
                vertices[idx] += clamped_displacement;
            }
            ctx.temperature = (ctx.cooling_fn)(ctx.temperature);
        })
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
        .to_layout();
    layout.iterate();
}
