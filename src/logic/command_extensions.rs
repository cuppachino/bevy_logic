use bevy::prelude::*;

use super::{
    commands::{ LogicEntityCommands, SpawnSourcesAndSinks },
    gates::*,
    components::*,
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

/// A trait for spawning common logic gates.
pub trait LogicGateExt<'w, 's> {
    fn node(&mut self) -> LogicEntityCommands;
    fn not_gate(&mut self) -> LogicEntityCommands;
    fn and_gate(&mut self) -> LogicEntityCommands;
    fn or_gate(&mut self, is_adder: bool) -> LogicEntityCommands;
}

impl<'w, 's, 'a> LogicGateExt<'w, 's> for Commands<'w, 's> {
    fn node(&mut self) -> LogicEntityCommands {
        let entity_commands = self.spawn(LogicNode);
        let mut logic_commands = LogicEntityCommands::new(entity_commands);
        logic_commands.with_sources(1).with_sinks(1);
        logic_commands
    }

    fn and_gate(&mut self) -> LogicEntityCommands {
        let entity_commands = self.spawn(AndGate);
        let mut logic_commands = LogicEntityCommands::new(entity_commands);
        logic_commands.with_sources(1).with_sinks(2);
        logic_commands
    }

    fn not_gate(&mut self) -> LogicEntityCommands {
        let entity_commands = self.spawn(NotGate);
        let mut logic_commands = LogicEntityCommands::new(entity_commands);
        logic_commands.with_sources(1).with_sinks(1);
        logic_commands
    }

    fn or_gate(&mut self, is_adder: bool) -> LogicEntityCommands {
        let entity_commands = self.spawn(OrGate { is_adder });
        let mut logic_commands = LogicEntityCommands::new(entity_commands);
        logic_commands.with_sources(1).with_sinks(2);
        logic_commands
    }
}
