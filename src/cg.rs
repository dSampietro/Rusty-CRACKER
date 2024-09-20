#![allow(unused_imports)]
use concurrent_graph::{ConcurrentDiGraph, ConcurrentUnGraph};
use concurrentgraph_utils_rayon::{min_selection_base, par_seed_propagation, prune};
use dashmap::DashSet;
use rayon::{iter, ThreadPoolBuilder};

mod concurrentgraph_utils_rayon;
fn main() {
    let num_threads = 4;
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();


    println!("w/ {num_threads} threads");
    //let graph: ConcurrentDiGraph<i32> = ConcurrentDiGraph::new();

    /*
    // Add edges
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(4, 5);
    graph.add_edge(4, 2);

    {
        let tree = ConcurrentDiGraph::<u16>::new();
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
    }*/

    /*{
        graph.add_edge(0, 0);
        graph.add_edge(1, 0);

        println!("{:?}", graph.get_neighborhoods(true));
    }*/

    {
        let g: ConcurrentUnGraph<u16> = ConcurrentUnGraph::new();
        g.add_edge(0, 1);
        g.add_edge(1, 0);
        println!("g: {:?}", g);
        println!("#edges: {:?}", g.edge_count());

        println!("closed neigh: {:?}", g.get_closed_neighborhoods());
        println!("closed neigh und: {:?}", g.get_closed_neighborhoods_undirected());

        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 2);  // = 2 : (a->b) + (b->a)

        let h = min_selection_base(&g);
        println!("h: {:?}", h);

    }

    /*{
        let (x, t) = prune(graph, ConcurrentDiGraph::new());
        println!("#nodes in h: {:?}", x.node_count());
        println!("h: {:?}", x);
        println!("t: {:?}", t);
    }*/




}