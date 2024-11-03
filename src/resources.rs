use bevy::prelude::*;
use petgraph::{ algo::kosaraju_scc, graphmap::DiGraphMap };

use crate::{ components::Wire, logic::builder::{ GateData, WireData } };

pub mod prelude {
    pub use super::LogicGraph;
}

/// The logic graph resource determines the order
/// logic gates are evaluated in.
#[derive(Resource, Default, Reflect)]
pub struct LogicGraph {
    #[reflect(ignore)]
    pub graph: DiGraphMap<Entity, Entity>,
    sorted: Vec<Entity>,
}

impl LogicGraph {
    /// Insert [`LogicGraphData`] into self.
    pub fn add_data<T: LogicGraphData>(&mut self, data: T) -> &mut Self {
        data.add_to_graph(self);
        self
    }

    /// Remove [`LogicGraphData`] from self.
    pub fn remove_data<T: LogicGraphData>(&mut self, data: T) -> &mut Self {
        data.remove_from_graph(self);
        self
    }

    /// Add a gate to the graph.
    pub fn add_gate(&mut self, gate_entity: Entity) -> &mut Self {
        self.graph.add_node(gate_entity);
        self
    }

    /// Connect two gates with a wire.
    pub fn add_wire(
        &mut self,
        from_gate: Entity,
        to_gate: Entity,
        wire_entity: Entity
    ) -> &mut Self {
        self.graph.add_edge(from_gate, to_gate, wire_entity);
        self
    }

    /// Remove a gate from the graph.
    pub fn remove_gate(&mut self, gate_entity: Entity) -> &mut Self {
        self.graph.remove_node(gate_entity);
        self
    }

    /// Remove a wire from the graph.
    pub fn remove_wire(&mut self, from_gate: Entity, to_gate: Entity) -> &mut Self {
        self.graph.remove_edge(from_gate, to_gate);
        self
    }

    /// Returns an iterator over all incoming wires to a gate.
    ///
    /// The tuple represents `(wire_entity, Wire { from, to })`.
    pub fn iter_incoming_wires(&self, gate: Entity) -> impl Iterator<Item = (Entity, Wire)> + '_ {
        self.graph
            .edges_directed(gate, petgraph::Direction::Incoming)
            .map(|(from, to, wire)| (*wire, Wire { from, to }))
    }

    /// Returns an iterator over all outgoing wires from a gate.
    pub fn iter_outgoing_wires(&self, gate: Entity) -> impl Iterator<Item = (Entity, Wire)> + '_ {
        self.graph
            .edges_directed(gate, petgraph::Direction::Outgoing)
            .map(|(from, to, wire)| (*wire, Wire { from, to }))
    }

    /// Iterate over all wires connected to a gate. This includes both incoming and outgoing wires.
    pub fn iter_all_wires(&self, gate: Entity) -> impl Iterator<Item = (Entity, Wire)> + '_ {
        self.iter_incoming_wires(gate).chain(self.iter_outgoing_wires(gate))
    }

    pub fn compile(&mut self) {
        self.sorted = kosaraju_scc(&self.graph).into_iter().flatten().rev().collect();
    }

    pub fn sorted(&self) -> &[Entity] {
        &self.sorted
    }
}

pub trait LogicGraphData {
    /// Add `self` to a [`LogicGraph`].
    fn add_to_graph(&self, graph: &mut LogicGraph);

    /// Remove `self` from a [`LogicGraph`].
    fn remove_from_graph(&self, graph: &mut LogicGraph);
}

impl<I, O> LogicGraphData for GateData<I, O> {
    fn add_to_graph(&self, graph: &mut LogicGraph) {
        graph.graph.add_node(self.id());
    }

    fn remove_from_graph(&self, graph: &mut LogicGraph) {
        graph.graph.remove_node(self.id());
    }
}

impl LogicGraphData for WireData {
    fn add_to_graph(&self, graph: &mut LogicGraph) {
        graph.graph.add_edge(self.from_gate, self.to_gate, self.id());
    }

    fn remove_from_graph(&self, graph: &mut LogicGraph) {
        graph.graph.remove_edge(self.from_gate, self.to_gate);
    }
}

impl<T: LogicGraphData> LogicGraphData for Vec<T> {
    fn add_to_graph(&self, graph: &mut LogicGraph) {
        for data in self {
            data.add_to_graph(graph);
        }
    }

    fn remove_from_graph(&self, graph: &mut LogicGraph) {
        for data in self {
            data.remove_from_graph(graph);
        }
    }
}

impl<T: LogicGraphData> LogicGraphData for &[T] {
    fn add_to_graph(&self, graph: &mut LogicGraph) {
        for data in self.iter() {
            data.add_to_graph(graph);
        }
    }

    fn remove_from_graph(&self, graph: &mut LogicGraph) {
        for data in self.iter() {
            data.remove_from_graph(graph);
        }
    }
}
