# Force Smith

A Rust toolkit for implementing and developing force-directed layout algorithms with built-in debugging and visualization capabilities.

## Repository Structure

```
force_smith/
├── force_smith/          # Main library crate
├── force_smith_macros/   # Procedural macros
└── graphs/              # Sample graph files for examples
```

## Quick Start

See the [force_smith/README.md](force_smith/README.md) for detailed documentation and usage examples.

### Running Examples

```bash
cd force_smith

# Run the Fruchterman-Reingold algorithm
cargo run --example fruchterman_reingold

# Run with interactive visualizer
cargo run --example fruchterman_reingold_visualizer --features visualizer

# See the Parameterized derive macro in action
cargo run --example parameterized_macro --features visualizer
```

## Features

- 🔧 **Builder API**: Ergonomic composition of force-directed algorithms
- 🎨 **Interactive Visualizer**: Real-time visualization with debug mode
- ⚡ **High Performance**: Built on Bevy ECS
- 🔍 **Debug Tools**: Step-by-step force visualization
- 📦 **Modular Design**: Mix and match components
- 📚 **Complete Examples**: Learn from Fruchterman-Reingold implementation
- 🎛️ **Parameterized Macro**: Automatic UI generation for algorithm parameters

## Examples Included

1. **Fruchterman-Reingold** - Classic force-directed algorithm implementation
2. **Fruchterman-Reingold Visualizer** - Interactive version with real-time parameter tuning
3. **Parameterized Macro** - Demonstrates the #[derive(Parameterized)] macro with f32, i32, and bool parameters

## License

MIT License - see LICENSE file for details.
