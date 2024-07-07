use std::collections::HashMap;
use petgraph::{graphmap::{DiGraphMap, GraphMap, NodeTrait, UnGraphMap}, Direction::Outgoing, EdgeType};
use rayon::prelude::*;

fn get_neighborhood<V: NodeTrait, E, D: EdgeType>(g: &GraphMap<V, E, D>) -> HashMap<V, Vec<V>> {
    let mut neigh: HashMap<V, Vec<V>> = HashMap::new();
    
    g.nodes().into_iter().for_each(|n| {
        let mut local_neigh: Vec<V> = Vec::new();
        
        g.neighbors_directed(n, Outgoing).into_iter().for_each(|e| {
            local_neigh.push(e)
        });
        neigh.insert(n, local_neigh);
    });

    return neigh;
}


fn main(){
    let edges = [(0, 1), (1, 2), (2, 4), (2, 5), (3, 4), (3, 6), (3, 7), (5, 8), (7, 8), (9, 10)];
    let ungraph: UnGraphMap<u8, ()> = UnGraphMap::from_edges(&edges);
    let digraph: DiGraphMap<u8, ()> = DiGraphMap::from_edges(&edges);


    println!("[U] #edges: {:?}", ungraph.edge_count());
    /*let neigh: HashMap<u8, Vec<u8>> = get_neighborhood(&ungraph);
    neigh.into_par_iter().for_each(|(n, neigh)| {
        println!("{:?} -> {:?}", n, neigh);
    });*/

    /*neigh.iter().for_each(|(n, neigh)| {
        println!("{:?} -> {:?}", n, neigh);
    });*/

}