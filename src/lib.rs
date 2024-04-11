use bevy::prelude::*;

pub trait SpawnDemoLights {
    fn spawn_demo_lights(&mut self);
}

impl<'w, 's> SpawnDemoLights for Commands<'w, 's> {
    fn spawn_demo_lights(&mut self) {
        self.spawn(PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });

        self.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_752.7,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
                ..default()
            },
            ..default()
        });
    }
}

/// Creates a rounded rectangle mesh. The rectangle is centered at the origin.
pub fn gate_mesh(input_count: usize, output_count: usize) -> Mesh {
    const GATE_SIZE: f32 = 1.0;
    const GATE_THICKNESS: f32 = 0.25;

    let gate_height = GATE_SIZE * ((input_count.max(output_count) / 2).max(1) as f32);

    #[allow(deprecated)]
    let mesh = Mesh::from(shape::Box::new(GATE_SIZE, gate_height, GATE_THICKNESS));
    mesh
}
