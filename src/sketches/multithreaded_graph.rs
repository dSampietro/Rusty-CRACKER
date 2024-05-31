use std::time::Instant;

use dashmap::DashMap;
use petgraph::graph::{NodeIndex, UnGraph};
use rayon::prelude::*;

mod graph_utils;
use graph_utils::get_node_neighborhood;


fn main(){
    let before = Instant::now();
    let num_threads: usize = match std::thread::available_parallelism() {
        Ok(v) => v.into(),
        Err(_) => 1,
    };

    //let mut handles = vec![];

    let edges = [(0, 1), (1, 2), (2, 4), (2, 5), (3, 4), (3, 6), (3, 7), (5, 8), (7, 8)];
    let graph = UnGraph::<(), ()>::from_edges(&edges);

    let results: DashMap<NodeIndex, Vec<NodeIndex>> = DashMap::<NodeIndex, Vec<NodeIndex>>::new();
    
    //rayon
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();

    // Process node indices in parallel using a thread pool with 4 threads
    node_indices.par_iter()
        .with_max_len(num_threads)
        .for_each(|&node| {

            results.insert(node, get_node_neighborhood(&graph, node));
    });

    println!("{:?}", results);
    println!("Elapsed time: {:.2?}", before.elapsed());
}



/*
copy serialization and deserialization
• Like zero-copy, without the limitations
• Unlike abomonation, does not change the memory (e.g., you can map
immutable files into memory)
• Unlike Zerovec, no impact on performance, and you use standard structures
• Unlike rkiv, the structure you deserialize is the structure you serialize, and no
impact on performance
• Requires collaboration from the underlying struct
*/