use bevy::prelude::*;

pub mod logic;
pub mod systems;
pub mod components;
pub mod resources;
pub mod commands;

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::logic::prelude::*;
    pub use crate::systems::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::resources::prelude::*;

    pub use super::LogicSimulationPlugin;
}

/// A plugin group containing all logic simulation systems.
#[derive(Default)]
pub struct LogicSimulationPlugin;

impl Plugin for LogicSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<logic::signal::Signal>()
            .register_type::<components::LogicFans>()
            .register_type::<components::Wire>()
            .register_type::<components::GateFan>()
            .register_type::<components::AndGate>()
            .register_type::<components::OrGate>();

        app.add_plugins(components::LogicGateTraitQueryPlugin)
            .init_resource::<resources::LogicGraph>()
            .add_systems(FixedUpdate, systems::step_logic)
            .insert_resource(Time::<Fixed>::from_seconds(0.5));
    }
}
