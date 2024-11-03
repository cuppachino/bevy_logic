use bevy::{ ecs::world::Command, prelude::* };
use crate::{
    components::{ GateOutput, Wire },
    logic::builder::WireData,
    prelude::{ LogicGateFans, LogicGraph },
};

pub mod prelude {
    pub use super::{
        AddGateToLogicGraph,
        RemoveGateFromLogicGraph,
        AddWireToLogicGraph,
        RemoveWireFromLogicGraph,
    };
}

/// A command that adds a logic entity to the [`LogicGraph`] resource and
/// all wires connected to it. This does not spawn any entities.
///
/// [`LogicGraph`]: crate::resources::LogicGraph
pub struct AddGateToLogicGraph(pub Entity);

impl Command for AddGateToLogicGraph {
    fn apply(self, world: &mut World) {
        let wire_data = extract_outgoing_wires(self.0, world);

        world.resource_mut::<LogicGraph>().add_gate(self.0).add_data(wire_data).compile();
    }
}

/// A command that removes a logic entity from the [`LogicGraph`] resource and
/// all wires connected to it. This updates the [`GateOutput::wires`] set
/// for the targeted gate and any gates incoming via wire.
///
/// This command does not despawn any entities.
///
/// Unlike [`AddGateToLogicGraph`], this command collects wire data
/// using the [`LogicGraph`] resource instead of [`extract_outgoing_wires`].
///
/// [`GateOutput::wires`]: crate::components::GateOutput::wires
/// [`LogicGraph`]: crate::resources::LogicGraph
pub struct RemoveGateFromLogicGraph(pub Entity);

impl Command for RemoveGateFromLogicGraph {
    fn apply(self, world: &mut World) {
        let mut sim = world.resource_mut::<LogicGraph>();
        let incoming_wires: Vec<_> = sim.iter_incoming_wires(self.0).collect();
        sim.remove_gate(self.0).compile();

        for (wire_entity, wire) in incoming_wires {
            world
                .get_mut::<GateOutput>(wire.from)
                .expect("Wire::from Entity does not have GateOutput component")
                .wires.remove(&wire_entity);
        }
    }
}

/// A command that adds an edge between two logic entities in the [`LogicGraph`] resource and
/// updates the [`GateOutput::wires`] set for the output fan entity.
///
/// This command does not spawn any entities.
///
/// [`GateOutput::wires`]: crate::components::GateOutput::wires
/// [`LogicGraph`]: crate::resources::LogicGraph
pub struct AddWireToLogicGraph(pub Entity);

impl Command for AddWireToLogicGraph {
    fn apply(self, world: &mut World) {
        let wire_entity = self.0;
        let &wire = world.get::<Wire>(wire_entity).expect("Entity does not have a Wire component");

        // Update the `wires` set in the output fan.
        world
            .get_mut::<GateOutput>(wire.from)
            .expect("Wire::from Entity does not have GateOutput component")
            .wires.insert(wire_entity);

        // Grab the gates for the graph.
        let from_gate = world
            .get::<Parent>(wire.from)
            .expect("GateOutput does not have a parent gate")
            .get();
        let to_gate = world
            .get::<Parent>(wire.to)
            .expect("GateInput does not have a parent gate")
            .get();

        // Add the data and recompile
        world.resource_mut::<LogicGraph>().add_wire(from_gate, to_gate, wire_entity).compile();
    }
}

/// A command that removes a wire between two logic entities in the [`LogicGraph`] resource and
/// updates the [`GateOutput::wires`] set for the output fan entity.
///
/// This command does not despawn any entities. It is important that the [`Entity`]
/// already exists with a [`Wire`] component.
///
/// [`GateOutput::wires`]: crate::components::GateOutput::wires
/// [`LogicGraph`]: crate::resources::LogicGraph
pub struct RemoveWireFromLogicGraph(pub Entity);

impl Command for RemoveWireFromLogicGraph {
    fn apply(self, world: &mut World) {
        let wire_entity = self.0;
        let &wire = world.get::<Wire>(wire_entity).expect("Entity does not have a Wire component");

        // Update the `wires` set in the output fan.
        world
            .get_mut::<GateOutput>(wire.from)
            .expect("Wire::from Entity does not have GateOutput component")
            .wires.remove(&wire_entity);

        // Grab the gates for the graph.
        let from_gate = world
            .get::<Parent>(wire.from)
            .expect("GateOutput does not have a parent gate")
            .get();
        let to_gate = world
            .get::<Parent>(wire.to)
            .expect("GateInput does not have a parent gate")
            .get();

        // Remove the data and recompile
        world.resource_mut::<LogicGraph>().remove_wire(from_gate, to_gate).compile();
    }
}

/// A [`Command`] that adds or removes a wire entity from a [`GateOutput`] component's `wires` set.
///
/// The set may be used to lookup out-going wires from a gate output entity, so it's important to
/// keep it up-to-date when adding or removing wires.
///
/// This does not change the [`LogicGraph`] resource.
///
/// [`LogicGraph`]: crate::resources::LogicGraph
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

/// Collect outgoing [`WireData`] from a logic gate entity in the world.
pub fn extract_outgoing_wires(entity: Entity, world: &mut World) -> Vec<WireData> {
    world
        .get::<LogicGateFans>(entity)
        .expect("Cannot add an entity without `LogicGateFans` to the `LogicGraph`.")
        .some_outputs()
        .into_iter()
        .map(|output_entity| {
            world
                .get::<GateOutput>(output_entity)
                .expect(
                    "Entity stored in `LogicGateFans::outputs` does not have a `GateOutput` component"
                )
                .wires.iter()
                .map(|wire_entity| {
                    {
                        let wire = world
                            .get::<Wire>(*wire_entity)
                            .expect("`GateOutput` should only store IDs to `Wire` entities");
                        let to_gate = world
                            .get::<Parent>(wire.to)
                            .expect("GateInput should have a parent entity")
                            .get();

                        WireData {
                            entity: *wire_entity,
                            from_gate: entity,
                            from: wire.from,
                            to: wire.to,
                            to_gate,
                        }
                    }
                })
        })
        .flatten()
        .collect::<Vec<_>>()
}
