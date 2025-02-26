use petgraph::graph::{DiGraph, NodeIndex, EdgeIndex};
use csv::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub cost: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: usize,
    pub label: String,
}

pub struct CsvGraph {
    pub graph: DiGraph<String, i64>,
    pub node_map: HashMap<String, NodeIndex>,
}

impl CsvGraph {
    // Create a new empty graph
    pub fn new() -> Self {
        CsvGraph {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        }
    }

    // Read from file. Note: WASM cannot access file system
    pub fn load_csv_from_file(&mut self, file_path: &str) -> Result<(), io::Error> {
        let file = File::open(file_path)?;
        let mut reader = Reader::from_reader(BufReader::new(file));
        self.parse_csv(&mut reader)?;
        Ok(())
    }

    // Read from string. Note: WASM supported
    pub fn load_csv_from_string(&mut self, csv_data: &str) -> Result<(), io::Error> {
        let mut reader = Reader::from_reader(csv_data.as_bytes());
        self.parse_csv(&mut reader)?;
        Ok(())
    }

    fn parse_csv<R: std::io::Read>(&mut self, reader: &mut Reader<R>) -> Result<(), io::Error> {
        for result in reader.records() {
            let record = result.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("CSV error: {}", e)))?;
            let from = record[0].to_string();
            let to = record[1].to_string();
            let cost: i64 = record[2].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid cost value"))?; 
            // Need to work on handling decimals later. Floats are not precise.

            // Get/insert the 'from' node
            let from_index = *self.node_map.entry(from.clone()).or_insert_with(|| self.graph.add_node(from));

            // Get/insert the 'to' node
            let to_index = *self.node_map.entry(to.clone()).or_insert_with(|| self.graph.add_node(to));

            // Add a directed edge with cost
            self.graph.add_edge(from_index, to_index, cost);
        }

        Ok(())
    }

    // Get edges in JSON-friendly format
    pub fn get_edges(&self) -> Vec<Edge> {
        self.graph.edge_indices().map(|e| {
            let (from_index, to_index) = self.graph.edge_endpoints(e).unwrap();
            Edge {
                from: self.graph[from_index].clone(),
                to: self.graph[to_index].clone(),
                cost: self.graph[e],
            }
        }).collect()
    }

    // Get all nodes in JSON-friendly format
    pub fn get_nodes(&self) -> Vec<Node> {
        self.graph.node_indices().map(|idx| Node {
            id: idx.index(),
            label: self.graph[idx].clone(),
        }).collect()
    }
}




