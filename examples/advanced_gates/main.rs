//! This example showcases how one might design "advanced" gates with
//! functionality beyond what is typically permitted in real life.
//!
//! In this example, we design a `Selector` and a `Counter`, and spawn a demo circuit
//! that works as a permutational keypad. For simplicity, we will ignore incorrect key presses and only
//! require that the correct keys are pressed in order; however, reset functionality could easily
//! be implemented with a handful of OR gates.
//!
//! The valid code will be `2, 1, 1, 8`, in order.

use bevy::{ ecs::system::EntityCommands, prelude::* };
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_logic::{ logic::builder::{ GateData, GateFanEntityMut, Known }, prelude::* };
use itertools::Itertools;

mod camera_rig;
mod visual;
mod triangulation;
use visual::*;

use crate::camera_rig::CameraRigPlugin;

mod custom_gates {
    use derive_new::new;

    use super::*;

    /// A selector acts like a state machine. It has an even number of standard inputs and outputs, and 1 "cycle" input.
    ///
    /// Selectors should *always* have *exactly* 1 more input
    /// than outputs.
    ///
    /// Higher inputs take priority over lower ones.
    ///
    /// The cycle input is ignored if it was previously evaluated and the signal is still truthy.
    #[derive(Component, Clone, Debug, Default, Reflect)]
    pub struct Selector {
        stale_cycle_input: bool,
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

            if
                let Some((index, signal)) = inputs
                    .iter()
                    .rev()
                    .find_position(|input| { input.is_truthy() })
            {
                // flip the index
                let index = inputs.len() - 1 - index;

                if index == 0 {
                    if self.stale_cycle_input {
                        return;
                    } else {
                        self.stale_cycle_input = true;
                    }

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
                    // self.last_input.replace(index);
                    outputs.set_all(Signal::OFF);
                    outputs
                        .get_mut(index - 1)
                        .unwrap()
                        .replace(*signal);
                }
                return;
            } else {
                self.stale_cycle_input = false;
            }

            if outputs.iter().all(|s| s.is_falsy()) {
                if let Some(first) = outputs.first_mut() {
                    first.turn_on();
                }
            }
        }
    }
    /// A counter counts input signal pulses up to the target value, and
    /// only emits a signal once the target is met, unless the `signal_strength` is true,
    /// in which case the output signal is a value between `0.0..1.0` representing the
    /// completion of the counter.
    ///
    /// Counters have 1 standard input, 1 reset input, and 1 output.
    ///
    /// The first input is used as the "reset" input.
    #[derive(new, Component, Clone, Debug, Reflect)]
    pub struct Counter {
        /// The current count.
        pub current: i8,
        /// The max/target count.
        pub target: i8,
        /// If true, output a value between `0.0..1.0` representing the completion state of the counter.
        pub signal_strength: bool,
        /// Stores the previously evaluated inputs so that solid signals do not continuously trigger their input.
        #[new(default)]
        #[reflect(ignore)]
        stale_inputs: [bool; 2],
    }

    impl Default for Counter {
        fn default() -> Self {
            Self {
                current: 0,
                target: 10,
                signal_strength: false,
                stale_inputs: [false; 2],
            }
        }
    }

    impl Counter {
        /// Increments the counter if [`Self::current`] is less than [`Self::target`].
        #[inline]
        pub fn increment(&mut self) {
            if self.current < self.target {
                self.current += 1;
            }
        }

        /// Decrement the counter if [`Self::curent`] is greater than `0`.
        #[inline]
        pub fn decrement(&mut self) {
            if self.current > 0 {
                self.current -= 1;
            }
        }

        /// Reset the counter to `0`.
        #[inline]
        pub fn reset(&mut self) {
            self.current = 0;
        }

        /// Returns `Self::current / Self::target` as a [`Signal`].
        pub fn signal_strength(&self) -> Signal {
            ((self.current as f32) / (self.target as f32)).into()
        }

        /// Returns a [`Signal::Digital`] that is true if [`Self::current`] is equal to [`Self::target`]
        pub fn signal_on_off(&self) -> Signal {
            (self.current == self.target).into()
        }
    }

    impl LogicGate for Counter {
        fn evaluate(&mut self, inputs: &[Signal], outputs: &mut [Signal]) {
            if let Some((index, signal)) = inputs.iter().find_position(|s| s.is_truthy()) {
                if self.stale_inputs[index] {
                    return;
                } else {
                    self.stale_inputs[index] = true;
                }

                match index {
                    0 => self.reset(),
                    1 => {
                        if signal.is_sign_negative() {
                            self.decrement();
                        } else {
                            self.increment();
                        }
                    }
                    _ => panic!("Counter has too many inputs!"),
                }
            } else {
                self.stale_inputs.iter_mut().for_each(|b| {
                    *b = false;
                });
            }

            let output = outputs.get_mut(0).expect("Counter should have one output!");
            if self.signal_strength {
                output.replace(self.signal_strength());
            } else {
                let signal = self.signal_on_off();
                output.replace(signal);
            }
        }
    }
}

fn main() {
    const TICKS_PER_SECOND: f64 = 30.0;

    let mut app = App::new();

    // Register the [`Selector`] as a `LogicGate`.
    app.register_logic_gate::<custom_gates::Selector>().register_logic_gate::<custom_gates::Counter>();

    // Add the `LogicSimulationPlugin`.
    app.add_plugins((
        DefaultPlugins,
        CameraRigPlugin,
        WorldInspectorPlugin::new(),
        LogicSimulationPlugin,
    ))
        .insert_resource(ClearColor(Color::rgba_linear(0.22, 0.402, 0.598, 1.0)))
        .insert_resource(Time::<LogicStep>::from_hz(TICKS_PER_SECOND))
        .add_systems(Startup, systems::setup_scene)
        .add_systems(Update, gizmo_wires)
        .add_systems(
            Update,
            systems::reserve_ui_button_signal.before(LogicSystemSet::PropagateNoEval)
        )
        .add_systems(
            LogicUpdate,
            systems::release_ui_button_signals.after(LogicSystemSet::StepLogic)
        )
        .run();
}

mod systems {
    use super::*;

    /// Scene setup.
    pub fn setup_scene(
        mut commands: Commands,
        mut sim: ResMut<LogicGraph>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>
    ) {
        // Spawn a light
        commands.spawn(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
            point_light: PointLight {
                intensity: 150000.0,
                ..Default::default()
            },
            ..Default::default()
        });

        // Create a Selector with 5 states (and one cycle input).
        let selector = helpers::spawn_selector(
            &mut commands,
            &mut meshes,
            &mut materials,
            5,
            Vec2::new(-1.0, 0.0)
        );

        // Prepare 4 AND gates with 2 inputs each.
        let and_a = helpers::spawn_and_gate(
            &mut commands,
            &mut meshes,
            &mut materials,
            2,
            Vec2::new(3.0, 3.0)
        );
        let and_b = helpers::spawn_and_gate(
            &mut commands,
            &mut meshes,
            &mut materials,
            2,
            Vec2::new(3.0, 1.0)
        );
        let and_c = helpers::spawn_and_gate(
            &mut commands,
            &mut meshes,
            &mut materials,
            2,
            Vec2::new(3.0, -1.0)
        );
        let and_d = helpers::spawn_and_gate(
            &mut commands,
            &mut meshes,
            &mut materials,
            2,
            Vec2::new(3.0, -3.0)
        );

        // Spawn "one-shot" Counters - Counters that count to 1 and then reset.
        let counter_a = helpers::spawn_counter(
            &mut commands,
            &mut meshes,
            &mut materials,
            1,
            false,
            Vec2::new(1.0, 2.0)
        );
        sim.add_data(commands.spawn_wire(&counter_a, 0, &counter_a, 0).downgrade());

        let counter_bc = helpers::spawn_counter(
            &mut commands,
            &mut meshes,
            &mut materials,
            1,
            false,
            Vec2::new(1.0, 0.0)
        );
        sim.add_data(commands.spawn_wire(&counter_bc, 0, &counter_bc, 0).downgrade());

        let counter_d = helpers::spawn_counter(
            &mut commands,
            &mut meshes,
            &mut materials,
            1,
            false,
            Vec2::new(1.0, -2.0)
        );
        sim.add_data(commands.spawn_wire(&counter_d, 0, &counter_d, 0).downgrade());

        // Spawn the keypad.
        let keypad = helpers::spawn_keypad_ui(&mut commands);

        // Spawn a "reset" button wired to the first state of the Selector.
        let reset_button = commands
            .spawn((
                camera_rig::UiWorldPosition::default(),
                OutputBundle::default(),
                NoEvalOutput,
                ButtonBundle {
                    background_color: Color::GRAY.into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(20.0),
                        bottom: Val::Px(20.0),
                        padding: UiRect::axes(Val::Px(24.0), Val::Px(8.0)),
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|button| {
                button.spawn(
                    TextBundle::from_section("RESET", TextStyle {
                        font_size: 32.0,
                        ..default()
                    })
                );
            })
            .id();
        commands.spawn_no_eval_wire(reset_button, selector.input(1)); // button out.0 -> selector std.in 0

        // Our UI button's don't take any inputs, do not need evaluation, and
        // do not need to be sorted by the [`LogicGraph`]. We'll wire each keypad
        // to a "one-shot" counter and then wire the counters to the appropriate AND gate.
        commands.spawn_no_eval_wire(keypad[1], counter_a.input(1)); // 2 -> counter std.in 0
        commands.spawn_no_eval_wire(keypad[0], counter_bc.input(1)); // 1 -> counter std.in 0
        commands.spawn_no_eval_wire(keypad[7], counter_d.input(1)); // 8 -> counter std.in 0

        // We still add logic gates because they always need to be evaluated.
        sim.add_data(
            vec![
                // Wire each selector output to an AND gate.
                commands.spawn_wire(&selector, 0, &and_a, 0).downgrade(), // 0 -> and 0
                commands.spawn_wire(&selector, 1, &and_b, 0).downgrade(), // 1 -> and 0
                commands.spawn_wire(&selector, 2, &and_c, 0).downgrade(), // 2 -> and 0
                commands.spawn_wire(&selector, 3, &and_d, 0).downgrade(), // 3 -> and 0

                // Wire each counter output to an AND gate.
                commands.spawn_wire(&counter_a, 0, &and_a, 1).downgrade(), // out 0 -> and 0
                commands.spawn_wire(&counter_bc, 0, &and_b, 1).downgrade(), // out 0 -> and 0
                commands.spawn_wire(&counter_bc, 0, &and_c, 1).downgrade(), // out 0 -> and 0
                commands.spawn_wire(&counter_d, 0, &and_d, 1).downgrade(), // out 0 -> and 0

                // Wire each and gate to the next selector INPUT except for the last one.
                commands.spawn_wire(&and_a, 0, &selector, 2).downgrade(), // and a -> selector std.in 1
                commands.spawn_wire(&and_b, 0, &selector, 3).downgrade(), // and b -> selector std.in 2
                commands.spawn_wire(&and_c, 0, &selector, 4).downgrade(), // and c -> selector std.in 3
                commands.spawn_wire(&and_d, 0, &selector, 5).downgrade() // and d -> selector std.in 4
            ]
        )
            .add_data(vec![selector, and_a, and_b, and_c, and_d, counter_a, counter_bc, counter_d])
            .compile();
    }

    /// This system allows players to activate a button immediately and leave the button
    /// on until the logic step system has ran.
    ///
    /// Hover/None effects are applied as long as the button hasn't been pressed yet.
    pub fn reserve_ui_button_signal(
        mut query_buttons: Query<
            (&Interaction, &mut BackgroundColor, &mut Signal),
            (Changed<Interaction>, With<Button>)
        >
    ) {
        for (interaction, mut color, mut signal) in &mut query_buttons {
            match *interaction {
                Interaction::Pressed => {
                    *color = Color::GREEN.into();
                    *signal = Signal::ON;
                }
                Interaction::Hovered => {
                    if signal.is_falsy() {
                        *color = Color::DARK_GRAY.into();
                    }
                }
                Interaction::None => {
                    if signal.is_falsy() {
                        *color = Color::GRAY.into();
                    }
                }
            }
        }
    }

    /// Runs after the logic step function and updates the state of the button to match.
    pub fn release_ui_button_signals(
        mut query_buttons: Query<
            (&Interaction, &mut BackgroundColor, &mut Signal),
            (Changed<Interaction>, With<Button>)
        >
    ) {
        for (interaction, mut color, mut signal) in &mut query_buttons {
            match *interaction {
                Interaction::Pressed => {
                    *color = Color::GREEN.into();
                    *signal = Signal::ON;
                }
                Interaction::Hovered => {
                    *color = Color::DARK_GRAY.into();
                    *signal = Signal::OFF;
                }
                Interaction::None => {
                    *color = Color::GRAY.into();
                    *signal = Signal::OFF;
                }
            }
        }
    }
}

mod helpers {
    use super::*;

    /// Spawns 9 UI buttons in the shape of a keypad and returns their entity IDs.
    ///
    /// The buttons have an [`OutputBundle`] and [`NoEvalOutput`] component that lets
    /// them act as standalone outputs and forward their signal without "evaluating".
    pub fn spawn_keypad_ui(commands: &mut Commands) -> [Entity; 9] {
        let mut entities = [Entity::PLACEHOLDER; 9];
        commands
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_template_columns: vec![RepeatedGridTrack::auto(3)],
                    grid_template_rows: vec![RepeatedGridTrack::auto(3)],
                    aspect_ratio: Some(1.0),
                    height: Val::Percent(10.0),
                    row_gap: Val::Px(8.0),
                    column_gap: Val::Px(8.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|root| {
                for (i, entity) in entities.iter_mut().enumerate() {
                    *entity = root
                        .spawn((
                            camera_rig::UiWorldPosition::default(),
                            OutputBundle::default(),
                            NoEvalOutput,
                            ButtonBundle {
                                background_color: Color::GRAY.into(),
                                style: Style {
                                    aspect_ratio: Some(1.0),
                                    height: Val::Percent(100.0),
                                    display: Display::Grid,
                                    align_items: AlignItems::Center,
                                    align_content: AlignContent::Center,
                                    justify_content: JustifyContent::Center,
                                    justify_items: JustifyItems::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        ))
                        .with_children(|button| {
                            button.spawn(
                                TextBundle::from_section(format!("{}", i + 1), TextStyle {
                                    font_size: 32.0,
                                    ..default()
                                })
                            );
                        })
                        .id();
                }
            });

        entities
    }

    /// Spawns a 3D Selector.
    pub fn spawn_selector(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        states: usize,
        position: Vec2
    ) -> GateData<Known, Known> {
        commands
            .spawn_gate((
                Name::new("Selector"),
                custom_gates::Selector::default(),
                pbr(position.extend(0.0), meshes.add(build_mesh(states, states, 1)), materials),
            ))
            .build_inputs(states + 1, selector_input_entity_mut(states + 1))
            .build_outputs(states, fan_entity_mut(GateFan::Output, states))
            .build()
    }

    /// Spawns a 3D Counter.
    pub fn spawn_counter(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        max_count: i8,
        signal_strength: bool,
        position: Vec2
    ) -> GateData<Known, Known> {
        commands
            .spawn_gate((
                Name::new("Counter"),
                custom_gates::Counter::new(0, max_count, signal_strength),
                pbr(position.extend(0.0), meshes.add(build_mesh(1, 1, 1)), materials),
            ))
            .build_inputs(2, selector_input_entity_mut(2))
            .build_outputs(1, fan_entity_mut(GateFan::Output, 1))
            .build()
    }

    /// Spawns a 3D AND gate.
    pub fn spawn_and_gate(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        inputs: usize,
        position: Vec2
    ) -> GateData<Known, Known> {
        commands
            .spawn_gate((
                Name::new("AND"),
                AndGate,
                pbr(position.extend(0.0), meshes.add(build_mesh(inputs, 1, 0)), materials),
            ))
            .build_inputs(inputs, fan_entity_mut(GateFan::Input, inputs))
            .build_outputs(1, fan_entity_mut(GateFan::Output, 1))
            .build()
    }

    pub const GATE_UNIT_SIZE: f32 = 1.0;
    pub const GATE_UNIT_HALF_SIZE: f32 = 0.5;
    pub const GATE_UNIT_HALF_THICKNESS: f32 = 0.05;

    /// Position the input fans of a [`Selector`] logic gate.
    pub fn selector_input_entity_mut(total_inputs: usize) -> impl GateFanEntityMut {
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

    /// Position the fans of a generic logic gate.
    pub fn fan_entity_mut(kind: GateFan, num_ports: usize) -> impl GateFanEntityMut {
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

    /// Create a standard pbr bundle from a position and mesh_handle.
    pub fn pbr(
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
}
