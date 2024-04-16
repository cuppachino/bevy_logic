mod utils;
mod gui;
mod camera_rig;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::prelude::*;
use bevy_logic::{
    commands::LogicExt,
    components::{ AndGate, Battery, LogicFans, NotGate, OrGate },
    prelude::*,
};
use camera_rig::CameraRigPlugin;
use gui::{ GateIcon, GuiPlugin, LogicGateIcons };

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, WorldInspectorPlugin::new(), CameraRigPlugin, GuiPlugin))
        .insert_resource(ClearColor(Color::rgba_linear(0.22, 0.402, 0.598, 1.0)))
        .add_plugins(LogicSimulationPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, colorize_logic_gates)
        .run();
}

fn setup(world: &mut World) {
    let battery_bundle = pbr_bundle(world, GateIcon::Battery, Vec2::new(-4.0, 0.0));
    let or_bundle = pbr_bundle(world, GateIcon::Or, Vec2::new(-2.0, 0.0));
    let and_bundle_a = pbr_bundle(world, GateIcon::And, Vec2::new(0.0, 0.0));
    let and_bundle_b = pbr_bundle(world, GateIcon::And, Vec2::new(2.0, -2.0));
    let not_bundle_a = pbr_bundle(world, GateIcon::Not, Vec2::new(2.0, 0.0));
    let not_bundle_b = pbr_bundle(world, GateIcon::Not, Vec2::new(4.0, 0.0));

    let or_gate = world
        .spawn_gate((Name::new("OR"), OrGate::default()))
        .with_outputs(2)
        .with_inputs(2)
        .insert_bundle(or_bundle)
        .build();
    let not_gate_a = world
        .spawn_gate((Name::new("NOT"), NotGate))
        .with_inputs(1)
        .with_outputs(1)
        .insert_bundle(not_bundle_a.clone())
        .build();
    let not_gate_b = world
        .spawn_gate((Name::new("NOT"), NotGate))
        .with_inputs(1)
        .with_outputs(1)
        .insert_bundle(not_bundle_b)
        .build();
    let and_gate_a = world
        .spawn_gate((Name::new("AND"), AndGate))
        .with_inputs(2)
        .with_outputs(1)
        .insert_bundle(and_bundle_a.clone())
        .build();
    let and_gate_b = world
        .spawn_gate((Name::new("AND"), AndGate))
        .with_inputs(2)
        .with_outputs(1)
        .insert_bundle(and_bundle_b)
        .build();
    let battery = world
        .spawn_gate((Name::new("BAT"), Battery::ON))
        .with_outputs(1)
        .insert_bundle(battery_bundle)
        .build();
    let wires = vec![
        world.spawn_wire(&not_gate_a, 0, &not_gate_a, 0).downgrade(),
        world.spawn_wire(&not_gate_a, 0, &not_gate_b, 0).downgrade(),
        world.spawn_wire(&not_gate_a, 0, &or_gate, 0).downgrade(),
        world.spawn_wire(&battery, 0, &or_gate, 1).downgrade(),
        world.spawn_wire(&battery, 0, &and_gate_a, 0).downgrade(),
        world.spawn_wire(&or_gate, 0, &and_gate_a, 1).downgrade(),
        world.spawn_wire(&and_gate_a, 0, &and_gate_b, 0).downgrade(),
        world.spawn_wire(&not_gate_a, 0, &and_gate_b, 1).downgrade()
    ];

    let mut sim = world.resource_mut::<LogicGraph>();

    sim.add_data(vec![or_gate, not_gate_a, not_gate_b, and_gate_a, and_gate_b])
        .add_data(battery)
        .add_data(wires)
        .compile();

    let ordered = sim.sorted().to_vec();

    for entity in ordered {
        let name = world.get::<Name>(entity).unwrap();
        // sim.graph.edges_directed(node, bevy::utils::petgraph::Direction::Outgoing);
        println!("{:?}", name);
    }
}

fn pbr_bundle(
    world: &mut World,
    gate_icon: GateIcon,
    position: Vec2
) -> MaterialMeshBundle<StandardMaterial> {
    let mut meshes = world.resource_mut::<Assets<Mesh>>();
    let mesh = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));

    let icons = world.resource::<LogicGateIcons>();
    let icon_handle = icons.get(gate_icon);

    let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(icon_handle),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    PbrBundle {
        mesh,
        material,
        transform: Transform::from_translation(position.extend(0.0)),
        ..Default::default()
    }
}

fn colorize_logic_gates(
    query: Query<(&LogicFans, &Handle<StandardMaterial>)>,
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
