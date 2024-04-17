// use bevy::prelude::*;

// mod camera_rig;
// mod components;
// mod cursor;
// mod logic;
// mod materials;

// pub mod prelude {
//     pub use crate::components::prelude::*;
// }

// fn main() {
//     let mut app = App::new();

//     // external plugins
//     app.add_plugins(DefaultPlugins).insert_resource(
//         ClearColor(Color::rgba_linear(0.22, 0.402, 0.598, 1.0))
//     );

//     // crate plugins
//     app.add_plugins((
//         camera_rig::CameraRigPlugin,
//         cursor::CursorPlugin,
//         logic::LogicSimulationPlugin,
//         materials::MaterialsPlugin,
//     ));

//     // main systems
//     app.add_systems(Startup, setup);

//     // run
//     app.run();
// }

// use logic::{ commands::{ LogicCommandsExt, SpawnSourcesAndSinks }, sensors::UiClickSensor };
// use bevy_logic::{ SpawnDemoLights, gate_mesh };
// use materials::logic_gate_material::LogicGateMaterial;
// use self::logic::prelude::*;

// pub fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<LogicGateMaterial>>
// ) {
//     commands.spawn_demo_lights();

//     /*  let mut battery = commands.battery(true);
//     battery.insert(MaterialMeshBundle {
//         mesh: meshes.add(gate_mesh(0, 1)),
//         transform: Transform::from_xyz(-2.0, 0.0, 2.0),
//         material: materials.add(LogicGateMaterial::from(Color::WHITE)),
//         ..default()
//     });
//     let battery = battery.downgrade();

//     let mut not_gate = commands.not_gate();
//     not_gate.insert((
//         MaterialMeshBundle {
//             mesh: meshes.add(gate_mesh(1, 1)),
//             transform: Transform::from_xyz(0.0, 0.0, 2.0),
//             material: materials.add(LogicGateMaterial::from(Color::YELLOW)),
//             ..default()
//         },
//     ));
//     let not_gate = not_gate.downgrade();

//     commands.wire(battery.source(0).unwrap(), not_gate.sink(0).unwrap());

//     let mut not_gate2 = commands.not_gate();
//     not_gate2.insert((
//         MaterialMeshBundle {
//             mesh: meshes.add(gate_mesh(1, 1)),
//             transform: Transform::from_xyz(2.0, 0.0, 2.0),
//             material: materials.add(LogicGateMaterial::from(Color::YELLOW)),
//             ..default()
//         },
//     ));
//     let not_gate2 = not_gate2.downgrade();

//     commands.wire(not_gate.source(0).unwrap(), not_gate2.sink(0).unwrap());

//     let mut and_gate = commands.and_gate();
//     and_gate.insert((
//         MaterialMeshBundle {
//             mesh: meshes.add(gate_mesh(2, 1)),
//             transform: Transform::from_xyz(4.0, 2.0, 2.0),
//             material: materials.add(LogicGateMaterial::from(Color::BEIGE)),
//             ..default()
//         },
//     ));
//     let and_gate = and_gate.downgrade();

//     commands.wire(battery.source(0).unwrap(), and_gate.sink(0).unwrap());
//     commands.wire(not_gate2.source(0).unwrap(), and_gate.sink(1).unwrap());

//     let mut not_gate3 = commands.not_gate();
//     not_gate3.insert((
//         MaterialMeshBundle {
//             mesh: meshes.add(gate_mesh(1, 1)),
//             transform: Transform::from_xyz(0.0, -2.0, 2.0),
//             material: materials.add(LogicGateMaterial::from(Color::YELLOW.with_a(0.5))),
//             ..default()
//         },
//     ));
//     let not_gate3 = not_gate3.downgrade();

//     let mut node = commands.node();
//     node.insert((
//         MaterialMeshBundle {
//             mesh: meshes.add(bevy::math::primitives::Circle::new(0.15)),
//             transform: Transform::from_xyz(0.0, -1.5, 2.0),
//             material: materials.add(LogicGateMaterial::from(Color::WHITE)),
//             ..default()
//         },
//     ));
//     let node = node.downgrade();

//     commands.wire(not_gate3.source(0).unwrap(), node.sink(0).unwrap());
//     commands.wire(node.source(0).unwrap(), not_gate3.sink(0).unwrap()); */

//     {
//         let mut cmd = commands.logic();

//         //

//         let ui_button_a = {
//             let mut button = cmd.spawn((
//                 UiClickSensor::default(),
//                 ButtonBundle {
//                     style: Style {
//                         width: Val::Px(150.0),
//                         height: Val::Px(65.0),
//                         border: UiRect::all(Val::Px(5.0)),
//                         // horizontally center child text
//                         justify_content: JustifyContent::Center,
//                         // vertically center child text
//                         align_items: AlignItems::Center,
//                         ..default()
//                     },
//                     border_color: BorderColor(Color::BLACK),
//                     background_color: Color::rgb(0.15, 0.15, 0.15).into(),
//                     ..default()
//                 },
//             ));
//             button.with_children(|parent| {
//                 parent.spawn(
//                     TextBundle::from_section("Button", TextStyle {
//                         font_size: 40.0,
//                         color: Color::rgb(0.9, 0.9, 0.9),
//                         ..Default::default()
//                     })
//                 );
//             });
//             button.with_output_bundles(
//                 vec![SpatialBundle::from_transform(Transform::from_xyz(-150.0, 5.0, 0.0))]
//             );
//             button.downgrade()
//         };

//         let ui_button_b = {
//             let mut button = cmd.spawn((
//                 UiClickSensor::default(),
//                 ButtonBundle {
//                     style: Style {
//                         width: Val::Px(150.0),
//                         height: Val::Px(65.0),
//                         border: UiRect::all(Val::Px(5.0)),
//                         // horizontally center child text
//                         justify_content: JustifyContent::Center,
//                         // vertically center child text
//                         align_items: AlignItems::Center,
//                         position_type: PositionType::Absolute,
//                         top: Val::Px(300.0),
//                         ..default()
//                     },
//                     border_color: BorderColor(Color::BLACK),
//                     background_color: Color::rgb(0.15, 0.15, 0.15).into(),
//                     ..default()
//                 },
//             ));
//             button.with_children(|parent| {
//                 parent.spawn(
//                     TextBundle::from_section("Button", TextStyle {
//                         font_size: 40.0,
//                         color: Color::rgb(0.9, 0.9, 0.9),
//                         ..Default::default()
//                     })
//                 );
//             });
//             button.with_output_bundles(
//                 vec![SpatialBundle::from_transform(Transform::from_xyz(-150.0, -310.0, 0.0))]
//             );
//             button.downgrade()
//         };

//         //

//         let nor_a = {
//             let mut nor_a = cmd.spawn((
//                 OrGate::NOR,
//                 MaterialMeshBundle {
//                     mesh: meshes.add(bevy::math::primitives::Circle::new(0.5)),
//                     transform: Transform::from_xyz(0.0, -0.0, 2.0),
//                     material: materials.add(LogicGateMaterial::from(Color::CYAN)),
//                     ..default()
//                 },
//             ));
//             nor_a.with_sinks(2).with_sources(1);
//             nor_a.downgrade()
//         };

//         let nor_b = {
//             let mut nor_b = cmd.spawn((
//                 OrGate::NOR,
//                 MaterialMeshBundle {
//                     mesh: meshes.add(bevy::math::primitives::Circle::new(0.5)),
//                     transform: Transform::from_xyz(0.0, -2.0, 2.0),
//                     material: materials.add(LogicGateMaterial::from(Color::CYAN)),
//                     ..default()
//                 },
//             ));
//             nor_b.with_sinks(2).with_sources(1);
//             nor_b.downgrade()
//         };

//         let node_a = {
//             let mut node = cmd.node();
//             node.insert(MaterialMeshBundle {
//                 mesh: meshes.add(bevy::math::primitives::Circle::new(0.15)),
//                 transform: Transform::from_xyz(2.0, -0.0, 2.0),
//                 material: materials.add(LogicGateMaterial::from(Color::WHITE)),
//                 ..default()
//             });
//             node.downgrade()
//         };

//         let node_b = {
//             let mut node = cmd.node();
//             node.insert(MaterialMeshBundle {
//                 mesh: meshes.add(bevy::math::primitives::Circle::new(0.15)),
//                 transform: Transform::from_xyz(2.0, -2.0, 2.0),
//                 material: materials.add(LogicGateMaterial::from(Color::WHITE)),
//                 ..default()
//             });
//             node.downgrade()
//         };

//         // cyclic not-gate
//         {
//             let not = {
//                 let mut not = cmd.spawn((
//                     NotGate,
//                     MaterialMeshBundle {
//                         mesh: meshes.add(gate_mesh(1, 1)),
//                         transform: Transform::from_xyz(4.0, 0.0, 2.0),
//                         material: materials.add(LogicGateMaterial::from(Color::YELLOW)),
//                         ..default()
//                     },
//                 ));
//                 not.with_sinks(1).with_sources(1);
//                 not.downgrade()
//             };

//             let node_c = {
//                 let mut node = cmd.node();
//                 node.insert(MaterialMeshBundle {
//                     mesh: meshes.add(bevy::math::primitives::Circle::new(0.15)),
//                     transform: Transform::from_xyz(6.5, 2.0, 2.0),
//                     material: materials.add(LogicGateMaterial::from(Color::WHITE)),
//                     ..default()
//                 });
//                 node.downgrade()
//             };

//             let node_d = {
//                 let mut node = cmd.node();
//                 node.insert(MaterialMeshBundle {
//                     mesh: meshes.add(bevy::math::primitives::Circle::new(0.15)),
//                     transform: Transform::from_xyz(4.0, 2.0, 2.0),
//                     material: materials.add(LogicGateMaterial::from(Color::WHITE)),
//                     ..default()
//                 });
//                 node.downgrade()
//             };

//             cmd.wire(not.source(0).unwrap(), node_c.sink(0).unwrap());
//             cmd.wire(node_c.source(0).unwrap(), node_d.sink(0).unwrap());
//             cmd.wire(node_d.source(0).unwrap(), not.sink(0).unwrap());
//         }

//         commands.wire(nor_a.source(0).unwrap(), node_a.sink(0).unwrap());
//         commands.wire(nor_b.source(0).unwrap(), node_b.sink(0).unwrap());

//         commands.wire(node_a.source(0).unwrap(), nor_b.sink(0).unwrap());
//         commands.wire(node_b.source(0).unwrap(), nor_a.sink(0).unwrap());

//         commands.wire(ui_button_a.source(0).unwrap(), nor_a.sink(1).unwrap());
//         commands.wire(ui_button_b.source(0).unwrap(), nor_b.sink(1).unwrap());

//         // commands.wire(battery.source(0).unwrap(), nor_a.sink(1).unwrap());
//     }
// }

fn main() {}
