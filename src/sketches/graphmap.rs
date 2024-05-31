use std::{collections::HashMap, hash::Hash};

use petgraph::graphmap::UnGraphMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn get_mins<V: Ord + Hash + Copy>(neigh: &HashMap<V, Vec<V>>) -> HashMap<V, V>{
    let min_neigh = neigh.into_iter()
        .filter_map(|(&node, neighbors)| {
            neighbors.into_iter().min()
                .map(|&v_min| (node, v_min))
        })
        .collect();

    return min_neigh;
}

fn main(){
    let edges = [(0, 1), (1, 2), (2, 4), (2, 5), (3, 4), (3, 6), (3, 7), (5, 8), (7, 8)];
    let mut graph = UnGraphMap::<u8, ()>::from_edges(&edges);
    graph.add_node(9);

    println!("graph: {:?}", graph);

    for n in graph.nodes(){
        println!("{:?}", n);
    }

    let nodes: Vec<u8> = graph.nodes().collect();
    let neigh: HashMap<u8, Vec<u8>> = nodes.par_iter()
        .map(|&node| {
            let neighbors: Vec<_> = graph.neighbors(node).collect();
            (node, neighbors)
        })
        .collect();

    let min_neigh: HashMap<u8, u8> = get_mins(&neigh);
    
    println!("neigh: {:?}", neigh);
    println!("min_neigh: {:?}", min_neigh);

    /* 
    let mut h = DiGraphMap::<u8, ()>::new();
    for n in graph.nodes(){
        h.add_node(n);
    }

    h.remove_node(4);
    h.remove_node(6);
    h.remove_node(7);
    h.remove_node(8);

    h.add_edge(3, 5, ());

    println!("h: {:?}", h);
    println!("edgecount h: {:?}", h.edge_count());
    */

    
}