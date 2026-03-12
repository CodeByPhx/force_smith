//! Fruchterman-Reingold Layout Algorithm
//!
//! This example implements the classic Fruchterman-Reingold force-directed layout algorithm.
//! The algorithm uses:
//! - Attractive forces between connected nodes (like springs)
//! - Repulsive forces between all node pairs (like electrical charges)
//! - Temperature-based annealing for convergence
//!
//! ## Algorithm Details:
//! - Attractive force: f_a(d) = d² / k
//! - Repulsive force: f_r(d) = -k² / d
//! - Where k is the ideal spring length (sqrt(area/n))
//!
//! ## Usage:
//! ```bash
//! cargo run --example fruchterman_reingold
//! ```

use force_smith::prelude::*;

/// Configuration for the Fruchterman-Reingold algorithm
pub struct FruchtermanReingoldConfig {
    /// Temperature controls the maximum displacement per iteration
    temperature: f32,
    /// Ideal spring length between nodes
    k: f32,
    /// Cooling rate (how fast temperature decreases)
    cooling_rate: f32,
    /// Area of the layout space
    area: f32,
}

impl Default for FruchtermanReingoldConfig {
    fn default() -> Self {
        let area = 1000.0 * 1000.0; // 1000x1000 layout area
        let n = 10.0_f32; // Estimated number of nodes (will be updated)
        Self {
            temperature: 100.0,
            k: (area / n).sqrt(),
            cooling_rate: 0.95, // Multiply temperature by this each iteration
            area,
        }
    }
}

fn main() {
    // Create a simple graph for demonstration
    let graph = Graph {
        vertices: 10,
        edges: vec![
            Edge { from: 0, to: 1 },
            Edge { from: 1, to: 2 },
            Edge { from: 2, to: 3 },
            Edge { from: 3, to: 4 },
            Edge { from: 4, to: 0 },
            Edge { from: 5, to: 6 },
            Edge { from: 6, to: 7 },
            Edge { from: 7, to: 8 },
            Edge { from: 8, to: 9 },
            Edge { from: 9, to: 5 },
            Edge { from: 0, to: 5 },
        ],
    };

    // Build the layout algorithm
    let mut layout = LayoutBuilder::build()
        .with_context_type::<FruchtermanReingoldConfig>()
        .with_graph_loading_fn(|g, ctx| {
            // Update k based on actual number of vertices
            let n = g.vertices as f32;
            ctx.k = (ctx.area / n).sqrt();
            g.into()
        })
        .with_position_update_fn(|displacements, vertices, ctx| {
            // Apply displacements with temperature-based clamping
            for idx in 0..vertices.len() {
                let displacement = displacements[idx];
                let displacement_length = displacement.length();

                if displacement_length > 0.0 {
                    // Clamp displacement to temperature
                    let clamped_length = displacement_length.min(ctx.temperature);
                    let displacement = displacement.normalize() * clamped_length;
                    vertices[idx] += displacement;
                }
            }

            // Cool down the system
            ctx.temperature *= ctx.cooling_rate;
        })
        // Attractive force between connected nodes
        .with_force(Force {
            force_fn: |pair: VertexPair<Vec2>, ctx| {
                let delta = pair.to - pair.from;
                let distance = delta.length().max(0.01);
                let force_magnitude = distance * distance / ctx.k;
                delta.normalize() * force_magnitude
            },
            applicator_fn: linear_attraction_applicator,
        })
        // Repulsive force between all nodes
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

    // Load the graph
    layout.load_graph(&graph);

    // Run iterations
    println!("Running Fruchterman-Reingold layout algorithm...");
    println!("Initial positions:");
    for (idx, pos) in layout.get_positions().iter().take(3).enumerate() {
        println!("  Node {}: ({:.2}, {:.2})", idx, pos.x, pos.y);
    }

    for iteration in 0..100 {
        layout.iterate();

        // Print progress every 20 iterations
        if iteration % 20 == 0 {
            println!("\nIteration {}", iteration);
            println!("Sample positions:");
            for (idx, pos) in layout.get_positions().iter().take(3).enumerate() {
                println!("  Node {}: ({:.2}, {:.2})", idx, pos.x, pos.y);
            }
        }
    }

    println!("\nFinal positions after 100 iterations:");
    for (idx, pos) in layout.get_positions().iter().enumerate() {
        println!("  Node {}: ({:.2}, {:.2})", idx, pos.x, pos.y);
    }
}
