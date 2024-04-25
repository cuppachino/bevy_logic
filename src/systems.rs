use bevy::prelude::*;
use bevy_trait_query::One;
use crate::{
    components::{ GateFan, Wire },
    logic::{ signal::Signal, LogicGate },
    prelude::{ GateOutput, LogicGateFans },
    resources::LogicGraph,
};

/// A system that evaluates the [`LogicGraph`] resource and updates all entities in a single step.
///
/// This propagates signals through [`Signal`] and [`Wire`] components.
pub fn step_logic(
    logic_graph: Res<LogicGraph>,
    mut logic_entities: Query<(&LogicGateFans, One<&mut dyn LogicGate>)>,
    gate_outputs: Query<&GateOutput>,
    mut gate_fans: Query<&mut Signal, With<GateFan>>,
    mut wires: Query<(&mut Signal, &Wire), Without<GateFan>>
) {
    let sorted = logic_graph.sorted();

    for &entity in sorted.iter() {
        // Get the GATE.
        let (fans, mut gate) = logic_entities
            .get_mut(entity)
            .expect("Entity does not exist or does not have a LogicGateFans or dyn LogicGate");

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

            // Grab the out-going wires from this output.
            let out_going_wires = &gate_outputs
                .get(*entity)
                .expect("GateOutput does not exist").wires;

            // Update the wire signals.
            for entity in out_going_wires.iter() {
                let (mut wire_signal, wire) = wires.get_mut(*entity).expect("Wire does not exist");
                *wire_signal = signal;

                if let Ok(mut input_signal) = gate_fans.get_mut(wire.to) {
                    *input_signal = signal;
                }
            }
        }
    }
}
