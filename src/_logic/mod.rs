use bevy::prelude::*;

use crate::LogicGatePlugin;

use self::sensors::SensorPlugin;

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
pub mod sensors;
pub mod signal;
pub mod gates;
pub mod components {
    use derive_new::new;
    use derive_more::{ Add, AddAssign, Sub, SubAssign, From };

    use super::{ *, signal::Signal };

    /// An output node.
    #[derive(new, Component, Clone, Copy, Debug, Default, Deref, DerefMut, From)]
    pub struct Source {
        #[deref]
        pub signal: Signal,
    }

    impl Source {
        pub const OFF: Source = Source { signal: Signal::OFF };
        pub const ON: Source = Source { signal: Signal::ON };
    }

    /// An input node.
    #[derive(Component, Clone, Copy, Debug, Default, Deref, DerefMut, From)]
    pub struct Sink {
        #[deref]
        pub signal: Signal,
    }

    #[allow(dead_code)]
    impl Sink {
        pub const OFF: Sink = Sink { signal: Signal::OFF };
        pub const ON: Sink = Sink { signal: Signal::ON };
    }

    /// A connection between two nodes.
    #[derive(new, Component, Clone, Copy, Debug)]
    pub struct Wire {
        pub source: Entity,
        pub sink: Entity,
        #[new(default)]
        pub signal: Signal,
    }

    /// Blocks the evaluation of a `LogicGate` for a number of ticks.
    #[derive(
        Component,
        Clone,
        Copy,
        Debug,
        Default,
        Add,
        AddAssign,
        Sub,
        SubAssign,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        From
    )]
    pub struct Desync(pub u32);

    impl Desync {
        /// Increment the internal counter by `2`.
        pub(crate) fn increment(&mut self) {
            self.0 += 2;
        }

        /// Decrement the internal counter by `1`.
        pub(crate) fn decrement(&mut self) {
            self.0 -= 1;
        }
    }
}

pub struct LogicSimulationPlugin;

impl Plugin for LogicSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LogicGatePlugin, SensorPlugin));
    }
}

// pub struct LogicSimulationTimestep(pub f32);
