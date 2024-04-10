use bevy::{ prelude::*, sprite::MaterialMesh2dBundle };

mod camera_rig;
mod components;
mod logic;

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
    app.add_plugins(camera_rig::CameraRigPlugin);

    // main systems
    app.add_systems(Startup, setup);

    // run
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // cuboid
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::default())),
        material: materials.add(StandardMaterial::default()),
        transform: Transform::from_rotation(Quat::from_rotation_y((15_f32).to_radians())),
        ..Default::default()
    });
}
