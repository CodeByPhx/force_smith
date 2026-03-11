# Force Smith Examples

This directory contains comprehensive examples demonstrating various features of Force Smith.

## Quick Reference

| Example | Features Needed | Description |
|---------|----------------|-------------|
| `fruchterman_reingold` | None (default) | Complete FR algorithm implementation |
| `fruchterman_reingold_visualizer` | `--features visualizer` | FR with real-time tuning |
| `parameterized_macro` | `--features visualizer` | Derive macro demonstration |

## Examples by Learning Path

### Beginner Path

1. **fruchterman_reingold.rs** - Start here
   ```bash
   cargo run --example fruchterman_reingold
   ```
   Study a complete implementation of a classic algorithm with proper force calculations.

2. **parameterized_macro.rs** - UI integration
   ```bash
   cargo run --example parameterized_macro --features visualizer
   ```
   Learn how to expose parameters in the UI using the derive macro.

### Advanced Path

3. **fruchterman_reingold_visualizer.rs** - Full integration
   ```bash
   cargo run --example fruchterman_reingold_visualizer --features visualizer
   ```
   See everything come together: custom algorithm + parameterized macro + visualizer.

## Detailed Example Descriptions

### fruchterman_reingold.rs

**What it teaches:**
- Complete algorithm implementation
- Fruchterman-Reingold force formulas:
  - Attractive: `f_a(d) = d² / k`
  - Repulsive: `f_r(d) = -k² / d`
- Spring constant calculation
- Cooling schedule
- Iteration and convergence

**Lines of code:** ~140

**Key concepts:**
- Classical graph drawing algorithm
- Energy minimization
- Temperature scheduling
- Graph loading and position extraction

**Sample output:**
```
Running Fruchterman-Reingold layout algorithm...
Initial positions:
  Node 0: (7.53, -84.16)
  ...
Iteration 20
Sample positions:
  Node 0: (221.74, 454.44)
  ...
```

---

### fruchterman_reingold_visualizer.rs

**What it teaches:**
- Using `#[derive(Parameterized)]` macro
- Combining custom algorithm with visualizer
- Configuring visualizer colors and appearance
- Feature-gated code organization

**Lines of code:** ~90

**Key concepts:**
- Macro-based parameter exposure
- Algorithm + visualization integration
- VisualizerConfiguration customization

**Parameters exposed:**
- Temperature
- Spring Constant (k)
- Cooling Rate

---

### parameterized_macro.rs

**What it teaches:**
- All parameter types: `f32`, `i32`, `bool`
- Parameter naming with `#[parameter(name = "...")]`
- How parameters appear in UI
- Conditional behavior based on bool parameters
- Complex configuration with 8+ parameters

**Lines of code:** ~150

**Key concepts:**
- Derive macro usage
- Type-specific UI elements (sliders, checkboxes)
- Real-time algorithm tuning
- Feature toggles in algorithms

**Parameters demonstrated:**
- 6 × `f32` parameters (temperature, spring length, stiffness, cooling rate, repulsion, gravity)
- 1 × `i32` parameter (iterations per cool)
- 1 × `bool` parameter (enable gravity)

**UI appearance:**
- `f32` → Slider with decimal values
- `i32` → Slider with integer steps
- `bool` → Checkbox

---

## Running All Examples

Build all examples at once:
```bash
# Non-visualizer examples
cargo build --example fruchterman_reingold

# All examples including visualizer
cargo build --examples --features visualizer
```

## Example File Locations

All examples are in: `force_smith/examples/`

Related resources:
- Sample graphs: `../graphs/` (simple_graph.json, tree_graph.json, etc.)
- Main library: `../src/`
- Macro documentation: `../../force_smith_macros/README.md`

## Tips for Learning

1. **Start with Fruchterman-Reingold**: See a complete, well-known algorithm
2. **Experiment with parameters**: Use the visualizer examples to see effects in real-time
3. **Study the macro**: parameterized_macro shows all parameter types
4. **Read the code**: Examples are heavily commented - read them as tutorials

## Common Modifications

### Change the graph in visualizer
Use the **Graph Configuration** UI panel to:
- Load graphs from `../graphs/` directory
- Generate random graphs with custom vertex/edge counts
- Toggle "Connected" to ensure graph connectivity

### Adjust forces
```rust
.with_force(Force {
    force_fn: |pair: VertexPair<Vec2>, ctx| {
        // Your custom force formula
        let delta = pair.to - pair.from;
        delta * 0.5  // Half strength
    },
    applicator_fn: linear_attraction_applicator,
})
```

### Change colors
```rust
let config = VisualizerConfiguration {
    background_color: GlobalColor::Black,
    node_color: GlobalColor::Green,
    edge_color: GlobalColor::Grey,
    ..Default::default()
};
```

## Getting Help

- Read the main README: `../README.md`
- Check API docs: `../src/`
- Macro details: `../../force_smith_macros/README.md`
- Repository: https://github.com/CodeByPhx/force_smith
