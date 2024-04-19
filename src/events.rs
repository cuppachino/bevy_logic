use bevy::prelude::*;

use crate::logic::schedule::{ LogicSystemSet, LogicUpdate };

pub mod prelude {
    pub use super::{ LogicEvent, LogicEventPlugin };
}

/// This plugin provides a [`LogicEvent`]. It can be used to update the [`LogicGraph`] resource,
/// and [`GateOutput`] components that store connected wire entities. This is useful for keeping
/// the simulation in sync with your game's visual representation of logic gates and wires.
///
/// Because [`read_logic_events`] is added to [`LogicUpdate`],
/// the [`LogicSchedulePlugin`] must be added to the app.
///
/// Alternatively, you can manually register [`LogicEvent`] and add [`read_logic_events`] to
/// whichever schedule best fits your game.
///
/// [`read_logic_events`]: crate::systems::read_logic_events
/// [`LogicSchedulePlugin`]: crate::logic::schedule::LogicSchedulePlugin
pub struct LogicEventPlugin;

impl Plugin for LogicEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LogicEvent>().add_systems(
            LogicUpdate,
            crate::systems::read_logic_events
                .run_if(on_event::<LogicEvent>())
                .in_set(LogicSystemSet::SyncGraph)
        );
    }
}

/// Events that keep the [`LogicGraph`] resource in sync with the game world.
///
/// Variants that require an entity lookup are expected to be read before the entity is despawned,
/// otherwise the event will not be able to update the [`LogicGraph`] resource.
///
/// [LogicGraph]: crate::resources::LogicGraph
#[derive(Event, Clone, Copy, Debug)]
pub enum LogicEvent {
    /// Add an entity to the [`LogicGraph`]. The entity is expected
    /// to own a [`LogicGateFans`] component that stores the input and output fans of the gate,
    /// and is used to query the entity in [`crate::systems`].
    ///
    /// This does not spawn a new entity.
    AddGate(Entity),

    /// Remove a logic entity from the [`LogicGraph`].
    /// This will also remove all wires connected to the entity,
    /// and update wire sets stored in [`GateOutput`] components.
    ///
    /// This does not despawn any entities.
    RemoveGate(Entity),

    /// Add a wire to the [`LogicGraph`] by linking two gate entities.
    ///
    /// This does not spawn a new entity with a [`Wire`] component.
    ///
    /// This event adds an edge between two gates in the graph and
    /// updates the wire Sets stored in [`GateOutput`] components.
    AddWire {
        from_gate: Entity,
        from_output: Entity,
        to_gate: Entity,
        wire_entity: Entity,
    },

    /// Add a wire to the [`LogicGraph`] using an existing entity with a [`Wire`] component.
    ///
    /// This will also update wire sets stored in [`GateOutput`] components.
    AddWireByEntity(Entity),

    /// Remove a wire from the [`LogicGraph`] by the entities that define it.
    /// This will also update wire sets stored in [`GateOutput`] components.
    ///
    /// This does not despawn any entities.
    RemoveWire {
        from_gate: Entity,
        to_gate: Entity,
        wire_entity: Entity,
    },

    /// Remove a wire from the [`LogicGraph`].
    /// This will also update wire sets stored in [`GateOutput`] components.
    ///
    /// It is important that the entity exists and is not despawned before the event is read.
    ///
    /// This does not despawn any entities.
    RemoveWireByEntity(Entity),
}
