use bevy::prelude::*;
use super::signal::Signal;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{ LogicGatePlugin, NotGate, AndGate, OrGate };
}

/// A trait that defines the behavior of a logic gate.
#[bevy_trait_query::queryable]
pub trait LogicGate {
    /// Evaluate the current state of inputs (in order), and update the outputs (in order).
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Signal]);
}

/// A plugin that registers logic gates and simulates their behavior.
pub struct LogicGatePlugin;

impl Plugin for LogicGatePlugin {
    fn build(&self, app: &mut App) {
        // We must import this trait in order to register our components.
        // If we don't register them, they will be invisible to the game engine.
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn LogicGate, NotGate>()
            .register_component_as::<dyn LogicGate, AndGate>()
            .register_component_as::<dyn LogicGate, OrGate>();
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct NotGate;

impl LogicGate for NotGate {
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Signal]) {
        let signal = !inputs.iter().all(Signal::is_true);
        outputs.iter_mut().for_each(|output| {
            *output = signal.into();
        });
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct AndGate;

impl LogicGate for AndGate {
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Signal]) {
        let signal = inputs.iter().all(Signal::is_true);
        outputs.iter_mut().for_each(|output| {
            *output = signal.into();
        });
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct OrGate {
    /// If true, the gate will act as an analog adder,
    /// computing the sum of all inputs.
    pub is_adder: bool,
}

impl LogicGate for OrGate {
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Signal]) {
        let signal = if self.is_adder {
            inputs.iter().fold(Signal::OFF, |acc, input| { acc + *input })
        } else {
            inputs.iter().any(Signal::is_true).into()
        };
        outputs.iter_mut().for_each(|output| {
            *output = signal;
        });
    }
}
