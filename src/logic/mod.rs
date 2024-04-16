use bevy::prelude::*;

pub mod signal;

pub mod prelude {
    pub use super::signal::Signal;
}

/// A trait that defines the behavior of a logic gate.
#[bevy_trait_query::queryable]
pub trait LogicGate {
    /// Evaluate the current state of inputs (in order), and update the outputs (in order).
    fn evaluate(&self, inputs: &[signal::Signal], outputs: &mut [signal::Signal]);
}
