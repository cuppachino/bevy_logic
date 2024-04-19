# `bevy_logic`

[![Crates.io](https://img.shields.io/crates/v/bevy_logic)](https://crates.io/crates/bevy_logic)
[![docs](https://docs.rs/bevy_logic/badge.svg)](https://docs.rs/bevy_logic/)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/cuppachino/bevy_logic/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/d/bevy_logic)](https://crates.io/crates/bevy_logic)

A logic gate simulation plugin for [`bevy`](https://bevyengine.org/).

## Features

- A `LogicGraph` resource for sorting (potentially cyclic) logic gate circuits.
- A separate `LogicUpdate` schedule with a fixed timestep.
- `LogicGate` trait queries.
- `World` and `Commands` extensions for spawning and removing logic gates and child fans.
- Events for `LogicGraph` simulation synchronization.
- Modular plugin design. Pick and choose the features you need.

### Running examples

```cmd
cargo run --release --example 3d
```

## Bevy Compatibility

| `bevy` | `bevy_logic` |
| ------ | ------------ |
| 0.13   | 0.1.*        |
