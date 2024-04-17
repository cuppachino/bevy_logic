use bevy::{ ecs::entity::EntityHashSet, prelude::* };

use super::gates::{ LogicGate, LogicSystemSet };

pub mod prelude {
    pub use super::{ UiClickSensor, SensorPlugin };
}

pub struct SensorPlugin;

impl Plugin for SensorPlugin {
    fn build(&self, app: &mut App) {
        // We must import this trait in order to register our components.
        // If we don't register them, they will be invisible to the game engine.
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn LogicGate, UiClickSensor>();

        app.register_type::<UiClickSensor>().add_systems(
            FixedUpdate,
            update_ui_click_sensors.in_set(LogicSystemSet::SpawnEntities)
        );
    }
}

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct UiClickSensor {
    pub is_pressed: bool,
}

impl LogicGate for UiClickSensor {
    fn evaluate(&self, _: &[crate::Signal], outputs: &mut [crate::Source]) {
        let signal = self.is_pressed.into();
        outputs.iter_mut().for_each(|output| {
            output.signal = signal;
        });
    }
}

pub fn update_ui_click_sensors(
    mut query: Query<(&Interaction, &mut UiClickSensor), Changed<Interaction>>
) {
    for (interaction, mut sensor) in query.iter_mut() {
        sensor.is_pressed = match interaction {
            Interaction::Pressed => {
                println!("Pressed!");
                true
            }
            _ => false,
        };
    }
}
