# Force Smith Macros

Procedural macros for the Force Smith force-directed graph layout library.

## Overview

This crate provides derive macros that simplify working with Force Smith's visualizer by automatically generating trait implementations for parameterized layout algorithms.

## Macros

### `#[derive(Parameterized)]`

Automatically implements the `Parameterized` trait for structs, allowing them to expose their fields as adjustable parameters in the Force Smith visualizer UI.

#### Usage

```rust
use force_smith::prelude::*;

#[derive(Parameterized)]
pub struct MyLayoutConfig {
    #[parameter(name = "Temperature")]
    temperature: f32,

    #[parameter(name = "Edge Length")]
    edge_length: f32,

    #[parameter(name = "Enable Gravity")]
    gravity_enabled: bool,
}
```

This automatically generates:
- `get_parameters(&self) -> Vec<Parameter>` - Returns all parameters with their current values
- `update_parameters(&mut self, parameters: &[Parameter])` - Updates parameters from UI changes

#### Supported Types

- `f32` - Floating point parameters (displayed as draggable value in UI)
- `i32` - Integer parameters (displayed as draggable value in UI)
- `bool` - Boolean parameters (displayed as checkbox in UI)

#### Attributes

- `#[parameter(name = "Display Name")]` - Marks a field as a parameter with a custom display name
  - If `name` is not specified, the field name is used

#### Example

```rust
use force_smith::prelude::*;

#[derive(Parameterized, Default)]
pub struct ForceConfig {
    #[parameter(name = "Temperature")]
    pub temperature: f32,

    #[parameter(name = "Cooling Rate")]
    pub cooling_rate: f32,

    #[parameter(name = "Ideal Edge Length")]
    pub ideal_edge_length: f32,

    #[parameter(name = "Apply Repulsion")]
    pub repulsion_enabled: bool,
}

impl Default for ForceConfig {
    fn default() -> Self {
        Self {
            temperature: 100.0,
            cooling_rate: 0.95,
            ideal_edge_length: 50.0,
            repulsion_enabled: true,
        }
    }
}
```

When used with the visualizer, these parameters will appear in the "Parameter Configuration" UI window and can be adjusted in real-time while the layout algorithm runs.

## Live Example

See a complete working example with multiple parameter types:

```bash
cd ../force_smith
cargo run --example parameterized_macro --features visualizer
```

This example demonstrates:
- Multiple `f32` parameters (temperature, spring length, stiffness, etc.)
- An `i32` parameter (iterations per cool)
- Multiple `bool` parameters (gravity enabled)
- How parameters appear and behave in the visualizer UI
- Real-time updates affecting algorithm behavior

## Requirements

This crate is designed to work with Force Smith 0.1+ and requires the `visualizer` feature to be enabled.

## Integration

The macro generates code that integrates with Force Smith's visualizer module. The generated implementation uses:
- `force_smith::visualizer::layout_trait::Parameterized` - The trait being implemented
- `force_smith::visualizer::layout_trait::ToParameterValue` - For converting Rust types to parameter values
- `force_smith::visualizer::layout_trait::FromParameterValue` - For converting parameter values back to Rust types
- `force_smith::prelude::Parameter` - The parameter type used in the visualizer

## License

MIT License - see LICENSE file for details.

## Repository

https://github.com/CodeByPhx/force_smith
