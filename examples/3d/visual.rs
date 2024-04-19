use bevy::{ prelude::*, utils::HashMap };
use bevy_logic::{ components::{ GateFan, GateOutput, LogicGateFans, Wire }, logic::signal::Signal };

pub struct VisualPlugin;

impl Plugin for VisualPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_logic_gate_icons).add_systems(Update, (
            colorize_logic_gates,
            gizmo_wires,
        ));
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GateIcon {
    And,
    Or,
    Not,
    Battery,
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

fn colorize_logic_gates(
    query: Query<(&LogicGateFans, &Handle<StandardMaterial>)>,
    query_outputs: Query<&Signal, With<GateOutput>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    for (fans, material) in query.iter() {
        // if any of the outputs are true, the gate is on.
        let is_active = fans
            .some_outputs()
            .iter()
            .any(|output| {
                let signal = query_outputs.get(*output).unwrap();
                signal.is_true()
            });

        let color = if is_active { Color::WHITE } else { Color::GRAY };

        let material = materials.get_mut(material).unwrap();
        material.base_color = color;
    }
}

fn gizmo_wires(
    mut gizmos: Gizmos,
    query_wires: Query<(&Wire, &Signal)>,
    query_fans: Query<&GlobalTransform, With<GateFan>>
) {
    for (wire, signal) in query_wires.iter() {
        let Ok(from) = query_fans.get(wire.from).map(|t| t.translation()) else {
            continue;
        };
        let Ok(to) = query_fans.get(wire.to).map(|t| t.translation()) else {
            continue;
        };

        let color = if signal.is_true() { Color::GREEN } else { Color::BLACK };

        gizmos.line(from, to, color);
        gizmos.circle(from, Direction3d::Z, 0.1, color);
        gizmos.circle(to, Direction3d::Z, 0.1, color);
    }
}
