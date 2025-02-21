use wasm_bindgen::prelude::*;
use petgraph::graph::{DiGraph, NodeIndex};

#[wasm_bindgen]
pub struct Graph {
    graph: DiGraph<String, ()>,
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Graph {
            graph: DiGraph::new(),
        }
    }

    #[wasm_bindgen]
    pub fn add_node(&mut self, label: String) -> usize {
        let index = self.graph.add_node(label);
        index.index()
    }

    #[wasm_bindgen]
    pub fn add_edge(&mut self, from: usize, to: usize) {
        let from = NodeIndex::new(from);
        let to = NodeIndex::new(to);
        self.graph.add_edge(from, to, ());
    }

    #[wasm_bindgen]
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }
}
