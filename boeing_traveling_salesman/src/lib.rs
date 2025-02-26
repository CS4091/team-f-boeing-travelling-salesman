mod csv_reader;

use wasm_bindgen::prelude::*;
use csv_reader::{CsvGraph, Edge, Node};
use serde_wasm_bindgen::to_value;
use console_log;
use wasm_logger::init;


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
    
    pub fn load_csv_from_file(&mut self, file_path: String) -> JsValue {
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

        to_value(&self.graph).unwrap()
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
