use std::{collections::HashMap, time::Instant};
use petgraph::graph::{NodeIndex, UnGraph};

mod graph_utils;
use graph_utils::get_neighborhood;


fn main(){
    let before = Instant::now();

    let edges = [(0, 1), (1, 2), (2, 4), (2, 5), (3, 4), (3, 6), (3, 7), (5, 8), (7, 8)];
    let graph = UnGraph::<(), ()>::from_edges(&edges);

    let results: HashMap<NodeIndex, Vec<NodeIndex>> = get_neighborhood(&graph);
    
    println!("{:?}", results);
    println!("Elapsed time: {:.2?}", before.elapsed());
}