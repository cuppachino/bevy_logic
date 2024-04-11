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
    use derive_new::new;

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
    #[derive(new, Component, Clone, Copy, Debug)]
    pub struct Wire {
        pub source: Entity,
        pub sink: Entity,
        #[new(default)]
        pub signal: Signal,
    }
}
