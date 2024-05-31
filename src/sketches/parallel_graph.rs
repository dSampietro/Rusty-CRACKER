use dashmap::DashMap;
use petgraph::graphmap::UnGraphMap;
use rayon::prelude::*;

fn main(){
    let edges = [(0, 1), (1, 2), (2, 4), (2, 5), (3, 4), (3, 6), (3, 7), (5, 8), (7, 8), (9, 10)];
    let graph: UnGraphMap<u8, ()> = UnGraphMap::from_edges(&edges);


    let nodes: Vec<u8> = graph.nodes().collect();
    
    /*let mut ns = HashMap::<u8, Vec<u8>>::new();

    for n in nodes{
        ns.insert(n, graph.neighbors(n).collect());
    }*/

    let ns = DashMap::new(); 
    nodes.par_iter().for_each(|&node| {
        let node_neighbors: Vec<u8> = graph.neighbors(node).collect();
        ns.insert(node, node_neighbors);
    });



    println!("{:?}", ns);
}