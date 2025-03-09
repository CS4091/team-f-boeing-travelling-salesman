

mod tests;
mod algorithms;
mod analyzer;
mod csv_readers;
//use std::error::Error;
//use std::io;
//use std::fs;
//use std::collections::HashMap;













fn main() {
     
    let filename = "full_world.csv";
    match crate::csv_readers::read_csv_as_matrix(filename) {
        Ok(distance_matrix) => {
            // Check if the matrix is 20x20 or smaller
            if distance_matrix.len() <= 20 && distance_matrix.iter().all(|row| row.len() <= 20) {
                let (cost, path) = crate::algorithms::held_karp_algorithm(&distance_matrix);
                println!("Held_Karps_Optimal solution algorithm-");
                println!("Minimum Cost: {}", cost);
                println!("Optimal Path: {:?}", path);
            } else {
                eprintln!("Matrix is too large. Only 20 locations or less are allowed.");
            }
        }
        Err(e) => eprintln!("Error reading CSV: {}", e),
    }

    if filename == "full_world.csv" {
        match crate::csv_readers::read_csv_as_hashmap(filename) {
            Ok(distances) => {  
                let (cost, path) = crate::algorithms::nearest_neighbor_full_graph( &distances);
                println!("Nearest Neighbor full world solution algorithm-");
                println!("Minimum Cost: {}", cost);
                println!("Optimal Path: {:?}", path);
            }
            Err(e) => {  // Handle potential errors
                eprintln!("Error reading file: {}", e);
            }
        }    
    }

    if filename == filename {
        match crate::csv_readers::read_csv_as_hashmap(filename) {
            Ok(distances) => {  
                let (cost, path) = crate::algorithms::nearest_neighbor_sparse( &distances);
                println!("Nearest Neighbor sparse world solution algorithm-");
                println!("Minimum Cost: {}", cost);
                println!("Optimal Path: {:?}", path);
            }
            Err(e) => {  // Handle potential errors
                eprintln!("Error reading file: {}", e);
            }
        }    
    }
    println!("");
    crate::analyzer::algorithm_analyzer("Sparse graphs with 15 to 20 locations".to_string(), "test-csv-sparse".to_string(), 15,20); // needs table name, folder name for csv files, location lower bound, location upper bound
    crate::analyzer::algorithm_analyzer("Sparse graphs with 100 to 100 locations".to_string(), "test-csv-sparse".to_string(), 100,100);
    crate::analyzer::algorithm_analyzer("Full graphs with 100 to 100 locations".to_string(), "test-csv-full-world".to_string(), 100,100);
    crate::analyzer::algorithm_analyzer("Sparse graphs with 1000 to 1000 locations".to_string(), "test-csv-sparse".to_string(), 1000,1000);
    crate::analyzer::algorithm_analyzer("Full graphs with 1000 to 1000 locations".to_string(), "test-csv-full-world".to_string(), 1000,1000);
}