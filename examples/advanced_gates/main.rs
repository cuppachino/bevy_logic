use bevy::{ ecs::system::EntityCommands, prelude::* };
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_logic::{ components::GateFan, logic::builder::GateFanEntityMut, prelude::* };
use itertools::Itertools;

mod camera_rig;
mod visual;
mod triangulation;
use visual::*;

use crate::camera_rig::CameraRigPlugin;

/// A selector should always have EXACTLY 1 more input
/// than the number of outputs. The first input is expected
/// to be the cycle input, instead of the last input because
/// the number of states/fans may change.
#[derive(Component, Default)]
struct Selector {
    last_input: Option<usize>,
}

impl LogicGate for Selector {
    fn evaluate(&mut self, inputs: &[Signal], outputs: &mut [Signal]) {
        #[cfg(debug_assertions)]
        {
            if inputs.len() != outputs.len() + 1 {
                panic!(
                    "Selector does not have the correct number of child fans. There must be exactly 1 more input than outputs."
                );
            }
        }

        if let Some((index, signal)) = inputs.iter().find_position(|input| { input.is_truthy() }) {
            if self.last_input.is_some_and(|i| i == index) {
                return;
            } else {
                self.last_input.replace(index);
            }

            if index == 0 {
                let mut iter = outputs.iter_mut().skip_while(|o| o.is_falsy());
                if let Some(active) = iter.next() {
                    active.turn_off();
                }
                if let Some(next) = iter.next() {
                    next.replace(*signal);
                } else {
                    outputs
                        .first_mut()
                        .expect("Selector does not have any outputs")
                        .replace(*signal);
                }
            } else {
                self.last_input.replace(index);
                outputs.set_all(Signal::OFF);
                outputs
                    .get_mut(index - 1)
                    .unwrap()
                    .replace(*signal);
            }
            return;
        } else {
            self.last_input = None;
        }

        if outputs.iter().all(|s| s.is_falsy()) {
            if let Some(first) = outputs.first_mut() {
                first.turn_on();
            }
        }
    }
}

fn main() {
    let mut app = App::new();

    // Register the `Selector` as a `LogicGate`.
    use bevy_trait_query::RegisterExt;
    app.register_component_as::<dyn LogicGate, Selector>();

    // Add the `LogicSimulationPlugin`.
    app.add_plugins((
        DefaultPlugins,
        CameraRigPlugin,
        WorldInspectorPlugin::new(),
        LogicSimulationPlugin,
    ))
        .insert_resource(ClearColor(Color::rgba_linear(0.22, 0.402, 0.598, 1.0)))
        .add_systems(Startup, (setup_scene, spawn_logic))
        .add_systems(Update, gizmo_wires)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        point_light: PointLight {
            intensity: 150000.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_logic(
    mut commands: Commands,
    mut sim: ResMut<LogicGraph>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let selector = commands
        .spawn_gate((
            Name::new("Selector"),
            Selector::default(),
            pbr(Vec3::new(-1.0, 0.0, 0.0), meshes.add(build_mesh(4, 4, 1)), &mut materials),
        ))
        .build_inputs(5, selector_input_entity_mut(5))
        .build_outputs(4, fan_entity_mut(GateFan::Output, 4))
        .build();

    let or_gate = commands
        .spawn_gate((
            Name::new("OR"),
            OrGate::default(),
            pbr(Vec3::new(2.0, 1.0, 0.0), meshes.add(build_mesh(3, 1, 0)), &mut materials),
        ))
        .build_inputs(3, fan_entity_mut(GateFan::Input, 3))
        .build_outputs(1, fan_entity_mut(GateFan::Output, 1))
        .build();

    sim.add_data(
        vec![
            commands.spawn_wire(&selector, 0, &or_gate, 0).downgrade(),
            commands.spawn_wire(&selector, 1, &or_gate, 1).downgrade(),
            commands.spawn_wire(&selector, 3, &or_gate, 2).downgrade()
        ]
    )
        .add_data(selector)
        .add_data(or_gate)
        .compile();
}

pub const GATE_UNIT_SIZE: f32 = 1.0;
pub const GATE_UNIT_HALF_SIZE: f32 = 0.5;
pub const GATE_UNIT_HALF_THICKNESS: f32 = 0.05;

fn selector_input_entity_mut(total_inputs: usize) -> impl GateFanEntityMut {
    let normal_inputs = total_inputs - 1;
    let height = ((normal_inputs as f32) * 0.5 * GATE_UNIT_SIZE).max(GATE_UNIT_SIZE);
    let half_height = height * 0.5;
    let section_height: f32 = height / (total_inputs as f32);

    move |cmd: &mut EntityCommands, index: usize| {
        if index == 0 {
            let position = Vec3::new(0.0, -half_height, 0.0);
            cmd.insert((
                Name::new("in.cycle"),
                SpatialBundle::from_transform(Transform::from_translation(position)),
            ));
        } else {
            let position = Vec3::new(
                -GATE_UNIT_HALF_SIZE,
                -1.0 * (section_height * (index as f32) - half_height),
                0.0
            );
            cmd.insert((
                Name::new(format!("in.{index}")),
                SpatialBundle::from_transform(Transform::from_translation(position)),
            ));
        }
    }
}

fn fan_entity_mut(kind: GateFan, num_ports: usize) -> impl GateFanEntityMut {
    let height = ((num_ports as f32) * 0.5 * GATE_UNIT_SIZE).max(GATE_UNIT_SIZE);
    let half_height = height * 0.5;
    let section_height: f32 = height / ((num_ports + 1) as f32);

    let x = match kind {
        GateFan::Input => -GATE_UNIT_HALF_SIZE,
        GateFan::Output => GATE_UNIT_HALF_SIZE,
    };
    move |cmd: &mut EntityCommands, index: usize| {
        let position = Vec3::new(
            x,
            -1.0 * (section_height * ((1 + index) as f32) - half_height),
            0.0
        );
        cmd.insert(SpatialBundle::from_transform(Transform::from_translation(position)));
    }
}

fn pbr(
    position: Vec3,
    mesh_handle: Handle<Mesh>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) -> impl Bundle {
    PbrBundle {
        mesh: mesh_handle,
        material: materials.add(StandardMaterial::default()),
        transform: Transform::from_translation(position),
        ..Default::default()
    }
}
