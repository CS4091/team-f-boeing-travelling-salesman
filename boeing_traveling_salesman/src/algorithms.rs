
use std::f64::INFINITY;
use std::collections::HashMap;
use bitvec::prelude::*;
use std::process;

pub fn held_karp_algorithm(distance_matrix: &Vec<Vec<f64>>) -> (f64, Vec<usize>) { 
    // charles files could possibly have no valid path with start node 0 thus this will try start node 1 and so on till valid path found
    let n = distance_matrix.len();
    let full_mask = (1 << n) - 1;
    
    for start_node in 0..n { // Try different start nodes until a valid path is found
        let mut dp = vec![vec![INFINITY; n]; 1 << n];
        let mut parent = vec![vec![-1; n]; 1 << n];

        // Base case: Starting at start_node, cost is 0
        dp[1 << start_node][start_node] = 0.0;

        // Iterate over all subsets of nodes
        for mask in 1..(1 << n) {
            for last_visited in 0..n {
                if mask & (1 << last_visited) != 0 {
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

        // Find the minimum cost path
        let mut min_cost = INFINITY;
        let mut last_node = -1;
        for i in 0..n {
            let cost = dp[full_mask][i];
            if cost < min_cost {
                min_cost = cost;
                last_node = i as isize;
            }
        }

        // If a valid path is found, reconstruct and return it
        if min_cost < INFINITY {
            let mut path = Vec::new();
            let mut mask = full_mask;
            let mut current_node = last_node;

            while current_node != -1 {
                path.push(current_node as usize);
                let next_node = parent[mask][current_node as usize];
                mask ^= 1 << (current_node as usize);
                current_node = next_node;
            }

            path.reverse();
            return (min_cost, path);
        }
    }

    // If no valid path is found for any start node, return failure
    (INFINITY, Vec::new())
}

pub fn nearest_neighbor_full_graph(distances: &HashMap<usize,HashMap<usize,f64>>) -> (f64, Vec<usize>) { // nearest neighbor for full tree
    let mut total_min_cost:f64 = 0.0;
    let mut path = Vec::new();
    path.push(0);
    let amount_of_nodes = distances.keys().len();
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







pub fn nearest_neighbor_sparse(distances: &HashMap<usize,HashMap<usize,f64>>) -> (f64, Vec<usize>) {
    let mut total_min_cost:f64 = 0.0;
    // Collect the keys of distances into a Vec<usize>
    let nodes = distances.keys();
    let mut collection: Vec<usize> = nodes.cloned().collect();
    collection.sort(); // Ensure deterministic order
    
    let mut paths: Vec<Vec<usize>> = Vec::new();

    // Iterate over each element in collection
    for node in collection { // makes a vector of length one paths ex - [[0],[1], [2]]
        let mut new_path: Vec<usize> = Vec::new();
        new_path.push(node); 
        paths.push(new_path); 
    }
    
    
    
    let mut banned_paths: Vec<Vec<usize>> = Vec::new(); // prevents cycles has a large worse case memory usage need to manage better
    let mut _banned_counter:usize = 0;
    let mut counter = 0;
    while paths.len() > 1 { // TODO Fix infinite loop that can happend if nothing can connect. 
           
        
            if let Some(path) = paths.pop() { // grab path at front of list
                let mut min_cost: f64 = f64::INFINITY; 
                let mut path_to_add:Vec<usize> = Vec::new();
                    for possible_path_to_add in paths.clone(){
                            if let Some(front_node) = path.last() { // gets the last location of the popped path
                                if let Some(last_node) = possible_path_to_add.first() { // gets the front location of another path (going to try to connect the two paths)
                                    if let Some(inner_map) = distances.get(&front_node) { // use hashmap to get cost
                                        if let Some(&cost) = inner_map.get(&last_node) { // use hashmap to get cost
                                                if path != possible_path_to_add &&cost < min_cost // checks if visited yet and cost is cheaper
                                                {
                                                    let mut temp_path = path.clone();
                                                    temp_path.extend(possible_path_to_add.clone());
                                                    if !banned_paths.contains(&temp_path){
                                                        min_cost = cost;
                                                        path_to_add = possible_path_to_add;
                                                    }
                                                } 

                                            }
                                            else{
                                                //println!("No arc between paths");
                                            }
                                        } 
                                        else {
                                            //println!("front node has no valid arcs to anywhere");
                                        }
                                        
                                }      
                                else {
                                    println!("Path has no nodes");
                                    process::exit(1);
                                } 
                            }
                            else { //covers path.last and path.first in above if statement
                                println!("Path has no nodes");
                                process::exit(1);
                            }
                    }      
                    if min_cost != f64::INFINITY { // found a node to visit
                                paths.retain(|x| *x != path_to_add); // get rid of path from main vector of paths
                                let mut new_path = path.clone();
                                new_path.extend(path_to_add.clone());
                                paths.insert(0,new_path.clone()); // adds the new combined paths to the paths vector
                                total_min_cost = total_min_cost + min_cost;

                
                    }
                    else {// no path found so pop one in path and stufff
                        if path.len() > 1{ 
                            let mut new_path = path;
                            banned_paths.push(new_path.clone()); // bans path to prevent cycles from repeating paths
                            _banned_counter += 1;
                            if let Some(last_node) = new_path.pop() { // pop node out of path
                                if let Some(temp_node) = new_path.last() { 
                                    total_min_cost = total_min_cost - distances[temp_node][&last_node]; // undoes the cost of that path
                                    paths.push(new_path); // adds path with popped node back to main paths
                                    let mut temp_path: Vec<usize> = Vec::new();
                                    temp_path.push(last_node);
                                    paths.insert(0,temp_path) // adds the node that was popped out of path back to front of vector of paths
                                    
                                }
                                else { // should never run unless path somehow becomes empty
                                    println!("No node to fall back to, path does not contain start node anymore!");
                                    process::exit(1);
                                }
                            } else { //handles error of possible empty vector
                                println!("The start node had no valid edges to go to ERROR!");
                                process::exit(1);
                            }

                        }
                        else {
                            paths.insert(0,path) // insert single length paths at front of list
                        }
                        
                    
                        
                    }
                } else {
                    println!("No paths left to process!");
                    process::exit(1);
                }
            
    
        counter = counter + 1;
        
    }
    //println!("Banned- {:?}, Amount of banned paths - {}", banned_paths, banned_paths.len()); // for developement and testing
    //println!("Amount of banned paths - {}", _banned_counter);
    if let Some(path) = paths.first(){
        (total_min_cost, path.clone())
    }
    else
    {
        println!("No path generated");
        process::exit(1);
    }
    
}
