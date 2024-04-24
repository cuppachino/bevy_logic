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

// /// A system that reads [`LogicEvent`]s and updates the [`LogicGraph`] resource and
// /// associated [`GateOutput`] components.
// ///
// /// See [`LogicEventPlugin`] for more information.
// ///
// /// [`LogicEvent`]: crate::events::LogicEventPlugin
// pub fn read_logic_events(
//     mut logic_ev_rd: EventReader<LogicEvent>,
//     mut sim: ResMut<LogicGraph>,
//     mut gate_outputs: Query<&mut GateOutput>,
//     wires: Query<&Wire>,
//     fans: Query<&Parent, With<GateFan>>
// ) {
//     for ev in logic_ev_rd.read() {
//         match ev {
//             LogicEvent::AddGate(entity) => sim.add_gate(*entity),
//             LogicEvent::RemoveGate(entity) => {
//                 // Try to update the wires Set in [`GateOutput`]s
//                 // of all incoming wires in the world,
//                 // if the gate still exists.
//                 sim.iter_all_wires(*entity).for_each(|(wire_entity, wire)| {
//                     let Ok(mut output) = gate_outputs.get_mut(wire.from) else {
//                         return;
//                     };
//                     output.wires.remove(&wire_entity);
//                 });

//                 // Remove the gate from the graph.
//                 sim.remove_gate(*entity);
//             }
//             &LogicEvent::AddWire { from_gate, from_output, to_gate, wire_entity } => {
//                 sim.add_wire(from_gate, to_gate, wire_entity);

//                 // Try to update the wires Set in the [`GateOutput`].
//                 let Ok(mut output) = gate_outputs.get_mut(from_output) else {
//                     continue;
//                 };
//                 output.wires.insert(wire_entity);
//             }
//             &LogicEvent::RemoveWire { from_gate, to_gate, wire_entity } => {
//                 // Try to update the wires Set in [`GateOutput`]s
//                 // of all incoming wires in the world, if the gate still exists.
//                 let Ok(mut output) = gate_outputs.get_mut(from_gate) else {
//                     continue;
//                 };
//                 output.wires.remove(&wire_entity);

//                 // Remove the wire from the graph.
//                 sim.remove_wire(from_gate, to_gate);
//             }
//             LogicEvent::AddWireByEntity(entity) => {
//                 let wire = wires
//                     .get(*entity)
//                     .expect(
//                         "Tried to `LogicEvent::AddWire` an entity that either does not exist or does not have a `Wire` component"
//                     );

//                 // Get the parent of each fan of the wire.
//                 let from_gate = {
//                     let parent = fans
//                         .get(wire.from)
//                         .expect(
//                             "Wire.from does not have a parent, did u connect a wire directly to gate instead of a fan?"
//                         );
//                     parent.get()
//                 };
//                 let to_gate = {
//                     let parent = fans
//                         .get(wire.to)
//                         .expect(
//                             "Wire.to does not have a parent, did u connect a wire directly to gate instead of a fan?"
//                         );
//                     parent.get()
//                 };

//                 // Add the wire to the graph.
//                 sim.add_wire(from_gate, to_gate, *entity);
//             }
//             LogicEvent::RemoveWireByEntity(entity) => {
//                 let wire = wires
//                     .get(*entity)
//                     .expect(
//                         "Tried to `LogicEvent::RemoveWire` an entity that either does not exist or does not have a `Wire` component"
//                     );

//                 // Get the parent of each fan of the wire.
//                 let from_gate = {
//                     let parent = fans
//                         .get(wire.from)
//                         .expect(
//                             "Wire.from does not have a parent, did u connect a wire directly to gate instead of a fan?"
//                         );
//                     parent.get()
//                 };
//                 let to_gate = {
//                     let parent = fans
//                         .get(wire.to)
//                         .expect(
//                             "Wire.to does not have a parent, did u connect a wire directly to gate instead of a fan?"
//                         );
//                     parent.get()
//                 };

//                 // Remove the wire from the graph.
//                 sim.remove_wire(from_gate, to_gate);
//             }
//         }
//     }

//     sim.compile();
// }
