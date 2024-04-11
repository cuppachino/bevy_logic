use bevy::prelude::*;

mod camera_rig;
mod components;
mod cursor;
mod logic;
mod materials;

pub mod prelude {
    pub use crate::components::prelude::*;
}

fn main() {
    let mut app = App::new();

    // external plugins
    app.add_plugins(DefaultPlugins).insert_resource(
        ClearColor(Color::rgba_linear(0.22, 0.402, 0.598, 1.0))
    );

    // crate plugins
    app.add_plugins((
        camera_rig::CameraRigPlugin,
        cursor::CursorPlugin,
        logic::prelude::LogicGatePlugin,
        materials::MaterialsPlugin,
    ));

    // main systems
    app.add_systems(Startup, (setup, example));

    // run
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // cuboid
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::default())),
            material: materials.add(StandardMaterial::default()),
            transform: Transform::from_rotation(Quat::from_rotation_y((15_f32).to_radians())),
            ..Default::default()
        },
        bevy_mod_picking::PickableBundle::default(),
    ));
}

use logic_tools::{ SpawnDemoLights, gate_mesh };
use self::logic::prelude::*;

pub fn example(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn_demo_lights();

    let mut battery = commands.battery(true);
    battery.insert(PbrBundle {
        mesh: meshes.add(gate_mesh(0, 1)),
        transform: Transform::from_xyz(-2.0, 0.0, 2.0),
        material: materials.add(StandardMaterial::from(Color::WHITE)),
        ..default()
    });
    let battery = battery.downgrade();

    let mut not_gate = commands.not_gate();
    not_gate.insert((
        PbrBundle {
            mesh: meshes.add(gate_mesh(1, 1)),
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            material: materials.add(StandardMaterial::from(Color::YELLOW)),
            ..default()
        },
    ));
    let not_gate = not_gate.downgrade();

    commands.wire(battery.source(0).unwrap(), not_gate.sink(0).unwrap());
}
