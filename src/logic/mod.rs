pub mod signal;
pub mod gates;
pub mod builder;
pub mod schedule;

pub mod prelude {
    pub use super::builder::LogicExt;
    pub use super::gates::*;
    pub use super::schedule::prelude::*;
    pub use super::signal::{ Signal, SignalExt };
    pub use super::LogicGate;
}

use signal::Signal;

/// A trait that defines the behavior of a logic gate.
#[bevy_trait_query::queryable]
pub trait LogicGate {
    /// Evaluate the current state of inputs (in order), and update the outputs (in order).
    fn evaluate(&mut self, inputs: &[Signal], outputs: &mut [Signal]);
}
