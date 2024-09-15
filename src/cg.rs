#![allow(unused_imports)]
mod concurrent_graph;
use concurrent_graph::{ConcurrentDiGraph, ConcurrentUnGraph};
use concurrentgraph_utils_rayon::par_seed_propagation;
use dashmap::DashSet;
use rand::Rng;
use rayon::{iter, ThreadPoolBuilder};

mod concurrentgraph_utils_rayon;
fn main() {
    let num_threads = 4;
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();


    println!("w/ {num_threads} threads");
    let graph: ConcurrentUnGraph<i32> = ConcurrentDiGraph::new_directed();

    // Add edges
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(4, 5);
    graph.add_edge(4, 2);

    {
        let tree = ConcurrentDiGraph::<u16>::new_directed();
        tree.add_edge(0, 1);
        tree.add_edge(0, 2);
        tree.add_edge(0, 3);
        tree.add_edge(0, 5);
        tree.add_edge(1, 4);
        tree.add_edge(2, 8);
        tree.add_edge(3, 6);
        tree.add_edge(3, 7);

        tree.add_edge(9, 10);

        println!("nodes: {:?}", tree.nodes());
        println!("#edges: {:?}", tree.edge_count());

        let seeds_map = par_seed_propagation(&tree);

        println!("seeds: {:?}", seeds_map);
    }




}