use std::env;
use petgraph::graphmap::{DiGraphMap, UnGraphMap};

mod graphmap_utils_par;
use graphmap_utils_par::{min_selection, prune, seed_propagation};

mod input_util;
use input_util::read_from_file;
use rayon::ThreadPoolBuilder;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    
    //setup parallelism
    let num_threads = 4;
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();
    
    type V = u16;

    
    let filename = "files/soc-wiki-vote.mtx";
    let edges_result = read_from_file::<V>(filename);
    if edges_result.is_err(){
        println!("Error reading edges from file: {:?}", edges_result.err());
        return ;
    }
    
    let edges = edges_result.unwrap_or(Vec::new());
    
    let graph: UnGraphMap<V, ()> = UnGraphMap::from_edges(&edges);

    let mut tree = DiGraphMap::<V, ()>::new();
    for n in graph.nodes(){
        tree.add_node(n);
    }

    let mut gt = graph.clone();
    let mut t = tree.clone();

    let mut num_it = 1;

    let now = std::time::Instant::now();

    loop {   
        //min selection
        let h = min_selection(&gt);
        //println!("h{num_it}: {:?}", h);

        //println!("{:?}", h);
        
        //pruning
        let (temp_g, tree) = prune(h, t);
        //println!("g{:?}: {:?}", num_it + 1, temp_g);
        
        gt = temp_g;//.clone();
        t = tree;//.clone();

        if gt.edge_count() == 0 {    
            break
        }

        num_it += 1;
    }

    let seeds = seed_propagation(t);
    
    println!("duration: {:?}", now.elapsed());
    
    println!("t: {num_it}");
    //println!("seeds: {seeds:?}");
    assert_eq!(seeds.len(), graph.node_count());    //all node have a seed => no nodes are lost

}
