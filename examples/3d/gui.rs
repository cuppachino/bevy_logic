use bevy::{ asset::LoadedAsset, prelude::*, utils::HashMap };

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_logic_gate_icons);
    }
}

#[derive(Resource, Default)]
pub struct LogicGateIcons {
    map: HashMap<GateIcon, Handle<Image>>,
}

impl LogicGateIcons {
    /// # Panics
    ///
    /// Panics if the icon is not found.
    pub fn get(&self, icon: GateIcon) -> Handle<Image> {
        self.map.get(&icon).unwrap().clone()
    }
}

fn init_logic_gate_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let or_icon: Handle<Image> = asset_server.load("textures/logic_gates/orx128.png");
    let and_icon: Handle<Image> = asset_server.load("textures/logic_gates/andx128.png");
    let not_icon: Handle<Image> = asset_server.load("textures/logic_gates/notx128.png");
    let battery_icon: Handle<Image> = asset_server.load("textures/logic_gates/batteryx128.png");

    commands.insert_resource(LogicGateIcons {
        map: {
            let mut hashmap = HashMap::default();
            hashmap.insert(GateIcon::And, and_icon);
            hashmap.insert(GateIcon::Or, or_icon);
            hashmap.insert(GateIcon::Not, not_icon);
            hashmap.insert(GateIcon::Battery, battery_icon);
            hashmap
        },
    });
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GateIcon {
    And,
    Or,
    Not,
    Battery,
}
