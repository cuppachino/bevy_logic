use bevy::prelude::*;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::components::*;
    pub use super::gates::{ LogicGatePlugin, Battery, NotGate, AndGate, OrGate };
    pub use super::signal::Signal;
    pub use super::command_extensions::*;
    pub use super::commands::{ LogicEntity, LogicEntityCommands, SourceSinkGetter };
}

pub mod commands;
pub mod command_extensions;
pub mod signal;
pub mod gates;
pub mod components {
    use super::{ *, signal::Signal };

    /// An output node.
    #[derive(Component, Clone, Copy, Debug, Default)]
    pub struct Source {
        pub signal: Signal,
    }

    /// An input node.
    #[derive(Component, Clone, Copy, Debug, Default)]
    pub struct Sink {
        pub signal: Signal,
    }

    /// A connection between two nodes.
    #[derive(Component, Clone, Copy, Debug)]
    pub struct Wire {
        pub source: Entity,
        pub sink: Entity,
    }
}

fn debug_logic_components(
    mut gizmos: Gizmos,
    query_gates: Query<(&dyn gates::LogicGate, &Children)>,
    query_sources: Query<&components::Source>,
    query_sinks: Query<&components::Sink>,
    query_wires: Query<&components::Wire>
) {
    for (gate, children) in query_gates.iter() {
        let sources = children.iter().filter_map(|entity| query_sources.get(*entity).ok());
        let sinks = children.iter().filter_map(|entity| query_sinks.get(*entity).ok());
        let wires = children.iter().filter_map(|entity| query_wires.get(*entity).ok());

        // todo: spatial bundles on gates

        // gizmos.circle(position, normal, radius, color)
    }
}
