use bevy::prelude::*;
use derive_new::new;

use crate::logic::{ signal::Signal, LogicGate };

pub mod prelude {
    pub use super::{ LogicFans, GateInput, GateOutput, OutputBundle };
}

#[derive(Component, Clone, Debug, Default, PartialEq, Eq, Reflect)]
pub struct LogicFans {
    pub inputs: Vec<Option<Entity>>,
    pub outputs: Vec<Option<Entity>>,
}

impl LogicFans {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// Push an input entity to the inputs vector.
    pub fn with_input(mut self, input: Option<Entity>) -> Self {
        self.inputs.push(input);
        self
    }

    /// Push an output entity to the outputs vector.
    pub fn with_output(mut self, output: Option<Entity>) -> Self {
        self.outputs.push(output);
        self
    }

    /// Return the number of inputs associated with this logic entity.
    pub fn input_len(&self) -> usize {
        self.inputs.len()
    }

    /// Return the number of outputs associated with this logic entity.
    pub fn output_len(&self) -> usize {
        self.outputs.len()
    }

    /// Resize the input vector in-place so that `len` is equal to `count`.
    pub fn resize_inputs(&mut self, count: usize) {
        self.inputs.resize(count, Default::default());
    }

    /// Resize the output vector in-place so that `len` is equal to `count`.
    pub fn resize_outputs(&mut self, count: usize) {
        self.outputs.resize(count, Default::default());
    }

    /// Returns a vector of entities that are not `None`.
    pub fn some_inputs(&self) -> Vec<Entity> {
        self.inputs.iter().flatten().copied().collect::<Vec<_>>()
    }

    /// Returns a vector of entities that are not `None`.
    pub fn some_outputs(&self) -> Vec<Entity> {
        self.outputs.iter().flatten().copied().collect::<Vec<_>>()
    }
}

/// Marks an entity as either an input or an output.
#[derive(Component, Reflect)]
pub enum GateFan {
    Input,
    Output,
}

impl GateFan {
    /// Returns `true` if the gate fan is [`Output`].
    ///
    /// [`Output`]: GateFan::Output
    #[must_use]
    pub fn is_output(&self) -> bool {
        matches!(self, Self::Output)
    }

    /// Returns `true` if the gate fan is [`Input`].
    ///
    /// [`Input`]: GateFan::Input
    #[must_use]
    pub fn is_input(&self) -> bool {
        matches!(self, Self::Input)
    }
}
impl From<GateInput> for GateFan {
    fn from(_: GateInput) -> Self {
        Self::Input
    }
}
impl From<GateOutput> for GateFan {
    fn from(_: GateOutput) -> Self {
        Self::Output
    }
}

#[derive(Component, Default)]
pub struct GateInput;

#[derive(Component, Default)]
pub struct GateOutput;

#[derive(Bundle)]
pub struct InputBundle {
    pub signal: Signal,
    pub input: GateInput,
    pub fan: GateFan,
    pub spatial_bundle: SpatialBundle,
}

impl Default for InputBundle {
    fn default() -> Self {
        Self {
            signal: Signal::Undefined,
            input: GateInput,
            fan: GateFan::Input,
            spatial_bundle: Default::default(),
        }
    }
}

#[derive(Bundle)]
pub struct OutputBundle {
    pub signal: Signal,
    pub output: GateOutput,
    pub fan: GateFan,
    pub spatial_bundle: SpatialBundle,
}

impl Default for OutputBundle {
    fn default() -> Self {
        Self {
            signal: Signal::Undefined,
            output: GateOutput,
            fan: GateFan::Output,
            spatial_bundle: Default::default(),
        }
    }
}

/// A connection between two nodes.
#[derive(new, Component, Clone, Copy, Debug, Reflect)]
pub struct Wire {
    pub from: Entity,
    pub to: Entity,
}

#[derive(Component, Clone, Copy, Debug, Reflect)]
pub struct AndGate;

impl LogicGate for AndGate {
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Signal]) {
        let signal = inputs.iter().all(Signal::is_true);
        outputs.iter_mut().for_each(|output_signal| {
            *output_signal = signal.into();
        });
    }
}

#[derive(Component, Clone, Copy, Debug, Reflect)]
pub struct NotGate;

impl LogicGate for NotGate {
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Signal]) {
        let signal = !inputs.iter().all(Signal::is_true);
        outputs.iter_mut().for_each(|output_signal| {
            *output_signal = signal.into();
        });
    }
}

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

        outputs.iter_mut().for_each(|output_signal| {
            *output_signal = signal;
        });
    }
}

/// A battery that emits a constant signal.
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

pub struct LogicGateTraitQueryPlugin;

impl Plugin for LogicGateTraitQueryPlugin {
    fn build(&self, app: &mut App) {
        // We must import this trait in order to register our components.
        // If we don't register them, they will be invisible to the game engine.
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn LogicGate, Battery>();

        app.register_component_as::<dyn LogicGate, AndGate>()
            .register_component_as::<dyn LogicGate, NotGate>()
            .register_component_as::<dyn LogicGate, OrGate>();
    }
}
