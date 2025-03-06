use crate::graph::{CsvGraph};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_to_graph(path: Vec<usize>, distance_matrix: &Vec<Vec<f64>>) -> JsValue {
    let mut new_graph = CsvGraph::new();

    for i in 0..path.len() - 1 {
        let from_index = path[i];
        let to_index = path[i + 1];
        let cost = distance_matrix[from_index][to_index];

        //Handle from node
        let from_label = format!("Node {}", from_index);
        let from_node = new_graph.node_map.entry(from_label.clone()).or_insert_with(|| {
            let id = new_graph.graph.node_count();
            new_graph.graph.add_node(Node { id, label: from_label })
        });

        //Handle to node
        let to_label = format!("Node {}", to_index);
        let to_node = new_graph.node_map.entry(from_label.clone()).or_insert_with(|| {
            let id = new_graph.graph.node_count();
            new_graph.graph.add_node(Node {id, label: to_label})
        });

        // Add directed weighted edge
        new_graph.graph.add_edge(*from_node, *to_node, cost);
    }

    JsValue::from_serde(&new_graph).unwrap()
}
