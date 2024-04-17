mod utils;
mod gui;
mod camera_rig;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::prelude::*;
use bevy_logic::{
    commands::{ GatePipeWorldMut, LogicExt },
    components::{ AndGate, Battery, GateFan, LogicFans, NotGate, OrGate },
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
        .run();
}

fn gate_pipe(kind: GateFan, len: usize, height: f32) -> impl GatePipeWorldMut {
    #[cfg(debug_assertions)]
    if len == 0 {
        panic!("GatePipe must have at least one input");
    }
    let x: f32 =
        (match kind {
            GateFan::Input => -1.0,
            GateFan::Output => 1.0,
        }) * 0.5;
    let section_height: f32 = height / ((len + 1) as f32);
    let half_height = height / 2.0;
    move |cmd: &mut EntityWorldMut, index: usize| {
        let position = Vec3::new(x, section_height * ((1 + index) as f32) - half_height, 0.0);
        cmd.insert(Transform::from_translation(position));
    }
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
        .build_inputs(2, gate_pipe(GateFan::Input, 2, 1.0))
        .build_outputs(1, gate_pipe(GateFan::Output, 1, 1.0))
        .insert_bundle(or_bundle)
        .build();
    let not_gate_a = world
        .spawn_gate((Name::new("NOT"), NotGate))
        .build_inputs(1, gate_pipe(GateFan::Input, 1, 1.0))
        .build_outputs(1, gate_pipe(GateFan::Output, 1, 1.0))
        .insert_bundle(not_bundle_a.clone())
        .build();
    let not_gate_b = world
        .spawn_gate((Name::new("NOT"), NotGate))
        .build_inputs(1, gate_pipe(GateFan::Input, 1, 1.0))
        .build_outputs(1, gate_pipe(GateFan::Output, 1, 1.0))
        .insert_bundle(not_bundle_b)
        .build();
    let and_gate_a = world
        .spawn_gate((Name::new("AND"), AndGate))
        .build_inputs(2, gate_pipe(GateFan::Input, 2, 1.0))
        .build_outputs(1, gate_pipe(GateFan::Output, 1, 1.0))
        .insert_bundle(and_bundle_a.clone())
        .build();
    let and_gate_b = world
        .spawn_gate((Name::new("AND"), AndGate))
        .build_inputs(2, gate_pipe(GateFan::Input, 2, 1.0))
        .build_outputs(1, gate_pipe(GateFan::Output, 1, 1.0))
        .insert_bundle(and_bundle_b)
        .build();

    let battery = world
        .spawn_gate((Name::new("BAT"), Battery::ON))
        .build_outputs(1, gate_pipe(GateFan::Output, 1, 1.0))
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
