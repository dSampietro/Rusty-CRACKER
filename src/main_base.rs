use std::{collections::HashSet, env};
use getopts::Options;
use petgraph::graphmap::{DiGraphMap, UnGraphMap};

mod graphmap_utils_par;
use graphmap_utils_par::{min_selection_base, prune, seed_propagation};

mod input_util;
use input_util::read_from_file;
use rayon::ThreadPoolBuilder;

// ~20 ms / 50k edges

macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    
    type V = u32;
    
    //setup parallelism
    let num_threads = 0;    //let rayon decide
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();
    

    //get cli args
    let args: Vec<String> = std::env::args().collect();

    //get opts
    let mut opts = Options::new();
    opts.optopt("f", "file", "provide the file containg the graph output file name", "FILEPATH");
    opts.optflag("h", "help", "print help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    //handle -h/--help
    if matches.opt_present("h") {
        let brief = format!("Usage: {} FILE [options]", args[0]);
        print!("{}", opts.usage(&brief));
    
        return;
    }

    let filename = matches.opt_str("f");
    if filename.is_none(){
        println!("Please provide a filename");
        return;
    }

    let edges_result = read_from_file::<V>(filename.unwrap().as_str());
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

    debug_println!("g_{:?} #edges: {:?}", num_it, gt.edge_count());
    loop {   
        //min selection
        let h = min_selection_base(&gt);
        debug_println!("h_{:?} #edges: {:?}", num_it, gt.edge_count());

        //pruning
        let (temp_g, tree) = prune(h, t);
        
        gt = temp_g;
        //println!("g{num_it}: {:?}", gt);
        t = tree;

        if gt.edge_count() == 0 {    
            break
        }

        num_it += 1;
        debug_println!("g_{:?} #edges: {:?}", num_it, gt.edge_count());
    }

    let seeds = seed_propagation(t);
    println!("{:?}", now.elapsed().as_millis());
    debug_println!("duration: {:?}", now.elapsed());
    
    debug_println!("t: {num_it}");
    assert_eq!(seeds.len(), graph.node_count());    //all node have a seed => no nodes are lost
    //println!("seeds: {seeds:?}");
    
    let num_conn_comp: HashSet<_> = seeds.values().collect();
    debug_println!("#CC: {:?}\nnum_conn_comp: {:?}", num_conn_comp.len(), num_conn_comp);
}
