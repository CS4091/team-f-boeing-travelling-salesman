
use std::fs;
use std::time::Instant;









pub fn algorithm_analyzer(table_name:String, folder_name:String, lower_bound: usize, upper_bound: usize ) {
    //--------------Initalize avg time and cost for algorithms here----------------// 
    // held karp nearest
    let mut held_karp_avg_cost:f64 = 0.0;
    let mut held_karp_avg_time:f64 = 0.0;
    //sparse nearest
    let mut sparse_nearest_avg_cost:f64 = 0.0;
    let mut sparse_nearest_avg_time:f64 = 0.0;
    // full nearest
    let mut full_graph_nearest_avg_cost:f64 = 0.0;
    let mut full_graph_nearest_avg_time:f64 = 0.0;
    //----------------------------------------------------------------------------//
    let folder_path = folder_name; 
    let mut routes_planned_counter = 0;
    for entry in fs::read_dir(&folder_path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let file_path = entry.path();
        if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "csv") {
            let distances = match crate::csv_readers::read_csv_as_hashmap(file_path.to_str().unwrap()) {   // gets hashmap for route algorithm
                Ok(distances) => distances,
                Err(e) => {
                    eprintln!("Error reading {}: {}", file_path.display(), e);
                    continue; // Skip this file and proceed to the next
                }
            };
            let distance_matrix = match crate::csv_readers::read_csv_as_matrix(file_path.to_str().unwrap()) { // gets distance matrix
                Ok(distance_matrix) => distance_matrix,
                Err(e) => {
                    eprintln!("Error reading {}: {}", file_path.display(), e);
                    continue; // Skip this file and proceed to the next
                }
            };
            let amount_of_nodes = distance_matrix.len();
            if amount_of_nodes >= lower_bound && amount_of_nodes <= upper_bound{
                routes_planned_counter += 1;
                //-------------------------Put new algorithms here and their limits here -------------------------------------//
                // held karp
                if distance_matrix.len() <= 20 {
                    let start_time = Instant::now();  // Start timer
                    let (cost,_path) = crate::algorithms::held_karp_algorithm(&distance_matrix);
                    let duration = start_time.elapsed().as_secs_f64();  // get total time in fractional seconds
                    held_karp_avg_cost += cost;
                    held_karp_avg_time += duration;
                }
                // sparse nearest neighbor
                let start_time = Instant::now();  // Start timer
                let (cost,_path) = crate::algorithms::nearest_neighbor_sparse(&distances);
                let duration = start_time.elapsed().as_secs_f64();  // get total time in fractional seconds
                sparse_nearest_avg_cost += cost;
                sparse_nearest_avg_time += duration;
                // full graph nearest neighbor
                if folder_path == "test-csv-full-world" {
                    let start_time = Instant::now();  // Start timer
                    let (cost,_path) = crate::algorithms::nearest_neighbor_full_graph(&distances);
                    let duration = start_time.elapsed().as_secs_f64();  // get total time in fractional seconds
                    full_graph_nearest_avg_cost += cost;
                    full_graph_nearest_avg_time += duration;
                }
                //--------------------------------------------------------------------------------------------------------------//
            }
            
        }
    }

    println!("{}", table_name);
    println!("_____________________________________________________________________________________________________________________");
    println!("|{:<20}|{:<19}|{:<13}|{:<16}|{:<14}|{:<13}|{:<15}|"
    , "Algorithms", "Avg Cost(per route)","Avg Time(Sec)" ,"Works on Sparse?" ,"Mult Aircraft?" , "20 locations?", "1000 locations?");
    //------------------------------Add algorithms info here----------------------------------//
    println!( // held karp algorithm
        "|{:<20}|{:<19.6}|{:<13.6}|{:<16}|{:<14}|{:<13}|{:<15}|",
        "Held Karp",
        held_karp_avg_cost as f64 / routes_planned_counter as f64,  // Ensure floating-point division
        held_karp_avg_time as f64 / routes_planned_counter as f64,  // Ensure floating-point division
        "Yes",
        "No",
        "Yes",
        "No"
    );
    println!( // sparse nearest neighbor
        "|{:<20}|{:<19.6}|{:<13.6}|{:<16}|{:<14}|{:<13}|{:<15}|",
        "Sparse Nearest",
        sparse_nearest_avg_cost as f64 / routes_planned_counter as f64,  // Ensure floating-point division
        sparse_nearest_avg_time as f64 / routes_planned_counter as f64,  // Ensure floating-point division
        "Yes",
        "No",
        "Yes",
        "Yes"
    );
    println!( // full_graph nearest neighbor
        "|{:<20}|{:<19.6}|{:<13.6}|{:<16}|{:<14}|{:<13}|{:<15}|",
        "Full Graph Nearest",
        full_graph_nearest_avg_cost as f64 / routes_planned_counter as f64,  // Ensure floating-point division
        full_graph_nearest_avg_time as f64 / routes_planned_counter as f64,  // Ensure floating-point division
        "No",
        "No",
        "Yes",
        "Yes"
    );

    //----------------------------------------------------------------------------------------//
    println!("_____________________________________________________________________________________________________________________");

}