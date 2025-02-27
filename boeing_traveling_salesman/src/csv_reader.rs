use petgraph::graph::{DiGraph, NodeIndex};
use csv::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use serde::{Serialize, Deserialize};
//use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub cost: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    #[serde(serialize_with = "as_u64", deserialize_with = "from_u64")]
    pub id: usize,
    pub label: String,
}

// Serialization for usize
fn as_u64<S>(x: &usize, serializer: S) -> Result<S::Ok, S::Error> where 
    S: serde::Serializer,
{
    serializer.serialize_u64(*x as u64)
}

// Deserialization for usize
fn from_u64<'de, D>(deserializer: D) -> Result<usize, D::Error> where  
    D: serde::Deserializer<'de>,
{
    let value: u64 = serde::Deserialize::deserialize(deserializer)?;
    Ok(value as usize)
}


// Defining a serializable version of the graph
#[derive(Serialize, Deserialize)]
pub struct SerializableGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}


pub struct CsvGraph {
    pub graph: DiGraph<Node, i64>,
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
            let from = record.get(0).unwrap().to_string();
            let to = record.get(1).unwrap().to_string();
            let cost: i64 = record.get(2).unwrap().parse()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid cost value"))?; 
            // Need to work on handling decimals later. Floats are not precise.

            // Get/insert the 'from' node
            let from_index = *self.node_map.entry(from.clone()).or_insert_with(|| {
                let id = self.graph.node_count();
                self.graph.add_node(Node { id, label: from})
            });

            // Get/insert the 'to' node
            let to_index = *self.node_map.entry(to.clone()).or_insert_with(|| {
                let id = self.graph.node_count();
                self.graph.add_node(Node { id, label: to})
            });

            // Add a directed edge with cost
            self.graph.add_edge(from_index, to_index, cost);
        }

        Ok(())
    }

    pub fn to_serializable(&self) -> SerializableGraph {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for node_index in self.graph.node_indices() {
            let node = &self.graph[node_index];
            nodes.push(Node { 
                id: node.id, 
                label: node.label.clone(),
            });
        }

        for edge_index in self.graph.edge_indices() {
            if let Some((from, to)) = self.graph.edge_endpoints(edge_index) {
                let cost = self.graph[edge_index];

                edges.push(Edge {
                    from: self.graph[from].label.clone(),
                    to: self.graph[to].label.clone(),
                    cost
                });
            }
        }

        SerializableGraph {nodes, edges}
    }

    // Gets the nodes
    pub fn get_nodes(&self) -> Vec<Node> {
        self.to_serializable().nodes
    }

    // Returns the edges
    pub fn get_edges(&self) -> Vec<Edge> {
        self.to_serializable().edges
    }
}




