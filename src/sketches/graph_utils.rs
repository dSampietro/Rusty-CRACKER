#![allow(dead_code)] 

use std::{cmp::min, collections::HashMap};
use petgraph::{ graph::NodeIndex, stable_graph::{StableDiGraph, StableGraph, StableUnGraph}, Direction::Outgoing, EdgeType};

/// Find the neighborhood of node NodeIndex
pub fn get_node_neighborhood<V, E, Type: EdgeType>(g: &StableGraph<V, E, Type>, node:NodeIndex) -> Vec<NodeIndex>{
    return g.neighbors(node).collect();
}

pub fn get_neighborhood<V, E, Type: EdgeType>(g: &StableGraph<V, E, Type>) -> HashMap<NodeIndex, Vec<NodeIndex>> {
    let mut neighborhoods: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();

    let nodes = g.node_indices();
    for n in nodes.into_iter(){
        let nn = g.neighbors(n);
        
        //TODO: use g.neighbor.collect instead of manually creating vec
        let mut local_neighbors = Vec::<NodeIndex>::new();  
        
        for x in nn{
            local_neighbors.push(x);
        }

        neighborhoods.insert(n, local_neighbors);
    }

    return neighborhoods;
}

fn get_vmins(neighborhoods: &HashMap<NodeIndex, Vec<NodeIndex>>) -> HashMap<NodeIndex, NodeIndex>{
    let mut v_mins: HashMap<NodeIndex, NodeIndex> = HashMap::new();
    for (node, neighbors) in neighborhoods{
        //TODO: min must be between neighbors and self

        if let Some(&min_neigh) = neighbors.iter().min(){
            let min = min(min_neigh, *node);
            v_mins.insert(*node, min);
        }
    }

    return v_mins;
}


fn un_to_directed<N>(g: &StableUnGraph<N, ()>) -> StableDiGraph<N, ()> {
    let h = StableDiGraph::new();


    return h;
}

//TODO: generalize edges
pub fn min_selection<N: Copy + std::fmt::Debug>(g: &StableUnGraph<N, ()>) -> StableDiGraph<N, ()> {
    println!("{:?}", g);
    for n in g.node_indices(){
        println!("{:?}", n);
    }

    let neighborhoods: HashMap<NodeIndex, Vec<NodeIndex>> = get_neighborhood(&g);
    let v_mins: HashMap<NodeIndex, NodeIndex> = get_vmins(&neighborhoods);

    // create directed graph h
    
    //Problem: when adding nodes from g, it skips absent NodeIndex
    let mut h: StableDiGraph<N, ()> = StableDiGraph::new();
    
    for n in g.node_indices(){
        h.add_node(*g.node_weight(n).unwrap()); //iterating on g nodes implies they exist => safe unwrap
    }

    //add edges
    for (k, v) in neighborhoods{
        let v_min = *v_mins.get(&k).unwrap();
                
        h.update_edge(k, v_min, ());
        for node in v {
            //println!("adding: {:?} -> {:?}", node, v_min);
            h.update_edge(node, v_min, ());
        }
    }

    return h;
}


fn get_outgoing_neighborhood<N>(h: &StableDiGraph<N, ()>) -> HashMap<NodeIndex, Vec<NodeIndex>>{
    let mut outgoing_neighborhoods: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();
    
    for n in h.node_indices(){
        //outgoing_neighbour = {v | (u->v) € H}
        let mut local_outgoing = Vec::<NodeIndex>::new();

        for dest in h.neighbors_directed(n, Outgoing){
            local_outgoing.push(dest);
        }

        outgoing_neighborhoods.insert(n, local_outgoing);
    }

    return outgoing_neighborhoods;
}


pub fn prune<N: Copy>(h: StableDiGraph<N, ()>, mut tree: StableDiGraph<N, ()>) -> (StableUnGraph<N, ()>, StableDiGraph<N, ()>) {
    
    //get outgoing neighborhoods
    let outgoing_neighborhoods: HashMap<NodeIndex, Vec<NodeIndex>> = get_outgoing_neighborhood(&h);
    let min_outgoing_neighborhoods = get_vmins(&outgoing_neighborhoods);

    let mut g2 = StableUnGraph::<N, ()>::with_capacity(h.node_count(), h.edge_count());
    for n in h.node_indices(){  //prima del pruning: g_(i+1) ha gli stessi nodi di h(i)
        g2.add_node(*h.node_weight(n).unwrap());
    }

    //add to G(t+1) + deactivation
    let mut deactivated_nodes: Vec<NodeIndex> = Vec::<NodeIndex>::new(); 

    for (u, neighbors) in &outgoing_neighborhoods {
        println!("Pruning @{:?}", *u);
        if neighbors.len() > 1 {
            let v_min = *min_outgoing_neighborhoods.get(&u).unwrap();
            
            for v in neighbors{
                if *v != v_min{
                    g2.update_edge(*v, v_min, ());
                }
            }
        }
        
        //deactivate nodes 
        //TODO: 3rd case (self-loop??)
        if !neighbors.contains(u) {
            let v_min = *min_outgoing_neighborhoods.get(&u).unwrap();
            tree.add_edge(v_min, *u, ());
            println!("Adding to tree: {:?} -> {:?}", v_min, *u);
            deactivated_nodes.push(*u);
        }
    }
    
    //TODO: unnecessary sort if StableGraph is used
    deactivated_nodes.sort();
    deactivated_nodes.reverse();

    //println!("g2: {:?}", g2);

    for deactivated in deactivated_nodes{
        g2.remove_node(deactivated);
    }

    return (g2, tree);
}

