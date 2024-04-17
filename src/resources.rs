use bevy::{ prelude::*, utils::petgraph::{ algo::kosaraju_scc, graphmap::DiGraphMap } };

use crate::logic::commands::{ GateData, WireData };

pub mod prelude {
    pub use super::LogicGraph;
}

/// A resources that stores logic gates' graph.
#[derive(Resource, Default)]
pub struct LogicGraph {
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

    pub fn compile(&mut self) {
        let post_order = kosaraju_scc(&self.graph);

        // Assert that all vecs contain a single element.
        #[cfg(debug_assertions)]
        for component in post_order.iter() {
            assert_eq!(component.len(), 1);
        }

        // flatten the vec of vecs into a single vec
        self.sorted = post_order.into_iter().flatten().rev().collect();
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
