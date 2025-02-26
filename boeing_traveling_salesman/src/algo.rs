use std::fs;
use std::error::Error;
use std::cmp::Ordering;
use std::f64::INFINITY;
use std::collections::HashMap;
use std::hash::{Hash,Hasher};
use std::io;
use bitvec::prelude::*;
use std::process;
use std::collections::LinkedList;
use bitvec::prelude::*;

fn read_csv_as_matrix(filename: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> { //provides a distance matrix for held_karp_algorithm
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




fn held_karp_algorithm(distance_matrix: &Vec<Vec<f64>>) -> (f64, Vec<usize>) {
    let n = distance_matrix.len();
    let full_mask = (1 << n) - 1; // All nodes visited
    let mut dp = vec![vec![INFINITY; n]; 1 << n];
    let mut parent = vec![vec![-1; n]; 1 << n];

    // Base case: Starting at node 0, cost is 0
    dp[1][0] = 0.0;

    // Iterate over all subsets of nodes
    for mask in 1..(1 << n) {
        for last_visited in 0..n {
            if mask & (1 << last_visited) != 0 {
                // Try all possible previous nodes 'prev'
                for prev in 0..n {
                    if prev != last_visited && (mask & (1 << prev)) != 0 {
                        let prev_mask = mask ^ (1 << last_visited);
                        let new_cost = dp[prev_mask][prev] + distance_matrix[prev][last_visited];
                        if new_cost < dp[mask][last_visited] {
                            dp[mask][last_visited] = new_cost;
                            parent[mask][last_visited] = prev as isize;
                        }
                    }
                }
            }
        }
    }

    // Find the minimum cost to complete the cycle (not returning to node 0)
    let mut min_cost = INFINITY;
    let mut last_node = -1;
    for i in 1..n {
        let cost = dp[full_mask][i]; //+ distance_matrix[i][0] dont need to return to start node
        if cost < min_cost {
            min_cost = cost;
            last_node = i as isize;
        }
    }

    // Reconstruct the path
    let mut path = Vec::new();
    let mut mask = full_mask;
    let mut current_node = last_node;

    while current_node != -1 {
        path.push(current_node as usize);
        let next_node = parent[mask][current_node as usize];
        mask ^= 1 << (current_node as usize);
        current_node = next_node;
    }

    path.reverse(); // Reverse the path to get the correct order
    

    (min_cost, path)
}







fn read_csv_as_hashmap(filename: &str) -> Result<(HashMap<usize, HashMap<usize,f64>>, usize), io::Error> { 
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
    let mut amount_of_nodes: usize = 0;
    for (i, j, cost) in edges {
        // Insert or append the (j, cost) tuple to the vector at distances[i]
        distances.entry(i)
            .or_insert_with(|| HashMap::new())  // If the key i doesn't exist, initialize a new vector
            .insert(j, cost);         // Append (j, cost) to the vector
    
        // Update the largest node number
        if i > amount_of_nodes {
            amount_of_nodes = i;
        }
        if j > amount_of_nodes {
            amount_of_nodes = j;
        }
    }
    amount_of_nodes = amount_of_nodes + 1; // adds one to account for node 0
    Ok((distances, amount_of_nodes)) // Return the HashMap wrapped in Result
}


fn nearest_neighbor_full_graph(distances: &mut HashMap<usize,HashMap<usize,f64>>, amount_of_nodes: usize) -> (f64, Vec<usize>) { // nearest neighbor for full tree
    let mut total_min_cost:f64 = 0.0;
    let mut path = Vec::new();
    path.push(0);
    let mut visited = bitvec![0; amount_of_nodes]; // keeps track of which nodes are visited
    visited.set(0, true);//assume plan needs to head back to start
    let mut visited_counter: usize = 1;
    let mut current_location: usize = 0;
   
    
    while visited_counter != amount_of_nodes{
        let mut min_cost: f64 = f64::INFINITY;
        let mut node_to_visit = usize::MAX;
        for (second, cost) in &distances[&current_location] { //finds nearest neighbor
            // Use `second` (usize) and `cost` (f64) here
            if  visited[*second] == false && *cost < min_cost
            {
                min_cost = *cost;
                node_to_visit = *second;
            } 
            //println!("Second part: {}, Cost: {}", second, cost);
        }
        if min_cost != f64::INFINITY { // found a node to visit
            path.push(node_to_visit);
            total_min_cost = total_min_cost + min_cost;
            //println!("The cost being added {}", min_cost);
            visited_counter = visited_counter + 1;
            current_location = node_to_visit;
            visited.set(node_to_visit,true);
            
        }
        else {
            println!("Could not find a node but graph should be a full graph ERROR");
            process::exit(1);
        
        }
        
    }
        //println!("{:?}, {:?}", path, visited);
        //println!("{:?}, ", path);
        //println!("Banned- {:?}", banned_list);
        (total_min_cost, path)
}












fn main() {
    
    let filename = "sparse_world.csv";
    match read_csv_as_matrix("sparse_world.csv") {
        Ok(distance_matrix) => {
            // Check if the matrix is 20x20 or smaller
            if distance_matrix.len() <= 20 && distance_matrix.iter().all(|row| row.len() <= 20) {
                let (cost, path) = held_karp_algorithm(&distance_matrix);
                println!("Minimum Cost: {}", cost);
                println!("Optimal Path: {:?}", path);
            } else {
                eprintln!("Matrix is too large. Only 20 locations or less are allowed.");
            }
        }
        Err(e) => eprintln!("Error reading CSV: {}", e),
    }

    if filename == "full_world.csv" {
        match read_csv_as_hashmap(filename) {
            Ok((mut distances, amount_of_nodes)) => {  // ✅ Correct tuple destructuring
                let (cost, path) = nearest_neighbor_full_graph(&mut distances, amount_of_nodes);
                println!("Minimum Cost: {}", cost);
                println!("Optimal Path: {:?}", path);
            }
            Err(e) => {  // Handle potential errors
                eprintln!("Error reading file: {}", e);
            }
        }    
    }
    

    


    



}
