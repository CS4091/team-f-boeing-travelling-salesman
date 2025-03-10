use std::error::Error;
use std::io;
use std::fs;
use std::collections::HashMap;



pub fn read_csv_as_matrix(filename: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> { //provides a distance matrix for held_karp_algorithm
    let contents = fs::read_to_string(filename)?; // Read the entire file
    let mut lines = contents.lines();

    lines.next(); // Skip the first line (header)

    let mut max_node = 0;
    let mut edges = Vec::new();

    // Parse each line as (node1, node2, cost)
    for line in lines {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() != 3 {
            continue; // Skip invalid lines
        }

        let node1: usize = parts[0].parse().unwrap_or(usize::MAX);
        let node2: usize = parts[1].parse().unwrap_or(usize::MAX);
        let cost: f64 = parts[2].parse().unwrap_or(f64::INFINITY); // Handle parse errors

        if node1 == usize::MAX || node2 == usize::MAX {
            continue; // Skip if parsing failed
        }

        max_node = max_node.max(node1).max(node2);
        edges.push((node1, node2, cost));
    }

    // Create a square matrix of size (max_node + 1)
    let size = max_node + 1;
    let mut matrix = vec![vec![f64::INFINITY; size]; size];

    // Fill the matrix with parsed distances
    for (i, j, cost) in edges {
        matrix[i][j] = cost;
    //    matrix[j][i] = cost; // Assuming undirected graph
    }

    // Set diagonal to 0 (distance from node to itself)
    //for i in 0..size {
    //    matrix[i][i] = 0.0;
    //}

    Ok(matrix)
}












pub fn read_csv_as_hashmap(filename: &str) -> Result<HashMap<usize, HashMap<usize,f64>>, io::Error> { 
    // Makes A nested HashMap, first node given gives you another hashMap, then give node you are heading to, to get cost
    // Saves space when dealing with sparse trees and provides quick look up of costs. 
    let mut distances: HashMap<usize, HashMap<usize, f64>> = HashMap::new();
    let contents = fs::read_to_string(filename)?; // Read the entire file
    let mut lines = contents.lines();
    lines.next(); // Skip the first line (header)

    let mut max_node: usize = 0;
    let mut edges: Vec<(usize, usize, f64)> = Vec::new();

    // Parse each line as (node1, node2, cost)
    for line in lines {
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() != 3 {
            continue; // Skip invalid lines
        }

        let node1: usize = parts[0].parse().unwrap_or(usize::MAX);
        let node2: usize = parts[1].parse().unwrap_or(usize::MAX);
        let cost: f64 = parts[2].parse().unwrap_or(f64::INFINITY); // Handle parse errors

        if node1 == usize::MAX || node2 == usize::MAX {
            continue; // Skip if parsing failed
        }

        max_node = max_node.max(node1).max(node2);
        edges.push((node1, node2, cost));
    }

    for (i, j, cost) in edges {
        // Insert or append the (j, cost) tuple to the vector at distances[i]
        distances.entry(i)
            .or_insert_with(|| HashMap::new())  // If the key i doesn't exist, initialize a new vector
            .insert(j, cost);         // Append (j, cost) to the vector
    
    }
    Ok(distances) // Return the HashMap wrapped in Result
}
