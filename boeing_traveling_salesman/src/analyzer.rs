
use std::fs;
use std::time::Instant;









pub fn algorithm_analyzer(table_name:String, folder_name:String, lower_bound: usize, upper_bound: usize ) {
    //--------------Initalize avg time and cost for algorithms here----------------// 
    const SIZE:usize = 5;
    // held karp nearest
    let mut held_karp_data: [f64; SIZE] = [0.0; SIZE]; // Avg cost, min cost, max cost, avg time, max time (only need worse case and avg for time really)
    held_karp_data[1] = f64::INFINITY;
    //sparse nearest
    let mut sparse_nearest_data: [f64; SIZE] = [0.0; SIZE]; // Avg cost, min cost, max cost, avg time, max time
    sparse_nearest_data[1] = f64::INFINITY;
    // full nearest
    let mut full_nearest_data: [f64; SIZE] = [0.0; SIZE]; // Avg cost, min cost, max cost, avg time, max time
    full_nearest_data[1] = f64::INFINITY;
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
            //println!("{}-{}-{}",amount_of_nodes, lower_bound,upper_bound );
            if amount_of_nodes >= lower_bound && amount_of_nodes <= upper_bound{
                routes_planned_counter += 1;
                //-------------------------Put new algorithms here and their limits here -------------------------------------//
                // held karp
                if distance_matrix.len() <= 20 {
                    let start_time = Instant::now();  // Start timer
                    let (cost,_path) = crate::algorithms::held_karp_algorithm(&distance_matrix);
                    let duration = start_time.elapsed().as_secs_f64();  // get total time in fractional seconds
                    held_karp_data[0] += cost;
                    held_karp_data[3] += duration;
                    if held_karp_data[1] > cost { // set min cost
                        held_karp_data[1] = cost;
                    }
                    if held_karp_data[2] < cost{ // set max cost
                        held_karp_data[2] = cost;
                    }
                    if held_karp_data[4] < duration{ // set max time
                        held_karp_data[4] = duration;
                    }
                }
                // sparse nearest neighbor
                let start_time = Instant::now();  // Start timer
                let (cost,_path) = crate::algorithms::nearest_neighbor_sparse(&distances);
                let duration = start_time.elapsed().as_secs_f64();  // get total time in fractional seconds
                sparse_nearest_data[0] += cost;
                sparse_nearest_data[3] += duration;
                if sparse_nearest_data[1] > cost { // set min cost
                    sparse_nearest_data[1] = cost;
                }
                if sparse_nearest_data[2] < cost{ // set max cost
                    sparse_nearest_data[2] = cost;
                }
                if sparse_nearest_data[4] < duration{ // set max time
                    sparse_nearest_data[4] = duration;
                }
                // full graph nearest neighbor
                if folder_path == "test-csv-full-world" {
                    let start_time = Instant::now();  // Start timer
                    let (cost,_path) = crate::algorithms::nearest_neighbor_full_graph(&distances);
                    let duration = start_time.elapsed().as_secs_f64();  // get total time in fractional seconds
                    full_nearest_data[0] += cost;
                    full_nearest_data[3] += duration;
                    if full_nearest_data[1] > cost { // set min cost
                        full_nearest_data[1] = cost;
                    }
                    if full_nearest_data[2] < cost{ // set max cost
                        full_nearest_data[2] = cost;
                    }
                    if full_nearest_data[4] < duration{ // set max time
                        full_nearest_data[4] = duration;
                    }
                }
                //--------------------------------------------------------------------------------------------------------------//
            }
            
        }
    }

    println!("{}", table_name);
    println!("_______________________________________________________________________________________________________________________________________________________________");
    println!("|{:<20}|{:<19}|{:<13}|{:<13}|{:<13}|{:<13}|{:<16}|{:<14}|{:<13}|{:<15}|"
    , "Algorithms", "Avg Cost(per route)", "Min Cost", "Max Cost","Avg Time(Sec)", "Max Time" ,"Works on Sparse?" ,"Mult Aircraft?" , "20 locations?", "1000 locations?");
    //------------------------------Add algorithms info here----------------------------------//
    println!( // held karp algorithm
        "|{:<20}|{:<19.6}|{:<13.6}|{:<13.6}|{:<13.6}|{:<13.6}|{:<16}|{:<14}|{:<13}|{:<15}|",
        "Held Karp",
        held_karp_data[0] as f64 / routes_planned_counter as f64,  // Ensure floating-point division, avg cost
        held_karp_data[1], //min cost
        held_karp_data[2], // max cost
        held_karp_data[3] as f64 / routes_planned_counter as f64,  // Ensure floating-point division, avg time
        held_karp_data[4], // max cost
        "Yes",
        "No",
        "Yes",
        "No"
    );
    println!( // sparse nearest algorithm
        "|{:<20}|{:<19.6}|{:<13.6}|{:<13.6}|{:<13.6}|{:<13.6}|{:<16}|{:<14}|{:<13}|{:<15}|",
        "Sparse Nearest",
        sparse_nearest_data[0] as f64 / routes_planned_counter as f64,  // Ensure floating-point division, avg cost
        sparse_nearest_data[1], //min cost
        sparse_nearest_data[2], // max cost
        sparse_nearest_data[3] as f64 / routes_planned_counter as f64,  // Ensure floating-point division, avg time
        sparse_nearest_data[4], // max cost
        "Yes",
        "No",
        "Yes",
        "No"
    );
    println!( // full nearest algorithm
        "|{:<20}|{:<19.6}|{:<13.6}|{:<13.6}|{:<13.6}|{:<13.6}|{:<16}|{:<14}|{:<13}|{:<15}|",
        "full Nearest",
        full_nearest_data[0] as f64 / routes_planned_counter as f64,  // Ensure floating-point division, avg cost
        full_nearest_data[1], //min cost
        full_nearest_data[2], // max cost
        full_nearest_data[3] as f64 / routes_planned_counter as f64,  // Ensure floating-point division, avg time
        full_nearest_data[4], // max cost
        "Yes",
        "No",
        "Yes",
        "No"
    );

    //----------------------------------------------------------------------------------------//
    println!("_______________________________________________________________________________________________________________________________________________________________");

}