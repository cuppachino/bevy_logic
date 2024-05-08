# `bevy_logic`

[![Crates.io](https://img.shields.io/crates/v/bevy_logic)](https://crates.io/crates/bevy_logic)
[![docs](https://docs.rs/bevy_logic/badge.svg)](https://docs.rs/bevy_logic/)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/cuppachino/bevy_logic/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/d/bevy_logic)](https://crates.io/crates/bevy_logic)

A logic gate simulation plugin for [`bevy`](https://bevyengine.org/).

## Features

- A `LogicGraph` resource for sorting (potentially cyclic) logic gate circuits.
- A fixed timestep `LogicUpdate` schedule that works just like bevy's `FixedUpdate`.
- A ternary approach to [`Signal`](src/logic/signal.rs), enabling non-boolean circuits and analog machines.
- [`LogicGate`](src/logic/mod.rs) trait queries.
- Builder traits for `World` and `Commands` that ease gate hierarchy construction.
- `Command`s for synchronizing a graph with the game world.
- Modular plugin design. Pick and choose which features you need.

### Running examples

```cmd
cargo run --release --example cycles
```

### Quickstart

Add the `LogicSimulationPlugin` to your app, and configure the `Time<LogicStep>` resource
to tick at the desired speed.

```rust
const STEPS_PER_SECOND: f64 = 30.0;
app.add_plugins(LogicSimulationPlugin)
    .insert_resource(Time::<LogicStep>::from_hz(STEPS_PER_SECOND));
```

### Custom logic gates

You can create your own logic gates by implementing the `LogicGate` trait...

```rust
use bevy_logic::prelude::*;

/// The XOR gate emits a signal if the number of "truthy" inputs is odd.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct XorGate;

impl LogicGate for XorGate {
    fn evaluate(&mut self, inputs: &[Signal], outputs: &mut [Signal]) {
        let signal: Signal = inputs
            .iter()
            .filter(|s| s.is_truthy())
            .count()
            .is_odd()
            .into();

        outputs.set_all(signal);
    }
}
```

And then registering the component with `bevy_trait_query`...

```rust
struct CustomLogicPlugin;

impl Plugin for CustomLogicPlugin {
    fn build(&self, app: &mut App) {
        app.register_logic_gate::<XorGate>();
    }
}
```

You can use the `logic::commands` module to spawn gates and fans,
and then connect fans with wires. Make sure to `compile()` the logic graph.

```rust
fn spawn_custom_gate(mut commands: Commands, mut sim: ResMut<LogicGraph>) {
    let xor_gate = commands
        .spawn_gate((Name::new("XOR"), XorGate))
        .with_inputs(2)
        .with_outputs(1)
        .build();

    let not_gate = commands
        .spawn_gate((Name::new("NOT"), NotGate))
        .with_inputs(1)
        .with_outputs(1)
        .build();

    let wire = commands.spawn_wire(&not_gate, 0, &xor_gate, 0).downgrade();

    sim.add_data(vec![xor_gate, not_gate]).add_data(wire).compile();
}
```

## Bevy Compatibility

| `bevy` | `bevy_logic` |
| ------ | ------------ |
| 0.13.2 | 0.6.x        |
