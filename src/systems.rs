use bevy::prelude::*;
use bevy_trait_query::One;
use crate::{
    components::{ GateFan, Wire },
    logic::{ signal::Signal, LogicGate },
    prelude::LogicFans,
    resources::LogicGraph,
};

pub mod prelude {}

pub fn step_logic(
    logic_graph: Res<LogicGraph>,
    logic_entities: Query<(&LogicFans, One<&dyn LogicGate>)>,
    mut gate_fans: Query<&mut Signal, With<GateFan>>,
    mut wires: Query<(&mut Signal, &Wire), Without<GateFan>>
) {
    let sorted = logic_graph.sorted();

    for &entity in sorted.iter() {
        // Get the GATE.
        let (fans, gate) = logic_entities
            .get(entity)
            .expect("Entity does not exist or does not have a LogicFans/LogicGate");

        // Collect its fan input signals.
        let input_signals = fans.inputs
            .iter()
            .filter_map(|&input| {
                let input = input?;
                let signal = gate_fans.get(input).ok().copied();
                signal
            })
            .collect::<Vec<_>>();

        // Collect its fan outputs entities, and create an empty signals vec matching the number of outputs.
        let (output_entities, mut output_signals): (Vec<_>, Vec<_>) = fans.outputs
            .iter()
            .filter_map(|&output| {
                let output = output?;
                let signal = gate_fans.get(output).ok().copied()?;
                Some((output, signal))
            })
            .unzip();

        // Evaluate the gate.
        gate.evaluate(&input_signals, &mut output_signals);

        // Update the output signals.
        for (entity, signal) in output_entities.iter().zip(output_signals) {
            if let Ok(mut output_signal) = gate_fans.get_mut(*entity) {
                *output_signal = signal;
            }
            // Update the wire signals.
            for (mut wire_signal, wire) in wires.iter_mut() {
                if wire.from == *entity {
                    *wire_signal = signal;

                    if let Ok(mut input_signal) = gate_fans.get_mut(wire.to) {
                        *input_signal = signal;
                    }
                }
            }
        }
    }
}
