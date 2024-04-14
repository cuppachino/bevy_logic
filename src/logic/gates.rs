use bevy::prelude::*;
use bevy_trait_query::One;
use derive_new::new;
use crate::Desync;

use super::signal::Signal;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{ LogicGatePlugin, NotGate, AndGate, OrGate, LogicNode, Battery };
}

/// A trait that defines the behavior of a logic gate.
#[bevy_trait_query::queryable]
pub trait LogicGate {
    /// Evaluate the current state of inputs (in order), and update the outputs (in order).
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Source]);
}

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogicSystemSet {
    SpawnEntities,
    PropagateSignals,
    EvaluateGates,
}

/// A plugin that registers logic gates and simulates their behavior.
pub struct LogicGatePlugin;

impl Plugin for LogicGatePlugin {
    fn build(&self, app: &mut App) {
        // We must import this trait in order to register our components.
        // If we don't register them, they will be invisible to the game engine.
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn LogicGate, Battery>()
            .register_component_as::<dyn LogicGate, AndGate>()
            .register_component_as::<dyn LogicGate, NotGate>()
            .register_component_as::<dyn LogicGate, OrGate>()
            .register_component_as::<dyn LogicGate, LogicNode>();

        app.configure_sets(
            Update,
            (
                LogicSystemSet::SpawnEntities,
                LogicSystemSet::PropagateSignals,
                LogicSystemSet::EvaluateGates,
            ).chain()
        );

        app.add_systems(Update, propagate_logic_signals.in_set(LogicSystemSet::PropagateSignals));
        app.add_systems(
            Update,
            evaluate_logic_gates
                .in_set(LogicSystemSet::EvaluateGates)
                .after(LogicSystemSet::PropagateSignals)
        );
        app.add_systems(Update, debug_logic_components.after(LogicSystemSet::EvaluateGates));

        // // Set the Fixed Timestep interval to 96 Hz
        // app.insert_resource(Time::<Fixed>::from_seconds(0.5));
    }
}

use super::components::{ Source, Sink, Wire };

fn propagate_logic_signals(
    mut query_wires: Query<(&mut Wire, Option<&mut Desync>)>,
    query_sources: Query<&Source>,
    mut query_sinks: Query<&mut Sink>
) {
    for (mut wire, desync) in query_wires.iter_mut() {
        if let Some(mut desync) = desync {
            if desync.0 > 0 {
                desync.0 -= 1;
                continue;
            }
        }

        let source = query_sources.get(wire.source).unwrap();
        let mut sink = query_sinks.get_mut(wire.sink).unwrap();
        sink.signal = source.signal;
        wire.signal = source.signal;
    }
}

fn evaluate_logic_gates(
    mut query_gates: Query<(One<&dyn LogicGate>, &Children, Option<&mut Desync>)>,
    query_sinks: Query<&Sink>,
    mut query_sources: Query<(Entity, &mut Source)>
) {
    for (gate, children, desync) in query_gates.iter_mut() {
        if let Some(mut desync) = desync {
            if desync.0 > 0 {
                desync.0 -= 1;
                continue;
            }
        }

        let mut inputs = Vec::new();
        let mut output_entities = Vec::new();

        for child in children {
            if let Ok(sink) = query_sinks.get(*child) {
                inputs.push(sink.signal);
            } else if let Ok((entity, _)) = query_sources.get(*child) {
                output_entities.push(entity);
            }
        }

        // construct a vec with the same length as the number of outputs
        let mut outputs = output_entities
            .iter()
            .map(|_| Source::default())
            .collect::<Vec<_>>();

        gate.evaluate(&inputs, &mut outputs);

        // flip it around so we can pop from the end
        outputs.reverse();

        let mut iter = query_sources.iter_many_mut(&output_entities);

        // `fetch_next` is new in bevy 13.0. Hopefully order is still deterministic.
        while let Some((_, mut source)) = iter.fetch_next() {
            if let Some(output) = outputs.pop() {
                *source = output;
            }
        }
    }
}

fn debug_logic_components(
    mut gizmos: Gizmos,
    query_sources: Query<(&Source, &GlobalTransform)>,
    query_sinks: Query<(&Sink, &GlobalTransform)>,
    query_wires: Query<&Wire>
) {
    for wire in query_wires.iter() {
        let source = query_sources
            .get(wire.source)
            .expect("Source does not have a GlobalTransform");
        let sink = query_sinks.get(wire.sink).expect("Sink does not have a GlobalTransform");

        gizmos.line(source.1.translation(), sink.1.translation(), {
            if source.0.signal.is_true() { Color::GREEN } else { Color::RED }
        });
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct NotGate;

impl LogicGate for NotGate {
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Source]) {
        let signal = !inputs.iter().all(Signal::is_true);
        outputs.iter_mut().for_each(|output| {
            output.signal = signal.into();
        });
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct AndGate;

impl LogicGate for AndGate {
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Source]) {
        let signal = inputs.iter().all(Signal::is_true);
        outputs.iter_mut().for_each(|output| {
            output.signal = signal.into();
        });
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct OrGate {
    /// If true, the gate will act as an analog adder,
    /// computing the sum of all inputs.
    pub is_adder: bool,

    /// If true, the gate will be a NOR gate instead of an OR gate.
    pub invert_output: bool,
}

impl Default for OrGate {
    fn default() -> Self {
        Self {
            is_adder: false,
            invert_output: false,
        }
    }
}

impl OrGate {
    pub const NOR: OrGate = OrGate { is_adder: false, invert_output: true };
}

impl LogicGate for OrGate {
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Source]) {
        let signal = if self.is_adder {
            inputs.iter().fold(Signal::OFF, |acc, input| { acc + *input })
        } else {
            inputs.iter().any(Signal::is_true).into()
        };

        let signal = if self.invert_output { !signal } else { signal };

        outputs.iter_mut().for_each(|output| {
            output.signal = signal;
        });
    }
}

/// A battery that emits a constant signal.
#[derive(Component, Clone, Copy, Debug)]
pub struct Battery {
    pub signal: Signal,
}

impl Default for Battery {
    fn default() -> Self {
        Self::MAX
    }
}

#[allow(dead_code)]
impl Battery {
    pub const OFF: Battery = Battery::new(Signal::OFF);
    pub const MAX: Battery = Battery::new(Signal::ON);
    pub const MIN: Battery = Battery::new(Signal::NEG);

    /// Create a new battery with `signal`.
    pub const fn new(signal: Signal) -> Self {
        Self { signal }
    }
}

impl LogicGate for Battery {
    fn evaluate(&self, _: &[Signal], outputs: &mut [Source]) {
        outputs.iter_mut().for_each(|output| {
            output.signal = self.signal;
        });
    }
}

/// A blank node that can be used for routing wires.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LogicNode;

impl LogicGate for LogicNode {
    fn evaluate(&self, inputs: &[Signal], outputs: &mut [Source]) {
        #[cfg(debug_assertions)]
        assert_eq!(
            inputs.len(),
            outputs.len(),
            "LogicNode inputs and outputs must be the same length"
        );

        for (output, input) in outputs.iter_mut().zip(inputs.iter()) {
            output.signal = *input;
        }
    }
}
