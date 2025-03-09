

#[cfg(test)]
mod tests{   

    use crate::*;
    use std::fs;
    use bitvec::prelude::*;
    //use std::path::Path;
    //use std::error::Error;

    fn is_valid_route(distance_matrix: &Vec<Vec<f64>>, cost: &f64, path: &Vec<usize>) -> String {
        if path.len() == 0 {
            return format!("No path found, path = []");
        }
        let mut visited = bitvec![0; distance_matrix.len()]; // keeps track of which nodes are visited
        visited.set(path[0], true); //visit start
        let mut cost_check: f64 = 0.0; //set up cost
        for i in 0..(path.len()-1) { // gets the information about the arc and stores in window
            println!("{}",i);
            let x = path[i];
            let y = path[i+1];
            if distance_matrix[x][y] == f64::INFINITY { // checks to see if arc in path actually exists
                return format!("No arc from {} to {}, in path {:?}", x, y, path);
            }
            if visited[y] { //checks to see if we are visiting a node twice
                return format!("Visited location {} twice in path {:?}", y, path);
            }
            else{ // sets node we are heading to to visited
                visited.set(y,true)
            }
            cost_check += distance_matrix[x][y]; // ads arc to cost
            
        }
        const TOLERANCE: f64 = 1e-6;
        if (cost_check - cost).abs() > TOLERANCE { //checks to see if costs are equal
            return format!("Cost was {} but algorithm returned {}", cost_check, cost);
        }

        return format!("valid"); //  returns valid if path had no issues
    }
    

    #[test]
    fn test_held_karp_route() {
        let folder_path = "test-csv-sparse"; // test sparses
    
        for entry in fs::read_dir(folder_path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let file_path = entry.path();
    
            // Ensure it's a file and has a .csv extension
            if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "csv") {
                println!("Testing file: {:?}", file_path.display());
                let distance_matrix = match crate::csv_readers::read_csv_as_matrix(file_path.to_str().unwrap()) {
                    Ok(matrix) => matrix,
                    Err(e) => {
                        eprintln!("Error reading {}: {}", file_path.display(), e);
                        continue; // Skip this file and proceed to the next
                    }
                };
    
                let size = distance_matrix.len();
                if size <= 20 {
                    let (cost, path) = crate::algorithms::held_karp_algorithm(&distance_matrix);
    
                    // Test validity of the route
                    let result = is_valid_route(&distance_matrix, &cost, &path);
    
                    // Assert that the result is valid
                    assert_eq!(result, "valid", "The route is invalid for file {}: {}", file_path.display(), result);
                } else {
                    assert!(size >= 20, "Expected result to be greater than 20, but got {}", size);
                }
            }
        }
    }
    #[test]
    fn test_nearest_neighbor_full_graph() {
        let folder_path = "test-csv-full-world"; // test full world
        for entry in fs::read_dir(folder_path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let file_path = entry.path();

            // Ensure it's a file and has a .csv extension
            if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "csv") {
                println!("Testing file: {:?}", file_path.display());
                let distances = match crate::csv_readers::read_csv_as_hashmap(file_path.to_str().unwrap()) {   // gets hashmap for route algorithm
                    Ok(distances) => distances,
                    Err(e) => {
                        eprintln!("Error reading {}: {}", file_path.display(), e);
                        continue; // Skip this file and proceed to the next
                    }
                };
                let distance_matrix = match crate::csv_readers::read_csv_as_matrix(file_path.to_str().unwrap()) { // gets distance matrix for validating route function
                    Ok(matrix) => matrix,
                    Err(e) => {
                        eprintln!("Error reading {}: {}", file_path.display(), e);
                        continue; // Skip this file and proceed to the next
                    }
                };

                let (cost, path) = crate::algorithms::nearest_neighbor_full_graph(&distances);

                // Test validity of the route
                let result = is_valid_route(&distance_matrix, &cost, &path);

                // Assert that the result is valid
                assert_eq!(result, "valid", "The route is invalid: {}", result);
            }
        }
    }
    #[test]
    fn test_nearest_neighbor_sparse() {
        let folder_path = "test-csv-sparse"; // test full world
        for entry in fs::read_dir(folder_path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let file_path = entry.path();

            // Ensure it's a file and has a .csv extension
            if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "csv") {
                println!("Testing file: {:?}", file_path.display());
                let distances = match crate::csv_readers::read_csv_as_hashmap(file_path.to_str().unwrap()) {   // gets hashmap for route algorithm
                    Ok(distances) => distances,
                    Err(e) => {
                        eprintln!("Error reading {}: {}", file_path.display(), e);
                        continue; // Skip this file and proceed to the next
                    }
                };
                let distance_matrix = match crate::csv_readers::read_csv_as_matrix(file_path.to_str().unwrap()) { // gets distance matrix for validating route function
                    Ok(matrix) => matrix,
                    Err(e) => {
                        eprintln!("Error reading {}: {}", file_path.display(), e);
                        continue; // Skip this file and proceed to the next
                    }
                };

                let (cost, path) = crate::algorithms::nearest_neighbor_sparse(&distances);

                // Test validity of the route
                let result = is_valid_route(&distance_matrix, &cost, &path);

                // Assert that the result is valid
                assert_eq!(result, "valid", "The route is invalid: {}", result);
            }
        }
    }
    #[test]
    fn test_invalid_arc() {
        // Create a distance matrix where there's no arc between nodes 0 and 3 (represented by `f64::INFINITY`)
        let distance_matrix = vec![
            vec![0.0, 1.0, f64::INFINITY],
            vec![1.0, 0.0, 1.0],
            vec![f64::INFINITY, 1.0, 0.0],
        ];
        
        // Path includes an invalid arc from 0 to 3 (which doesn't exist in the matrix)
        let path = vec![0, 2];
        
        let cost = 0.0; // The cost doesn't matter here since the arc is invalid
        
        // Run the function and assert the result
        let result = is_valid_route(&distance_matrix, &cost, &path);
        assert!(result.contains("No arc from 0 to 2"), "Test failed with result: {}", result);
    }
    #[test]
    fn test_revisited_node() {
        // Create a valid distance matrix where all nodes are connected
        let distance_matrix = vec![
            vec![0.0, 1.0, 1.0],
            vec![1.0, 0.0, 1.0],
            vec![1.0, 1.0, 0.0],
        ];
        
        // Path visits node 1 twice: [0, 1, 0]
        let path = vec![0, 1, 0];
        
        let cost = 2.0; // The correct cost for this path is 2.0
        
        // Run the function and assert the result
        let result = is_valid_route(&distance_matrix, &cost, &path);
        assert!(result.contains("Visited location 0 twice"), "Test failed with result: {}", result);
    }
    #[test]
    fn test_incorrect_cost() {
        // Create a distance matrix
        let distance_matrix = vec![
            vec![0.0, 1.0, 1.0],
            vec![1.0, 0.0, 1.0],
            vec![1.0, 1.0, 0.0],
        ];
        
        // Path with valid nodes but an incorrect cost: [0, 1, 2]
        let path = vec![0, 1, 2];
        
        // Incorrect expected cost (it should be 2.0, but we'll give it 3.0)
        let cost = 3.0;
        
        // Run the function and assert the result
        let result = is_valid_route(&distance_matrix, &cost, &path);
        assert!(result.contains("Cost was"), "Test failed with result: {}", result);
    }
    
}