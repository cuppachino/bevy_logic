use bevy::prelude::*;

use super::{
    commands::{ LogicCommands, LogicEntityCommands, SpawnSourcesAndSinks },
    components::*,
    gates::*,
    signal::Signal,
};

/// A trait for spawning a "battery" entity.
pub trait BatteryExt<'w, 's> {
    fn battery<T>(&mut self, strength: T) -> LogicEntityCommands where T: Into<Signal>;
}

impl<'w, 's, 'a> BatteryExt<'w, 's> for Commands<'w, 's> {
    fn battery<T>(&mut self, strength: T) -> LogicEntityCommands where T: Into<Signal> {
        let entity_commands = self.spawn(Battery::new(strength.into()));
        let mut logic_commands = LogicEntityCommands::new(entity_commands);
        logic_commands.with_sources(1);
        logic_commands
    }
}

/// A trait for spawning a "wire" entity.
pub trait WireExt<'w, 's> {
    fn wire(&mut self, source: Entity, sink: Entity) -> &mut Self;
}

impl<'w, 's, 'a> WireExt<'w, 's> for Commands<'w, 's> {
    fn wire(&mut self, source: Entity, sink: Entity) -> &mut Self {
        self.spawn(Wire::new(source, sink));
        self
    }
}

impl<'w, 's, 'a> WireExt<'w, 's> for LogicCommands<'w, 's, 'a> {
    fn wire(&mut self, source: Entity, sink: Entity) -> &mut Self {
        self.spawn_sync(Wire::new(source, sink));
        self
    }
}

impl<'w, 's, 'a> LogicCommands<'w, 's, 'a> {
    pub fn spawn_empty(&mut self) -> LogicEntityCommands {
        let entity_commands = self.commands.spawn(self.desync);
        self.desync.increment();
        LogicEntityCommands::new(entity_commands)
    }

    pub fn spawn(&mut self, bundle: impl Bundle) -> LogicEntityCommands {
        let entity_commands = self.commands.spawn((bundle, self.desync));
        self.desync.increment();
        LogicEntityCommands::new(entity_commands)
    }

    pub fn spawn_sync(&mut self, bundle: impl Bundle) -> LogicEntityCommands {
        let entity_commands = self.commands.spawn(bundle);
        LogicEntityCommands::new(entity_commands)
    }
}

pub trait NodeExt<'w, 's> {
    /// Create an entity with a [LogicNode] component,
    /// and then spawn one child [Source] and one child [Sink].
    ///
    /// Returns [LogicEntityCommands] for the node entity.
    fn node(&mut self) -> LogicEntityCommands;
}

impl<'w, 's, 'a> NodeExt<'w, 's> for LogicCommands<'w, 's, 'a> {
    fn node(&mut self) -> LogicEntityCommands {
        let mut node = self.spawn_sync(LogicNode);
        node.with_sources(1).with_sinks(1);
        node
    }
}
