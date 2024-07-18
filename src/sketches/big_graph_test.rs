mod input_util;
use input_util::read_from_file;


use std::collections::HashSet;
use petgraph::graphmap::{DiGraphMap, NodeTrait, UnGraphMap};


fn to_directed<N: NodeTrait + Send + Sync>(g: &UnGraphMap<N, ()>) -> DiGraphMap<N, ()> {
    let mut res: DiGraphMap<N, ()> = DiGraphMap::with_capacity(g.node_count(), 2*g.edge_count());

    for (a, b, _) in g.all_edges() {
        res.add_edge(a, b, ());
        res.add_edge(b, a, ());
    }
    
    //assert_eq!(res.edge_count(), 2 * g.edge_count());
    return res;
}


fn main(){
    type V = u32;
    let filename = "../files/rec-eachmovie.mtx";
    
    let edges = read_from_file::<V>(filename).unwrap();
    let graph: UnGraphMap<V, ()> = UnGraphMap::from_edges(&edges);


    println!("#edges: {:?}", edges.len());
    //println!("#edges as_directed: {:?}", 2*graph.edge_count());

    println!("#edges in graph: {:?}", graph.edge_count());

    println!("delta edges: {:?}", edges.len() - graph.edge_count());


    /*let mut edge_set = HashSet::new();
    for &edge in &edges {
        if !edge_set.insert(edge) {
            println!("Duplicate edge found: {:?}", edge);
        }
    }
    
    println!("#unique edges: {:?}", edge_set.len());
    */

    let directed = to_directed(&graph);

    println!("#edges directed: {:?}", directed.edge_count());

    let mut i: usize = 0;

    for (a,b,_) in graph.all_edges(){
        if directed.contains_edge(a, b)  {
            i += 1;
        }
        if directed.contains_edge(b, a) {
            i += 1;
        }
        
    }

    println!("t: {i}");


    //assert_eq!(directed.edge_count(), 2*graph.edge_count());

}