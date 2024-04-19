use std::marker::PhantomData;

use bevy::{ ecs::system::{ Command, EntityCommands }, prelude::* };

use crate::{
    components::{ GateOutput, InputBundle, LogicGateFans, OutputBundle, Wire },
    logic::signal::Signal,
};

pub trait LogicExt {
    type EntityBuilder<'a> where Self: 'a;
    type GateBuilder;
    type WireBuilder;

    fn spawn_gate(
        &mut self,
        bundle: impl Bundle
    ) -> GateBuilder<'_, Self::GateBuilder, Unknown, Unknown>;
    fn spawn_input(&mut self) -> Self::EntityBuilder<'_>;
    fn spawn_output(&mut self) -> Self::EntityBuilder<'_>;
    fn spawn_wire<I, O>(
        &mut self,
        from_gate: &GateData<I, Known>,
        from_output: usize,
        to_gate: &GateData<Known, O>,
        to_input: usize
    ) -> WireBuilder<'_, Self::WireBuilder>;
}

impl LogicExt for World {
    type EntityBuilder<'a> = EntityWorldMut<'a>;
    type GateBuilder = Self;
    type WireBuilder = Self;

    fn spawn_gate(&mut self, bundle: impl Bundle) -> GateBuilder<'_, Self::GateBuilder> {
        let entity = self.spawn(bundle).id();
        GateBuilder {
            cmd: self,
            data: GateData {
                entity,
                fans: LogicGateFans::default(),
                _state: PhantomData,
            },
        }
    }

    fn spawn_input(&mut self) -> Self::EntityBuilder<'_> {
        self.spawn(InputBundle::default())
    }

    fn spawn_output(&mut self) -> Self::EntityBuilder<'_> {
        self.spawn(OutputBundle::default())
    }

    /// Create a wire `from_gate` at `from_output` to `to_gate` at `to_input`,
    /// then update the gate output's `wires` set with the new wire entity.
    ///
    /// # Panics
    ///
    /// Panics if the input/output index is out of bounds, or if the input/output entity at `index` is `None`.
    fn spawn_wire<I, O>(
        &mut self,
        from_gate: &GateData<I, Known>,
        from_output: usize,
        to_gate: &GateData<Known, O>,
        to_input: usize
    ) -> WireBuilder<'_, Self::WireBuilder> {
        let from = from_gate.output(from_output);
        let to = to_gate.input(to_input);
        let entity = self.spawn((Signal::Undefined, Wire::new(from, to))).id();

        self.get_mut::<GateOutput>(from)
            .expect("from_gate entity does not have GateOutput component")
            .wires.insert(entity);

        WireBuilder {
            cmd: self,
            data: WireData {
                entity,
                from,
                to,
                from_gate: from_gate.id(),
                to_gate: to_gate.id(),
            },
        }
    }
}

impl<'w, 's> LogicExt for Commands<'w, 's> {
    type EntityBuilder<'a> = EntityCommands<'a> where Self: 'a;
    type GateBuilder = Self;
    type WireBuilder = Self;

    fn spawn_gate(&mut self, bundle: impl Bundle) -> GateBuilder<'_, Self::GateBuilder> {
        let entity = self.spawn(bundle).id();
        GateBuilder {
            cmd: self,
            data: GateData {
                entity,
                fans: LogicGateFans::default(),
                _state: PhantomData,
            },
        }
    }

    fn spawn_input(&mut self) -> Self::EntityBuilder<'_> {
        self.spawn(InputBundle::default())
    }

    fn spawn_output(&mut self) -> Self::EntityBuilder<'_> {
        self.spawn(OutputBundle::default())
    }

    /// Create a wire `from_gate` at `from_output` to `to_gate` at `to_input`,
    /// then update the gate output's `wires` set with the new wire entity.
    ///
    /// # Panics
    ///
    /// Panics if the input/output index is out of bounds, or if the input/output entity at `index` is `None`.
    fn spawn_wire<I, O>(
        &mut self,
        from_gate: &GateData<I, Known>,
        from_output: usize,
        to_gate: &GateData<Known, O>,
        to_input: usize
    ) -> WireBuilder<'_, Self::WireBuilder> {
        let from = from_gate.output(from_output);
        let to = to_gate.input(to_input);
        let entity = self.spawn((Signal::Undefined, Wire::new(from, to))).id();

        self.add(UpdateOutputWireSet::Add { output_entity: from, wire_entity: entity });

        WireBuilder {
            cmd: self,
            data: WireData {
                entity,
                from,
                to,
                from_gate: from_gate.id(),
                to_gate: to_gate.id(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Unknown;

#[derive(Debug, Clone, Copy)]
pub struct Known;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateData<I = Unknown, O = Unknown> {
    entity: Entity,
    fans: LogicGateFans,
    _state: PhantomData<(I, O)>,
}

impl<I, O> GateData<I, O> {
    pub fn id(&self) -> Entity {
        self.entity
    }

    pub fn inputs(&self) -> &[Option<Entity>] {
        &self.fans.inputs
    }

    pub fn outputs(&self) -> &[Option<Entity>] {
        &self.fans.outputs
    }
}

impl<O> GateData<Known, O> {
    /// Get the input entity at `index`.
    ///
    /// Returns `None` if the index is out of bounds or if the input entity is `None`.
    pub fn get_input(&self, index: usize) -> Option<Entity> {
        self.fans.inputs.get(index).copied().flatten()
    }

    /// # Panics
    ///
    /// Panics if the input index is out of bounds, or if the input entity at `index` is `None`.
    pub fn input(&self, index: usize) -> Entity {
        self.fans.inputs[index].expect("input entity is None")
    }
}

impl<I> GateData<I, Known> {
    /// Get the output entity at `index`.
    ///
    /// Returns `None` if the index is out of bounds or if the output entity is `None`.
    pub fn get_output(&self, index: usize) -> Option<Entity> {
        self.fans.outputs.get(index).copied().flatten()
    }

    /// # Panics
    ///
    /// Panics if the output index is out of bounds, or if the input entity at `index` is `None`.
    pub fn output(&self, index: usize) -> Entity {
        self.fans.outputs[index].expect("input entity is None")
    }
}

pub struct GateBuilder<'a, T, I = Unknown, O = Unknown> {
    cmd: &'a mut T,
    data: GateData<I, O>,
}

/// A trait that provides mutable access to an [`EntityWorldMut`] and its index in the range `0..count`.
pub trait GateFanWorldMut {
    fn modify_fan(&mut self, cmd: &mut EntityWorldMut, index: usize);
}
impl<T> GateFanWorldMut for T where T: FnMut(&mut EntityWorldMut, usize) {
    fn modify_fan(&mut self, cmd: &mut EntityWorldMut, index: usize) {
        self(cmd, index);
    }
}

pub trait GateFanEntityMut {
    fn modify_fan(&mut self, cmd: &mut EntityCommands, index: usize);
}

impl<T> GateFanEntityMut for T where T: FnMut(&mut EntityCommands, usize) {
    fn modify_fan(&mut self, cmd: &mut EntityCommands, index: usize) {
        self(cmd, index);
    }
}

impl<'a, I, O> GateBuilder<'a, World, I, O> {
    pub fn world(&mut self) -> &mut World {
        self.cmd
    }

    pub fn entity_commands(&mut self) -> EntityWorldMut<'_> {
        self.cmd.entity_mut(self.data.entity)
    }

    pub fn insert_bundle(mut self, bundle: impl Bundle) -> Self {
        self.entity_commands().insert(bundle);
        self
    }

    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.entity_commands().insert(bundle);
        self
    }
}

impl<'a, O> GateBuilder<'a, World, Unknown, O> {
    pub fn with_inputs(self, count: usize) -> GateBuilder<'a, World, Known, O> {
        let mut inputs = Vec::with_capacity(count);
        self.cmd.entity_mut(self.data.entity).with_children(|gate| {
            for _ in 0..count {
                inputs.push(Some(gate.spawn(InputBundle::default()).id()));
            }
        });

        GateBuilder {
            cmd: self.cmd,
            data: GateData {
                entity: self.data.entity,
                fans: LogicGateFans {
                    inputs,
                    outputs: self.data.fans.outputs,
                },
                _state: PhantomData,
            },
        }
    }

    /// Build `count` input entities and use `builder` on each entity. Provides
    /// access to the input [`EntityWorldMut`] and its index in the range `0..count`.
    pub fn build_inputs(
        self,
        count: usize,
        mut builder: impl GateFanWorldMut
    ) -> GateBuilder<'a, World, Known, O> {
        let mut inputs = Vec::with_capacity(count);

        self.cmd.entity_mut(self.data.entity).with_children(|gate| {
            for i in 0..count {
                let mut cmd = gate.spawn(InputBundle::default());
                let input_entity = cmd.id();
                inputs.push(Some(input_entity));
                builder.modify_fan(&mut cmd, i);
            }
        });

        GateBuilder {
            cmd: self.cmd,
            data: GateData {
                entity: self.data.entity,
                fans: LogicGateFans {
                    inputs,
                    outputs: self.data.fans.outputs,
                },
                _state: PhantomData,
            },
        }
    }
}

impl<'a, I> GateBuilder<'a, World, I, Unknown> {
    pub fn with_outputs(self, count: usize) -> GateBuilder<'a, World, I, Known> {
        let mut outputs = Vec::with_capacity(count);
        self.cmd.entity_mut(self.data.entity).with_children(|gate| {
            for _ in 0..count {
                outputs.push(Some(gate.spawn(OutputBundle::default()).id()));
            }
        });

        GateBuilder {
            cmd: self.cmd,
            data: GateData {
                entity: self.data.entity,
                fans: LogicGateFans {
                    inputs: self.data.fans.inputs,
                    outputs,
                },
                _state: PhantomData,
            },
        }
    }

    /// Build `count` output entities and call `builder` on each entity. Provides
    /// access to the output [`EntityWorldMut`] and its index in the range `0..count`.
    pub fn build_outputs(
        self,
        count: usize,
        mut builder: impl GateFanWorldMut
    ) -> GateBuilder<'a, World, I, Known> {
        let mut outputs = Vec::with_capacity(count);

        self.cmd.entity_mut(self.data.entity).with_children(|gate| {
            for i in 0..count {
                let mut cmd = gate.spawn(OutputBundle::default());
                let output_entity = cmd.id();
                outputs.push(Some(output_entity));
                builder.modify_fan(&mut cmd, i);
            }
        });

        GateBuilder {
            cmd: self.cmd,
            data: GateData {
                entity: self.data.entity,
                fans: LogicGateFans {
                    inputs: self.data.fans.inputs,
                    outputs,
                },
                _state: PhantomData,
            },
        }
    }
}

impl<'a, I, O> GateBuilder<'a, World, I, O> {
    /// Finalize construction of the gate hierarchy, link children, and insert a [`LogicEntity`]
    /// component into the root entity from [`Self::data`].
    ///
    /// Returns [`Self::data`], which can be used to wire inputs/outputs together
    /// by their [`Entity`] IDs and link gates in a logic graph.
    pub fn build(self) -> GateData<I, O> {
        self.cmd
            .entity_mut(self.data.entity)
            .push_children(
                &self.data.fans
                    .some_inputs()
                    .into_iter()
                    .chain(self.data.fans.some_outputs())
                    .collect::<Vec<_>>()
            )
            .insert(self.data.fans.clone());

        self.data
    }
}

//* Gate builder for `Commands` */

impl<'w, 's, 'a, I, O> GateBuilder<'a, Commands<'w, 's>, I, O> {
    pub fn entity_commands(&mut self) -> EntityCommands<'_> {
        self.cmd.entity(self.data.entity)
    }

    pub fn insert_bundle(mut self, bundle: impl Bundle) -> Self {
        self.entity_commands().insert(bundle);
        self
    }

    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.entity_commands().insert(bundle);
        self
    }
}

impl<'w, 's, 'a, O> GateBuilder<'a, Commands<'w, 's>, Unknown, O> {
    pub fn with_inputs(self, count: usize) -> GateBuilder<'a, Commands<'w, 's>, Known, O> {
        let mut inputs = Vec::with_capacity(count);
        self.cmd.entity(self.data.entity).with_children(|gate| {
            for _ in 0..count {
                inputs.push(Some(gate.spawn(InputBundle::default()).id()));
            }
        });

        GateBuilder {
            cmd: self.cmd,
            data: GateData {
                entity: self.data.entity,
                fans: LogicGateFans {
                    inputs,
                    outputs: self.data.fans.outputs,
                },
                _state: PhantomData,
            },
        }
    }

    /// Build `count` input entities and use `builder` on each entity. Provides
    /// access to the input [`EntityWorldMut`] and its index in the range `0..count`.
    pub fn build_inputs(
        self,
        count: usize,
        mut builder: impl GateFanEntityMut
    ) -> GateBuilder<'a, Commands<'w, 's>, Known, O> {
        let mut inputs = Vec::with_capacity(count);

        self.cmd.entity(self.data.entity).with_children(|gate| {
            for i in 0..count {
                let mut cmd = gate.spawn(InputBundle::default());
                let input_entity = cmd.id();
                inputs.push(Some(input_entity));
                builder.modify_fan(&mut cmd, i);
            }
        });

        GateBuilder {
            cmd: self.cmd,
            data: GateData {
                entity: self.data.entity,
                fans: LogicGateFans {
                    inputs,
                    outputs: self.data.fans.outputs,
                },
                _state: PhantomData,
            },
        }
    }
}

impl<'w, 's, 'a, I> GateBuilder<'a, Commands<'w, 's>, I, Unknown> {
    pub fn with_outputs(self, count: usize) -> GateBuilder<'a, Commands<'w, 's>, I, Known> {
        let mut outputs = Vec::with_capacity(count);
        self.cmd.entity(self.data.entity).with_children(|gate| {
            for _ in 0..count {
                outputs.push(Some(gate.spawn(OutputBundle::default()).id()));
            }
        });

        GateBuilder {
            cmd: self.cmd,
            data: GateData {
                entity: self.data.entity,
                fans: LogicGateFans {
                    inputs: self.data.fans.inputs,
                    outputs,
                },
                _state: PhantomData,
            },
        }
    }

    /// Build `count` output entities and call `builder` on each entity. Provides
    /// access to the output [`EntityWorldMut`] and its index in the range `0..count`.
    pub fn build_outputs(
        self,
        count: usize,
        mut builder: impl GateFanEntityMut
    ) -> GateBuilder<'a, Commands<'w, 's>, I, Known> {
        let mut outputs = Vec::with_capacity(count);

        self.cmd.entity(self.data.entity).with_children(|gate| {
            for i in 0..count {
                let mut cmd = gate.spawn(OutputBundle::default());
                let output_entity = cmd.id();
                outputs.push(Some(output_entity));
                builder.modify_fan(&mut cmd, i);
            }
        });

        GateBuilder {
            cmd: self.cmd,
            data: GateData {
                entity: self.data.entity,
                fans: LogicGateFans {
                    inputs: self.data.fans.inputs,
                    outputs,
                },
                _state: PhantomData,
            },
        }
    }
}

impl<'w, 's, 'a, I, O> GateBuilder<'a, Commands<'w, 's>, I, O> {
    /// Finalize construction of the gate hierarchy, link children, and insert a [`LogicEntity`]
    /// component into the root entity from [`Self::data`].
    ///
    /// Returns [`Self::data`], which can be used to wire inputs/outputs together
    /// by their [`Entity`] IDs and link gates in a logic graph.
    pub fn build(self) -> GateData<I, O> {
        self.cmd
            .entity(self.data.entity)
            .push_children(
                &self.data.fans
                    .some_inputs()
                    .into_iter()
                    .chain(self.data.fans.some_outputs())
                    .collect::<Vec<_>>()
            )
            .insert(self.data.fans.clone());

        self.data
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WireData {
    pub entity: Entity,
    pub from: Entity,
    pub from_gate: Entity,
    pub to: Entity,
    pub to_gate: Entity,
}

impl WireData {
    #[inline]
    pub const fn id(&self) -> Entity {
        self.entity
    }

    #[doc(alias = "from")]
    #[doc(alias = "sink")]
    #[inline]
    pub const fn input(&self) -> Entity {
        self.from
    }

    #[doc(alias = "to")]
    #[doc(alias = "source")]
    #[inline]
    pub const fn output(&self) -> Entity {
        self.to
    }
}

pub struct WireBuilder<'a, T> {
    cmd: &'a mut T,
    data: WireData,
}

impl<T> WireBuilder<'_, T> {
    pub fn world(&mut self) -> &mut T {
        self.cmd
    }

    /// Returns the [`Entity`] ID of the wire.
    pub const fn id(&self) -> Entity {
        self.data.entity
    }

    /// Returns the [`Entity`] associated with the `from` node of the wire.
    #[doc(alias = "from")]
    #[doc(alias = "sink")]
    #[inline]
    pub const fn input(&self) -> Entity {
        self.data.from
    }

    /// Returns the [`Entity`] associated with the `to` node of the wire.
    #[doc(alias = "to")]
    #[doc(alias = "source")]
    #[inline]
    pub const fn output(&self) -> Entity {
        self.data.to
    }
}

impl WireBuilder<'_, World> {
    /// Downgrade the builder into a [`WireData`] instance,
    /// dropping the mutable reference to the world.
    pub fn downgrade(self) -> WireData {
        self.data
    }

    pub fn entity_commands(&mut self) -> EntityWorldMut<'_> {
        self.cmd.entity_mut(self.data.entity)
    }

    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.entity_commands().insert(bundle);
        self
    }
}

impl<'w, 's> WireBuilder<'_, Commands<'w, 's>> {
    /// Downgrade the builder into a [`WireData`] instance,
    /// dropping the mutable reference to the world.
    pub fn downgrade(self) -> WireData {
        self.data
    }

    pub fn entity_commands(&mut self) -> EntityCommands<'_> {
        self.cmd.entity(self.data.entity)
    }

    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.entity_commands().insert(bundle);
        self
    }
}

/// A [`Command`] that adds or removes a wire entity from a [`GateOutput`] component's `wires` set.
///
/// The set may be used to lookup out-going wires from a gate output entity, so it's important to
/// keep it up-to-date when adding or removing wires.
pub enum UpdateOutputWireSet {
    Add {
        output_entity: Entity,
        wire_entity: Entity,
    },
    Remove {
        output_entity: Entity,
        wire_entity: Entity,
    },
}

impl Command for UpdateOutputWireSet {
    fn apply(self, world: &mut World) {
        match self {
            UpdateOutputWireSet::Add { output_entity, wire_entity } => {
                world
                    .get_mut::<GateOutput>(output_entity)
                    .expect("output entity does not have GateOutput component")
                    .wires.insert(wire_entity);
            }
            UpdateOutputWireSet::Remove { output_entity, wire_entity } => {
                world
                    .get_mut::<GateOutput>(output_entity)
                    .expect("output entity does not have GateOutput component")
                    .wires.remove(&wire_entity);
            }
        }
    }
}
