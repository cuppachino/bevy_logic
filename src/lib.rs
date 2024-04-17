use bevy::prelude::*;

pub mod logic;
pub mod systems;
pub mod components;
pub mod resources;

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::logic::prelude::*;
    pub use crate::systems::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::resources::prelude::*;

    pub use super::LogicSimulationPlugin;
    pub use super::LogicReflectPlugin;
}

/// A plugin group that adds all crate features to an [`App`].
#[derive(Default)]
pub struct LogicSimulationPlugin;

impl Plugin for LogicSimulationPlugin {
    fn build(&self, app: &mut App) {
        use prelude::*;
        app.add_plugins((LogicSchedulePlugin, LogicReflectPlugin, LogicGatePlugin))
            .insert_resource(Time::<LogicStep>::from_seconds(0.5))
            .init_resource::<LogicGraph>()
            .add_systems(LogicUpdate, systems::step_logic);
    }
}

/// A plugin that registers components' reflect data for a given [`App`].
pub struct LogicReflectPlugin;

impl Plugin for LogicReflectPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<logic::signal::Signal>()
            .register_type::<components::Wire>()
            .register_type::<components::GateFan>()
            .register_type::<components::LogicFans>()
            .register_type::<resources::LogicGraph>();
    }
}
