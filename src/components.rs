use bevy::{ ecs::entity::EntityHashSet, prelude::* };
use derive_new::new;

use crate::logic::signal::Signal;

pub mod prelude {
    pub use super::{ LogicFans, GateInput, GateOutput, OutputBundle };
}

/// A connection between two nodes.
#[derive(new, Component, Clone, Copy, Debug, Reflect)]
pub struct Wire {
    pub from: Entity,
    pub to: Entity,
}

/// Stores the input and output entities for a logic gate entity.
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

/// Marks an entity as an input.
#[derive(Component, Default)]
pub struct GateInput;

/// Marks an entity as an output, and stores
/// the [`Entity`] IDs of out-going wires.
#[derive(Component, Default)]
pub struct GateOutput {
    pub wires: EntityHashSet,
}

/// A bundle that can be used to create a child
/// **input** node of a logic gate entity.
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

/// A bundle that can be used to create a child
/// **output** node of a logic gate entity.
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
            output: GateOutput::default(),
            fan: GateFan::Output,
            spatial_bundle: Default::default(),
        }
    }
}
