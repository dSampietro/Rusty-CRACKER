#![allow(dead_code)] 

use std::{collections::HashMap, fmt::Debug};
use dashmap::DashMap;
use petgraph::{ graphmap::{DiGraphMap, GraphMap, NodeTrait, UnGraphMap}, Direction::{Incoming, Outgoing}, EdgeType};
use rayon::prelude::*;

/// Get the neighborhood (plus itself) of every node
pub fn get_neighborhood<V: NodeTrait, E, Type: EdgeType>(g: &GraphMap<V, E, Type>) -> HashMap<V, Vec<V>> {
    let nodes: Vec<V> = g.nodes().collect();

    let neigh: HashMap<V, Vec<V>> = nodes.iter()
        .map(|&node| {
            let mut neighbors: Vec<V> = g.neighbors(node).collect();
            neighbors.push(node);
            (node, neighbors)
        })
        .collect();

    return neigh;
}


fn get_neighborhood_par<V, E, Ty>(g: &GraphMap<V, E, Ty>) -> DashMap<V, Vec<V>>
where
    V: NodeTrait + Send + Sync,
    E: Send + Sync,
    Ty: EdgeType + Send + Sync
{
    let neighbors = DashMap::<V, Vec<V>>::new();
    
    let nodes: Vec<V> = g.nodes().collect();

    nodes.par_iter().for_each(|&node|{
        let mut node_neighbors: Vec<V> = g.neighbors(node).collect();
        node_neighbors.push(node);
        neighbors.insert(node, node_neighbors);
    });

    return neighbors;
}

/// Get the min neighbor of every node
pub fn get_vmins<V: NodeTrait + Debug + Copy>(neighborhoods: &HashMap<V, Vec<V>>) -> HashMap<V, V>{
    
    /*let v_mins: HashMap<V, V> = neighborhoods.iter()
        .map(
        |(&node, neighbors)| {
            println!("getting v_min of {:?}", node);
            
            //a node may have be isolate => no neighbor => unsafe to unwrap => check is is_some
            let v_min = neighbors.iter().min();
            
            if v_min.is_some() {
                let min_neigh = min(*v_min.unwrap(), node);
                (node, min_neigh);
            }            
        })
        .collect();*/
    
    let v_mins: HashMap<V, V> = neighborhoods.into_iter()
        .filter_map(|(&node, neighbors)|{
            neighbors.into_iter()
                .min()
                .map(|&v_min| (node, v_min))
        })
        .collect();

    return v_mins;
}


//TODO: generalize edges
pub fn min_selection<N: NodeTrait + Eq + std::fmt::Debug>(g: &UnGraphMap<N, ()>) -> DiGraphMap<N, ()> {
    let neighborhoods: HashMap<N, Vec<N>> = get_neighborhood(&g);
    //println!("[Min Selection] neighborhoods: {:?}", neighborhoods);

    let v_mins: HashMap<N, N> = get_vmins(&neighborhoods);
    //println!("[Min Selection] min neighborhoods: {:?}", v_mins);

    // create directed graph h
    let mut h: DiGraphMap<N, ()> = DiGraphMap::new();
    
    // for graphMap: no need to add nodes; when adding edges, it adds nodes

    //add edges
    for (u, neighbors) in neighborhoods{
        let v_min_opt = v_mins.get(&u);
        
        if v_min_opt.is_none(){
            continue;
        }
        
        let v_min = *v_min_opt.unwrap();
                
        h.add_edge(u, v_min, ());
        for node in neighbors {
            //println!("adding: {:?} -> {:?}", node, v_min);
            h.add_edge(node, v_min, ());
        }
    }

    return h;
}


fn get_outgoing_neighborhood<N: NodeTrait>(h: &DiGraphMap<N, ()>) -> HashMap<N, Vec<N>>{
    let mut outgoing_neighborhoods: HashMap<N, Vec<N>> = HashMap::new();
    
    for n in h.nodes(){
        //outgoing_neighbour = {v | (u->v) € H}
        let mut local_outgoing = Vec::<N>::new();

        for dest in h.neighbors_directed(n, Outgoing){
            local_outgoing.push(dest);
        }

        outgoing_neighborhoods.insert(n, local_outgoing);
    }

    return outgoing_neighborhoods;
}


pub fn prune<N: NodeTrait + Copy + Debug>(h: DiGraphMap<N, ()>, mut tree: DiGraphMap<N, ()>) -> (UnGraphMap<N, ()>, DiGraphMap<N, ()>) {
    
    //get outgoing neighborhoods
    let outgoing_neighborhoods: HashMap<N, Vec<N>> = get_outgoing_neighborhood(&h);
    let min_outgoing_neighborhoods = get_vmins(&outgoing_neighborhoods);

    let mut g2 = UnGraphMap::<N, ()>::with_capacity(h.node_count(), h.edge_count());
    
    for n in h.nodes(){  //prima del pruning: g_(i+1) ha gli stessi nodi di h(i)
        g2.add_node(n);
    }

    //add to G(t+1) + deactivation
    let mut deactivated_nodes: Vec<N> = Vec::new(); 

    for (u, neighbors) in &outgoing_neighborhoods {
        //println!("Pruning @{:?}", *u);
        if neighbors.len() > 1 {
            let v_min = *min_outgoing_neighborhoods.get(&u).unwrap();
            
            for v in neighbors{
                if *v != v_min{
                    g2.add_edge(*v, v_min, ());
                }
            }
        }
        
        //deactivate nodes 
        //TODO: 3rd case (self-loop??)
        if !neighbors.contains(u) {
            let v_min = *min_outgoing_neighborhoods.get(&u).unwrap();
            tree.add_edge(v_min, *u, ());
            //println!("Adding to tree: {:?} -> {:?}", v_min, *u);
            deactivated_nodes.push(*u);
        }
    }
    
    //TODO: unnecessary sort if StableGraph is used
    deactivated_nodes.sort();
    deactivated_nodes.reverse();

    //println!("g2: {:?}", g2);

    for deactivated in deactivated_nodes{
        //println!("Removing node: {:?}", deactivated);
        g2.remove_node(deactivated);
    }

    return (g2, tree);
}

pub fn seed_propagation<V: NodeTrait + Debug>(tree: DiGraphMap<V, ()>) -> HashMap<V, V>{
    let mut res: HashMap<V, V> = HashMap::new();

    let mut nodes: Vec<V> = tree.nodes().collect();
    //assert_eq!(nodes.len(), tree.node_count());
    //println!("Nodes: {:?}", nodes);
    nodes.sort_unstable();  //no duplicates => can use unstable sorting => more efficient

    while nodes.len() != 0 {    
        let min_node = nodes[0];        //sorted nodes => min node will always be the 1st
        let incoming_edge = tree.edges_directed(min_node, Incoming);    //either 0 or 1 edge
        //println!("{:?}", incoming_edge);

        for edge in incoming_edge{
            //println!("Node {:?}, edge {:?}", min_node, edge);

            if res.contains_key(&edge.0){
                let parent_seed = res.get(&edge.0).unwrap();
                res.insert(min_node, *parent_seed);
            }
            else{
                res.insert(min_node, edge.0);
            }
        }

        //no incoming edge into node => node is root of a tree
        if res.contains_key(&min_node) == false{
            res.insert(min_node, min_node);
        }

        nodes.remove(0);
    }

    return res;
}
