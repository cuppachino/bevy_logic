use bevy::prelude::*;

use crate::{ logic::{ signal::Signal, LogicGate }, utils::NumExt };

use super::signal::SignalExt;

/// This plugin registers basic logic gates and a battery component.
///
/// They can be queried using the [`LogicGate`] trait.
///
/// # Example
///
/// ```no_run
/// fn query(q: Query<&One<dyn LogicGate>>) {
///     // ...
/// }
///
/// ```
pub struct LogicGatePlugin;

impl Plugin for LogicGatePlugin {
    fn build(&self, app: &mut App) {
        // We must import this trait in order to register our components.
        // If we don't register them, they will be invisible to the game engine.
        use bevy_trait_query::RegisterExt;
        app.register_component_as::<dyn LogicGate, AndGate>()
            .register_component_as::<dyn LogicGate, OrGate>()
            .register_component_as::<dyn LogicGate, NotGate>()
            .register_component_as::<dyn LogicGate, XorGate>()
            .register_component_as::<dyn LogicGate, Battery>();

        // Register the components' reflection data.
        app.register_type::<AndGate>()
            .register_type::<OrGate>()
            .register_type::<NotGate>()
            .register_type::<XorGate>()
            .register_type::<Battery>();
    }
}

/// A [`Battery`] emits a constant signal.
#[derive(Component, Clone, Copy, Debug, Reflect)]
pub struct Battery {
    pub signal: Signal,
}

impl Default for Battery {
    fn default() -> Self {
        Self::ON
    }
}

#[allow(dead_code)]
impl Battery {
    pub const OFF: Battery = Battery::new(Signal::OFF);
    pub const ON: Battery = Battery::new(Signal::ON);
    pub const NEG: Battery = Battery::new(Signal::NEG);

    /// Create a new battery with `signal`.
    pub const fn new(signal: Signal) -> Self {
        Self { signal }
    }
}

impl LogicGate for Battery {
    fn evaluate(&self, _: &[Signal], outputs: &mut [Signal]) {
        outputs.iter_mut().for_each(|output_signal| {
            *output_signal = self.signal;
        });
    }
}

/// An AND gate emits a signal if all inputs are true.
///
/// ```md
/// Truth table:
/// | A | B | Q |
/// |---|---|---|
/// | 0 | 0 | 0 |
/// | 0 | 1 | 0 |
/// | 1 | 0 | 0 |
/// | 1 | 1 | 1 |
/// ```
#[derive(Component, Clone, Copy, Debug, Reflect)]
pub struct AndGate;

impl LogicGate for AndGate {
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Signal]) {
        let signal: Signal = inputs.iter().all(Signal::is_true).into();
        outputs.set_all(signal);
    }
}

/// A NOT gate emits a signal if all inputs are false.
///
/// ```md
/// Truth table:
/// | A | Q |
/// |---|---|
/// | 0 | 1 |
/// | 1 | 0 |
/// ```
#[derive(Component, Clone, Copy, Debug, Reflect)]
pub struct NotGate;

impl LogicGate for NotGate {
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Signal]) {
        let signal: Signal = (!inputs.iter().all(Signal::is_true)).into();
        outputs.set_all(signal);
    }
}

/// An OR gate emits a signal if any input is true.
///
/// - If `invert_output` is true, the gate will be a NOR gate instead.
/// - If `is_adder` is true, the gate will act as an analog adder.
///
/// ```md
/// Truth table:
/// | A | B | Q |
/// |---|---|---|
/// | 0 | 0 | 0 |
/// | 0 | 1 | 1 |
/// | 1 | 0 | 1 |
/// | 1 | 1 | 1 |
/// ```
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct OrGate {
    /// If true, the gate will be a NOR gate instead of an OR gate.
    pub invert_output: bool,
    /// If true, the gate will act as an analog adder,
    /// computing the sum of all inputs.
    pub is_adder: bool,
}
impl OrGate {
    pub const NOR: OrGate = OrGate { is_adder: false, invert_output: true };
}

impl LogicGate for OrGate {
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Signal]) {
        let signal = if self.is_adder {
            inputs.iter().fold(Signal::OFF, |acc, input| { acc + *input })
        } else {
            inputs.iter().any(Signal::is_true).into()
        };

        let signal = if self.invert_output { !signal } else { signal };

        outputs.set_all(signal);
    }
}

/// The XOR gate emits a signal if the number of true inputs is odd.
///
/// In other words, if there are an odd number of truthy inputs, the output is true.
///
/// ```md
/// Truth table:
/// | A | B | Q |
/// |---|---|---|
/// | 0 | 0 | 0 |
/// | 0 | 1 | 1 |
/// | 1 | 0 | 1 |
/// | 1 | 1 | 0 |
/// ```
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct XorGate;

impl LogicGate for XorGate {
    fn evaluate(&self, inputs: &[super::signal::Signal], outputs: &mut [super::signal::Signal]) {
        let signal: Signal = inputs
            .iter()
            .filter(|s| s.is_true())
            .count()
            .is_odd()
            .into();

        outputs.set_all(signal);
    }
}
