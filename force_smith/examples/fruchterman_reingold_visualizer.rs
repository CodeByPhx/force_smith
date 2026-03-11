//! Fruchterman-Reingold with Interactive Visualizer
//!
//! This example demonstrates the Fruchterman-Reingold algorithm with the interactive visualizer.
//! You can adjust parameters in real-time and see the algorithm's behavior.
//!
//! ## Features:
//! - Real-time parameter tuning (k, temperature, cooling rate)
//! - Debug mode to see force vectors
//! - Normal mode for continuous execution
//!
//! ## Usage:
//! ```bash
//! cargo run --example fruchterman_reingold_visualizer --features visualizer
//! ```

#[cfg(feature = "visualizer")]
use force_smith::prelude::*;

#[cfg(feature = "visualizer")]
#[derive(Parameterized)]
pub struct FruchtermanReingoldConfig {
    #[parameter(name = "Temperature")]
    temperature: f32,
    #[parameter(name = "Spring Constant (k)")]
    k: f32,
    #[parameter(name = "Cooling Rate")]
    cooling_rate: f32,
}

#[cfg(feature = "visualizer")]
impl Default for FruchtermanReingoldConfig {
    fn default() -> Self {
        Self {
            temperature: 100.0,
            k: 50.0,
            cooling_rate: 0.98,
        }
    }
}

#[cfg(feature = "visualizer")]
fn main() {
    let layout = LayoutBuilder::build()
        .with_context_type::<FruchtermanReingoldConfig>()
        .with_graph_loading_fn(|g, _| g.into())
        .with_position_update_fn(|displacements, vertices, ctx| {
            for idx in 0..vertices.len() {
                let displacement = displacements[idx];
                let displacement_length = displacement.length();

                if displacement_length > 0.0 {
                    let clamped_length = displacement_length.min(ctx.temperature);
                    let displacement = displacement.normalize() * clamped_length;
                    vertices[idx] += displacement;
                }
            }
            ctx.temperature *= ctx.cooling_rate;
        })
        // Attractive force (Fruchterman-Reingold)
        .with_force(Force {
            force_fn: |pair: VertexPair<Vec2>, ctx| {
                let delta = pair.to - pair.from;
                let distance = delta.length().max(0.01);
                let force_magnitude = distance * distance / ctx.k;
                delta.normalize() * force_magnitude
            },
            applicator_fn: linear_attraction_applicator,
        })
        // Repulsive force (Fruchterman-Reingold)
        .with_force(Force {
            force_fn: |pair: VertexPair<Vec2>, ctx| {
                let delta = pair.to - pair.from;
                let distance = delta.length().max(0.01);
                let force_magnitude = -(ctx.k * ctx.k) / distance;
                delta.normalize() * force_magnitude
            },
            applicator_fn: linear_repulsion_applicator,
        })
        .to_layout();

    // Customize visualizer appearance
    let config = VisualizerConfiguration {
        background_color: GlobalColor::DarkGrey,
        node_radius: 12.0,
        node_color: GlobalColor::Cyan,
        edge_color: GlobalColor::LightGrey,
        initial_mode: VisualizerMode::Normal,
        ..Default::default()
    };

    visualize_dbg(Box::from(layout), config);
}

#[cfg(not(feature = "visualizer"))]
fn main() {
    panic!(
        "This example requires the 'visualizer' feature. Run with: \
        cargo run --example fruchterman_reingold_visualizer --features visualizer"
    );
}
