mod algorithms;
mod graph;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use graph::CsvGraph;
use serde_wasm_bindgen::{from_value, to_value};
use serde::{Serialize, Deserialize};


// Logging functions for Wasm targets. Pushes error messages to web console via Javascript.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console, js_name = log)]
        fn log(s: &str);
        //log a vector to console: log(&format!("{:?}", line));
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
        fn log_usize(a: usize);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
        fn log_i64(a: i64);
}

// Define mock logging functions for non-Wasm targets
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused)]
fn log(s: &str) {
    println!("{}", s);
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused)]
fn log_usize(a: usize) {
    println!("{}", a);
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused)]
fn log_i64(a: i64) {
    println!("{}", a);
}

#[derive(Serialize, Deserialize)]
struct AlgorithmResult {
    cost: f64,
    path: Vec<usize>,
}

//Wasm friendly Algorithm wrappers

#[wasm_bindgen]
pub fn held_karp_wasm(distance_matrix: JsValue) -> Result<JsValue, JsValue> {
    //Deserialze JsValue into Vec<Vec<f64>> because wasm didn't want to play nice
    let distance_matrix: Vec<Vec<f64>> = from_value(distance_matrix)
        .map_err(|e| JsValue::from_str(&format!("Failed to deserialize intput: {}", e)))?;

    let (cost, path) = algorithms::held_karp_algorithm(&distance_matrix);

    let result = AlgorithmResult {cost, path};

    //Convert back to JsValue
    to_value(&result).map_err(|e| {
        JsValue::from_str(&format!("Failed to serialize result: {}", e))
    })
}

#[wasm_bindgen]
pub fn nearest_neighbor_full_wasm(distance_matrix: JsValue) -> Result<JsValue, JsValue> {
    let distance_matrix: Vec<Vec<f64>> = from_value(distance_matrix)
        .map_err(|e| JsValue::from_str(&format!("Failed to deserialize input: {}", e)))?;

    let mut distance_map: HashMap<usize, HashMap<usize, f64>> = HashMap::new();

    for (i, row) in distance_matrix.iter().enumerate() {
        let mut inner_map = HashMap::new();
        for (j, &value) in row.iter().enumerate() {
            inner_map.insert(j, value);
        }
        distance_map.insert(i, inner_map);
    }
    let (cost, path) = algorithms::nearest_neighbor_full_graph(distance_map);

    let result = AlgorithmResult { cost, path };
    to_value(&result).map_err(|e| JsValue::from_str(&format!("Failed to serialize output: {}", e)))
}

#[wasm_bindgen]
pub fn nearest_neighbor_sparse_wasm(adj_list: JsValue) -> Result<JsValue, JsValue> {
    let adj_list: Vec<(usize, usize, f64)> = from_value(adj_list)
        .map_err(|e| JsValue::from_str(&format!("Failed to deserialize input: {}", e)))?;

    let mut distances: HashMap<usize, HashMap<usize, f64>> = HashMap::new();

    for (from, to, cost) in adj_list {
        distances
            .entry(from)
            .or_insert_with(HashMap::new)
            .insert(to, cost);
    }

    let (cost, path) = algorithms::nearest_neighbor_sparse(distances);

    let result = AlgorithmResult { cost, path };
    to_value(&result).map_err(|e| JsValue::from_str(&format!("Failed to serialize output: {}", e)))
}

#[wasm_bindgen]
pub struct GraphWrapper {
    graph: CsvGraph,
}

#[wasm_bindgen]
impl GraphWrapper {
    // Create a new empty graph
    #[wasm_bindgen(constructor)]
    pub fn new() -> GraphWrapper {
        GraphWrapper {
            graph: CsvGraph::new(),
        }
    }
   
    #[wasm_bindgen]
    pub fn load_csv_from_file(&mut self, file_path: String) -> JsValue {
        let file_path = "../full_world.csv";

        // Attempt to load the CSV file!
        match self.graph.load_csv_from_file(file_path) {
            Ok(_) => {
                // If the CSV is successfully loaded expose to JavaScript
                log("CSV file successfully loaded!");
            },
            Err(e) => {
                log(&format!("Error loading CSV file: {}", e));
            },
        }

        to_value(&self.graph.to_serializable()).unwrap()
    }

    // Load CSV from string. (for WebAssembly) Note: WASM accessible.
    #[wasm_bindgen]
    pub fn load_csv_from_string(&mut self, csv_data: &str) -> Result<(), JsValue> {
        self.graph
            .load_csv_from_string(csv_data)
            .map_err(|e| JsValue::from_str(&format!("CSV parse error: {}", e)))
    }

    // Expose nodes and Edges to JavaScript
    #[wasm_bindgen]
    pub fn get_edges(&self) -> JsValue {
        to_value(&self.graph.get_edges()).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_nodes(&self) -> JsValue {
        to_value(&self.graph.get_nodes()).unwrap()
    }

}
