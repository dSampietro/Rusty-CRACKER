use std::env;

use petgraph::{dot::{Config, Dot}, graphmap::{DiGraphMap, UnGraphMap}};

mod graphmap_utils;
//use graphmap_utils::prune;

use crate::graphmap_utils::{min_selection, prune};


fn main(){
    env::set_var("RUST_BACKTRACE", "1");

    /*
    let mut x = UnGraphMap::<u8, ()>::new();
    x.add_node(0);

    let neigh = get_neighborhood(&x);
    println!("neighbors: {:?}", neigh);

    let min_neigh = get_vmins(&neigh);
    println!("min_neighbors: {:?}", min_neigh);

    println!("min_selection g: {:?}", min_selection(&x));*/


    
    let edges = [(0, 1), (1, 2), (2, 4), (2, 5), (3, 4), (3, 6), (3, 7), (5, 8), (7, 8)];
    let g1: UnGraphMap<u8, ()> = UnGraphMap::from_edges(&edges);
    let _tree: DiGraphMap<u8, ()> = DiGraphMap::new();


    let h1 = min_selection(&g1);
    println!("h1: {:?}", h1);
    assert_eq!(h1.edge_count(), 20);

    /*println!("h1\n{:?}", Dot::with_config(&h1, &[Config::EdgeNoLabel]));
    

    let g2 = prune(h1, tree);
    println!("g2: {:?}", g2);*/



}