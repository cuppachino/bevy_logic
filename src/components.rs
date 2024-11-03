use bevy::{ ecs::entity::EntityHashSet, prelude::* };

use crate::logic::signal::Signal;

pub mod prelude {
    pub use super::{
        Wire,
        WireBundle,
        LogicGateFans,
        GateFan,
        GateInput,
        GateOutput,
        InputBundle,
        OutputBundle,
        NoEvalOutput,
    };
}

/// A component that connects two logic gates with the entity IDs
/// of their child fans.
#[derive(Component, Clone, Copy, Debug, Reflect)]
pub struct Wire {
    /// The [`GateOutput`] entity.
    pub from: Entity,
    /// The [`GateInput`] entity.
    pub to: Entity,
}

impl Wire {
    /// Create a new wire from an `Entity` with a [`GateOutput`] to an `Entity` with a [`GateInput`].
    pub fn new(from: Entity, to: Entity) -> Self {
        Self { from, to }
    }
}

/// A bundle used to create a wire between a [`GateOutput`] and [`GateInput`].
#[derive(Bundle, Clone, Copy)]
pub struct WireBundle {
    pub wire: Wire,
    pub signal: Signal,
}

/// Marks an entity as a logic gate entity, and stores the
/// input and output fans of the gate.
#[derive(Component, Clone, Debug, Default, PartialEq, Eq, Reflect)]
pub struct LogicGateFans {
    pub inputs: Vec<Option<Entity>>,
    pub outputs: Vec<Option<Entity>>,
}

impl LogicGateFans {
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

/// Marks an entity as an input.
#[derive(Component, Default)]
pub struct GateInput;

/// Marks an entity as an output, and stores
/// the [`Entity`] IDs of out-going wires.
#[derive(Component, Default)]
pub struct GateOutput {
    /// Holds [Entity] ids to outgoing wires.
    pub wires: EntityHashSet,
}

/// Marks an entity as an output that does not require
/// evaluation. If the entity includes an [`OutputBundle`],
/// it's [`Signal`] will be propagated to all connected wires
/// when changed.
///
/// This is useful for "gates" that do not have any inputs,
/// such as buttons and interactive gameplay elements that can
/// ONLY output a signal.
///
/// See the `advanced_gates` example.
#[derive(Component, Default)]
pub struct NoEvalOutput;

/// A bundle that can be used to create a child
/// **input** node of a logic gate entity.
#[derive(Bundle)]
pub struct InputBundle {
    pub signal: Signal,
    pub input: GateInput,
    pub fan: GateFan,
}

impl Default for InputBundle {
    fn default() -> Self {
        Self {
            signal: Signal::Undefined,
            input: GateInput,
            fan: GateFan::Input,
        }
    }
}

/// A bundle that can be used to create a child
/// **output** node of a logic gate entity.
#[derive(Bundle)]
pub struct OutputBundle {
    pub signal: Signal,
    pub output: GateOutput,
    pub fan: GateFan,
}

impl Default for OutputBundle {
    fn default() -> Self {
        Self {
            signal: Signal::Undefined,
            output: GateOutput::default(),
            fan: GateFan::Output,
        }
    }
}
