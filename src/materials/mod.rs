pub mod logic_gate_material;

use bevy::prelude::*;

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<logic_gate_material::LogicGateMaterial>::default());
    }
}
