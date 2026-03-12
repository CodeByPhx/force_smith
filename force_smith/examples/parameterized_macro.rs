//! Parameterized Derive Macro Example
//!
//! This example demonstrates how to use the `#[derive(Parameterized)]` macro
//! to automatically expose configuration parameters in the visualizer UI.
//!
//! The macro supports:
//! - f32: Displayed as sliders in the UI
//! - i32: Displayed as integer sliders
//! - bool: Displayed as checkboxes
//!
//! ## Usage:
//! ```bash
//! cargo run --example parameterized_macro --features visualizer
//! ```

#[cfg(feature = "visualizer")]
use force_smith::prelude::*;

#[cfg(feature = "visualizer")]
/// Configuration using the Parameterized derive macro
///
/// The macro automatically generates:
/// - `get_parameters()` method to read current values
/// - `update_parameters()` method to apply UI changes
///
/// This allows the visualizer to expose these parameters for real-time tuning.
#[derive(Parameterized)]
pub struct SpringConfig {
    /// Controls the maximum displacement per iteration
    #[parameter(name = "Temperature")]
    temperature: f32,

    /// Ideal spring length between connected nodes
    #[parameter(name = "Spring Length")]
    spring_length: f32,

    /// Spring stiffness constant
    #[parameter(name = "Spring Stiffness")]
    spring_stiffness: f32,

    /// How fast the system cools down (0.0 - 1.0)
    #[parameter(name = "Cooling Rate")]
    cooling_rate: f32,

    /// Repulsion strength between all nodes
    #[parameter(name = "Repulsion Strength")]
    repulsion_strength: f32,

    /// Number of iterations before next cooldown
    #[parameter(name = "Iterations Per Cool")]
    iterations_per_cool: i32,

    /// Enable gravity pulling nodes toward center
    #[parameter(name = "Enable Gravity")]
    gravity_enabled: bool,

    /// Gravity strength (only if enabled)
    #[parameter(name = "Gravity Strength")]
    gravity_strength: f32,
}

#[cfg(feature = "visualizer")]
impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            temperature: 100.0,
            spring_length: 80.0,
            spring_stiffness: 0.1,
            cooling_rate: 0.98,
            repulsion_strength: 5000.0,
            iterations_per_cool: 1,
            gravity_enabled: false,
            gravity_strength: 0.1,
        }
    }
}

#[cfg(feature = "visualizer")]
fn main() {
    println!("Parameterized Macro Example");
    println!("============================\n");
    println!("This example shows how the #[derive(Parameterized)] macro");
    println!("automatically exposes your configuration in the visualizer UI.\n");
    println!("Try adjusting the parameters in the 'Config' panel:");
    println!("  - Temperature: Controls node movement speed");
    println!("  - Spring Length: Target distance between connected nodes");
    println!("  - Spring Stiffness: How strongly springs pull nodes together");
    println!("  - Cooling Rate: Annealing speed (close to 1.0 = slow cooling)");
    println!("  - Repulsion Strength: How strongly nodes push each other apart");
    println!("  - Enable Gravity: Toggle gravity toward center");
    println!("  - Gravity Strength: Strength of gravity force\n");

    let layout = LayoutBuilder::build()
        .with_context_type::<SpringConfig>()
        .with_graph_loading_fn(|g, _| g.into())
        .with_position_update_fn(|displacements, vertices, ctx| {
            // Apply displacements with temperature limiting
            for idx in 0..vertices.len() {
                let mut displacement = displacements[idx];

                // Apply gravity toward center if enabled
                if ctx.gravity_enabled {
                    let to_center = -vertices[idx];
                    displacement += to_center * ctx.gravity_strength;
                }

                // Clamp by temperature
                let length = displacement.length();
                if length > ctx.temperature {
                    displacement = displacement.normalize() * ctx.temperature;
                }

                vertices[idx] += displacement;
            }

            // Cool down system based on iteration count
            if ctx.iterations_per_cool > 0 {
                static mut ITERATION_COUNT: i32 = 0;
                unsafe {
                    ITERATION_COUNT += 1;
                    if ITERATION_COUNT >= ctx.iterations_per_cool {
                        ctx.temperature *= ctx.cooling_rate;
                        ITERATION_COUNT = 0;
                    }
                }
            }
        })
        // Spring forces between connected nodes
        .with_force(Force {
            force_fn: |pair: VertexPair<Vec2>, ctx| {
                let delta = pair.to - pair.from;
                let distance = delta.length().max(0.01);

                // Spring force: F = k * (distance - spring_length)
                let displacement = distance - ctx.spring_length;
                let force_magnitude = displacement * ctx.spring_stiffness;

                delta.normalize() * force_magnitude
            },
            applicator_fn: linear_attraction_applicator,
        })
        // Repulsive forces between all nodes
        .with_force(Force {
            force_fn: |pair: VertexPair<Vec2>, ctx| {
                let delta = pair.to - pair.from;
                let distance = delta.length().max(0.01);

                // Repulsion force: F = -k / distance²
                let force_magnitude = -(ctx.repulsion_strength / distance.powi(2));

                delta.normalize() * force_magnitude
            },
            applicator_fn: linear_repulsion_applicator,
        })
        .to_layout();

    // Customize visualizer
    let config = VisualizerConfiguration {
        background_color: GlobalColor::DarkGrey,
        node_radius: 10.0,
        node_color: GlobalColor::Orange,
        edge_color: GlobalColor::LightGrey,
        edge_width: 2.0,
        initial_mode: VisualizerMode::Normal,
        smooth_movement_enabled: true,
        ..Default::default()
    };

    visualize_dbg(Box::from(layout), config);
}

#[cfg(not(feature = "visualizer"))]
fn main() {
    panic!(
        "This example requires the 'visualizer' feature. Run with: \
        cargo run --example parameterized_macro --features visualizer"
    );
}
