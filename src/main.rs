use std::env;
use petgraph::graphmap::{DiGraphMap, UnGraphMap};

mod graphmap_utils;
use graphmap_utils::{min_selection, prune};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    
    let edges = [(0, 1), (1, 2), (2, 4), (2, 5), (3, 4), (3, 6), (3, 7), (5, 8), (7, 8), (9, 10)];
    let graph: UnGraphMap<u8, ()> = UnGraphMap::from_edges(&edges);

    let mut tree = DiGraphMap::<u8, ()>::new();
    for n in graph.nodes(){
        tree.add_node(n);
    }

    let mut gt = graph.clone();
    let mut t = tree.clone();

    let mut num_it = 1;

    loop {   
        //min selection
        let h = min_selection(&gt);
        println!("h{num_it}: {:?}", h);

        //println!("{:?}", h);
        
        //pruning
        let (temp_g, tree) = prune(h, t);
        println!("g{:?}: {:?}", num_it + 1, temp_g);
        
        gt = temp_g;//.clone();
        t = tree;//.clone();

        if gt.edge_count() == 0 {    
            break
        }

        num_it += 1;
    }

    println!("t: {num_it}");
    println!("pruned g2: {:?}", gt);
    println!("T: {:?}", t);


    //println!("{:?}", Dot::with_config(&t, &[Config::EdgeNoLabel]));
    let test_tree = DiGraphMap::<u8, ()>::from_edges(&[(0, 1), (0, 2), (0, 3), (0, 5), (1, 4), (2, 8), (3, 6), (3, 7), (9, 10)]);
    assert_eq!(t.node_count(), test_tree.node_count());
    assert_eq!(t.edge_count(), test_tree.edge_count());

    //no extra edges
    for i in t.nodes(){
        for j in t.nodes(){
            assert_eq!(t.contains_edge(i, j), test_tree.contains_edge(i, j));
        }
    }
}
