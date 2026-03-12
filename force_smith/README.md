# Force Smith

A Rust toolkit for implementing and developing force-directed layout algorithms with built-in debugging and visualization capabilities.

## Features

- **🔧 Builder API**: Ergonomic API for composing force-directed layout algorithms from modular components
- **🎨 Interactive Visualizer**: Real-time visualization with debug mode for step-by-step algorithm execution
- **⚡ High Performance**: Built on Bevy ECS for efficient graph processing
- **🔍 Debug Tools**: Inspect forces, visualize node movements, and tune parameters in real-time
- **📦 Modular Design**: Mix and match force applicators, constraints, and layout strategies

## Quick Start

### Installation

Add Force Smith to your `Cargo.toml`:

```toml
[dependencies]
force_smith = "1.0.1"

# For full features
force_smith = { version = "1.0.1", features = ["full"] }
```

### Basic Usage

```rust
use force_smith::prelude::*;

// Build a layout algorithm
let layout = LayoutBuilder::new()
    .add_force(linear_attraction_applicator)
    .add_force(linear_repulsion_applicator)
    .with_parameter("temperature", 1.0)
    .with_parameter("ideal_edge_length", 100.0)
    .with_parameter("cooling_rate", 0.05)
    .build();

// Run with visualization
visualize_dbg(
    layout,
    VisualizerConfiguration::default()
);
```

## Features

### Core Features (always enabled)
- `builder`: Layout builder API for composing algorithms
- `utils`: Utilities including force applicators and helpers

### Optional Features
- `visualizer`: Interactive visualization and debugging tools (requires Bevy)

## Examples

Force Smith includes three comprehensive examples demonstrating different use cases:

### 1. Fruchterman-Reingold Algorithm

A complete implementation of the classic Fruchterman-Reingold force-directed layout algorithm.

```bash
cargo run --example fruchterman_reingold
```

**Features:**
- Attractive forces: f_a(d) = d² / k
- Repulsive forces: f_r(d) = -k² / d
- Temperature-based annealing
- Prints iteration progress and final positions

### 2. Fruchterman-Reingold with Visualizer

Interactive version with real-time parameter tuning.

```bash
cargo run --example fruchterman_reingold_visualizer --features visualizer
```

**Features:**
- Adjust spring constant (k), temperature, and cooling rate in real-time
- Switch between Normal and Debug modes
- Visualize force vectors in debug mode
- Load custom graphs or generate random ones

### 3. Parameterized Derive Macro

Comprehensive demonstration of the `#[derive(Parameterized)]` macro.

```bash
cargo run --example parameterized_macro --features visualizer
```

**Features:**
- Shows all supported parameter types: `f32`, `i32`, `bool`
- Demonstrates real-time parameter tuning in the UI
- Includes gravity toggle and iteration control
- 8 different tunable parameters
- Custom spring-based physics model

**What you'll learn:**
- How to use the Parameterized derive macro
- How to name parameters for the UI
- How different parameter types appear in the UI (sliders vs checkboxes)
- How to handle boolean flags that affect behavior

### Example Graphs

The `graphs/` directory contains sample graph files:
- `simple_graph.json` - 5-node pentagon (good starter)
- `line_graph.json` - 10-node chain
- `tree_graph.json` - 7-node hierarchical structure
- `star_graph.json` - 8-node hub structure
- `dense_graph.json` - 6-node dense graph

You can load these in the visualizer examples through the Graph Configuration UI.

## Visualizer Usage

### Loading a Graph

**From File:**
1. Select "From File" in the Graph Configuration window
2. Choose a graph from the `graphs/` directory
3. Click "Apply"

**Generate Random:**
1. Select "From Config" in the Graph Configuration window
2. Set number of vertices and edges
3. Check "Connected" to ensure graph connectivity
4. Click "Apply"

### Layout Controls

**Normal Mode (Continuous):**
- Click "▶ Run" to start continuous layout computation
- Click "⏹ Stop" to halt execution
- Nodes move smoothly toward their destination positions

**Debug Mode (Step-by-Step):**
1. Switch to "Debug Mode"
2. Click "⏭ Step" next to "Compute Forces"
   - Red arrows show individual force vectors
   - Green arrow shows the destination position
3. Click "⏭ Step" next to "Update Positions" to apply forces
4. Repeat to step through algorithm iterations

### Parameter Configuration

Adjust these parameters in real-time while the layout runs:
- **Temperature**: Controls node movement speed (higher = faster)
- **Ideal Edge Length**: Target distance between connected nodes
- **Cooling Rate**: How fast the system stabilizes (higher = faster convergence)

### UI Controls

- **Smooth Movement**: Toggle smooth interpolation vs instant node positioning
- Hover over UI elements for tooltips with detailed information

### Camera Controls

**Keyboard (Vim-style):**
- `h` - Move left
- `j` - Move down
- `k` - Move up
- `l` - Move right
- Arrow keys also work as an alternative
- `+` or `=` - Zoom in
- `-` - Zoom out

**Mouse/Touchpad:**
- **Scroll wheel** - Zoom in/out
- **Left-click + drag** - Pan around the scene

The camera movement speed scales with the current zoom level for smooth navigation at any scale.

## Graph File Format

Create custom graphs as JSON files:

```json
{
  "vertices": 5,
  "edges": [
    {"from": 0, "to": 1},
    {"from": 1, "to": 2},
    {"from": 2, "to": 3},
    {"from": 3, "to": 4},
    {"from": 4, "to": 0}
  ]
}
```

- `vertices`: Number of nodes in the graph
- `edges`: Array of connections (indices are 0-based)

## Architecture

Force Smith is organized into several modules:

- **`builder`**: Fluent API for constructing layout algorithms
- **`engine`**: Core layout engine and iteration logic
- **`graph`**: Graph data structures and utilities
- **`utils`**: Utility functions and helpers
  - **`forces`**: Force applicators (attraction, repulsion, etc.)
- **`visualizer`**: Bevy-based interactive visualization (optional)
  - **`camera`**: Camera controls and scene setup
  - **`interface`**: UI panels and user controls
  - **`rendering`**: Visual representation of graphs
  - **`simulation`**: Layout computation and execution modes

## API Overview

### Implementing Custom Algorithms

The builder pattern makes it easy to create custom force-directed algorithms:

```rust
use force_smith::prelude::*;

// Define your algorithm's configuration
struct MyAlgorithmConfig {
    temperature: f32,
    spring_constant: f32,
}

impl Default for MyAlgorithmConfig {
    fn default() -> Self {
        Self {
            temperature: 100.0,
            spring_constant: 50.0,
        }
    }
}

// Build your layout algorithm
let mut layout = LayoutBuilder::build()
    .with_context_type::<MyAlgorithmConfig>()
    .with_graph_loading_fn(|graph, _ctx| {
        // Convert graph to internal representation
        graph.into()
    })
    .with_position_update_fn(|displacements, vertices, ctx| {
        // Apply computed forces to node positions
        for idx in 0..vertices.len() {
            let displacement = displacements[idx];
            vertices[idx] += displacement.clamp_length_max(ctx.temperature);
        }
        // Cool down the system
        ctx.temperature *= 0.95;
    })
    // Add attractive forces between connected nodes
    .with_force(Force {
        force_fn: |pair: VertexPair<Vec2>, ctx| {
            let delta = pair.to - pair.from;
            let distance = delta.length().max(0.01);
            // Hooke's law: F = k * d
            delta.normalize() * (distance * ctx.spring_constant)
        },
        applicator_fn: linear_attraction_applicator,
    })
    // Add repulsive forces between all nodes
    .with_force(Force {
        force_fn: |pair: VertexPair<Vec2>, ctx| {
            let delta = pair.to - pair.from;
            let distance = delta.length().max(0.01);
            // Coulomb's law: F = k / d²
            delta.normalize() * -(ctx.spring_constant * 1000.0 / distance)
        },
        applicator_fn: linear_repulsion_applicator,
    })
    .to_layout();

// Use your algorithm
let graph = Graph { vertices: 10, edges: vec![...] };
layout.load_graph(&graph);

for _ in 0..100 {
    layout.iterate();
}

let positions = layout.get_positions();
```

### Force Applicators

Force applicators determine how forces are applied to nodes:

- **`linear_attraction_applicator`**: Apply forces only between connected nodes (edges)
- **`linear_repulsion_applicator`**: Apply forces between all pairs of nodes

You can also create custom applicators for specific layouts (e.g., hierarchical, radial).

### LayoutBuilder API

```rust
LayoutBuilder::build()
    .with_context_type::<YourContext>()           // Set configuration type
    .with_graph_loading_fn(|graph, ctx| {...})   // Load and initialize graph
    .with_position_update_fn(|disp, vert, ctx| {...}) // Update positions
    .with_force(Force { force_fn, applicator_fn }) // Add force calculations
    .to_layout()  // Build the algorithm
```

### Visualizer Configuration

Customize the visualizer appearance and behavior:

```rust
use force_smith::prelude::*;

let config = VisualizerConfiguration {
    // Scene
    background_color: GlobalColor::DarkGrey,

    // Camera
    camera_pan_speed: 1.0,
    camera_zoom_speed: 0.1,
    camera_keyboard_speed: 500.0,

    // Nodes
    node_radius: 10.0,
    node_color: GlobalColor::Cyan,
    node_movement_speed: 100.0,

    // Edges
    edge_width: 3.0,
    edge_color: GlobalColor::LightGrey,

    // Initial settings
    initial_mode: VisualizerMode::Normal,
    smooth_movement_enabled: true,

    ..Default::default()
};

visualize_dbg(layout, config);
```

**Available colors:**
- **Primary**: `Red`, `Green`, `Blue` (soft, muted tones)
- **Accents**: `Orange`, `Purple`, `Cyan`, `Pink`, `Yellow`
- **Neutrals**: `White`, `LightGrey`, `Grey`, `DarkGrey`, `Black`

All colors are carefully tuned for good contrast on dark/grey backgrounds.

### Using the Parameterized Derive Macro

For visualizer integration, use the `#[derive(Parameterized)]` macro to automatically expose your configuration parameters in the UI:

```rust
use force_smith::prelude::*;

#[derive(Parameterized)]
pub struct MyAlgorithmConfig {
    // Float parameters appear as sliders
    #[parameter(name = "Temperature")]
    temperature: f32,

    #[parameter(name = "Spring Constant")]
    spring_constant: f32,

    #[parameter(name = "Cooling Rate")]
    cooling_rate: f32,

    // Integer parameters appear as integer sliders
    #[parameter(name = "Iterations Per Cool")]
    iterations_per_cool: i32,

    // Boolean parameters appear as checkboxes
    #[parameter(name = "Enable Gravity")]
    gravity_enabled: bool,
}
```

**What the macro does:**
- Automatically implements the `Parameterized` trait
- Generates `get_parameters()` method to export current values to the UI
- Generates `update_parameters()` method to apply UI changes back to your config
- No manual serialization/deserialization needed!

**UI Representation:**
- `f32` → Slider with decimal precision
- `i32` → Slider with integer steps
- `bool` → Checkbox

**Supported types:** `f32`, `i32`, `bool`

The parameters will appear in the **"Config"** panel of the visualizer, allowing you to tune your algorithm interactively without recompiling.

**See it in action:**
```bash
cargo run --example parameterized_macro --features visualizer
```

This example shows 8 different parameters (f32, i32, and bool) and how they appear in the UI.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

MIT License - see LICENSE file for details.

## Repository

https://github.com/CodeByPhx/force_smith
