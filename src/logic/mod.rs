pub mod signal;
pub mod gates;
pub mod builder;
pub mod schedule;

pub mod prelude {
    pub use super::builder::LogicExt;
    pub use super::gates::*;
    pub use super::schedule::prelude::*;
    pub use super::signal::{ Signal, SignalExt };
    pub use super::{ LogicGate, AppLogicGateExt };
}

use bevy::prelude::*;
use signal::Signal;

/// A trait that defines the behavior of a logic gate.
#[bevy_trait_query::queryable]
pub trait LogicGate {
    /// Evaluate the current state of inputs (in order), and update the outputs (in order).
    fn evaluate(&mut self, inputs: &[Signal], outputs: &mut [Signal]);
}

/// An [App] extension for registering `LogicGate` components through `bevy_trait_query`.
pub trait AppLogicGateExt {
    /// Register a component that implements `LogicGate` via `bevy_trait_query`.
    ///
    /// Gates must be registered or they will not be queryable.
    ///
    /// Calling this multiple times with the same arguments will do nothing on subsequent calls.
    ///
    /// # Panics
    ///
    /// Panics if called after starting the [`World`] simulation.
    fn register_logic_gate<T: Component + LogicGate>(&mut self) -> &mut Self;
}

impl AppLogicGateExt for App {
    fn register_logic_gate<T: Component + LogicGate>(&mut self) -> &mut Self {
        use bevy_trait_query::RegisterExt;
        self.register_component_as::<dyn LogicGate, T>()
    }
}
