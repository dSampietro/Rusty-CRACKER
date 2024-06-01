#![allow(dead_code)] 

use std::{fmt::Debug, sync::Mutex};
use dashmap::DashMap;
use petgraph::{ graphmap::{DiGraphMap, GraphMap, NodeTrait, UnGraphMap}, Direction::Outgoing, EdgeType};
use rayon::prelude::*;

/// Get the neighborhood (plus itself) of every node
fn get_neighborhood<V, E, Ty>(g: &GraphMap<V, E, Ty>) -> DashMap<V, Vec<V>>
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
pub fn get_vmins<V: NodeTrait + Send + Sync + Copy>(neighborhoods: &DashMap<V, Vec<V>>) -> DashMap<V, V>
{
    let entries: Vec<_> = neighborhoods.iter().collect();

    /*let v_mins: DashMap<V, V> = entries.iter()
        .filter_map(|(&node, neighbors)|{
            neighbors.into_iter()
                .min()
                .map(|&v_min| (node, v_min))
        })
        .collect();
    */

    let v_mins: DashMap<V, V> = DashMap::new();

    // Use Rayon to find the minimum values in parallel
    entries.par_iter().for_each(|entry| {
        let (&key, vec) = entry.pair();
        if let Some(&min_value) = vec.iter().min() {
            let min_value = key.min(min_value);
            v_mins.insert(key, min_value);
        }
    });

    return v_mins;
}


//TODO: generalize edges
pub fn min_selection<N: NodeTrait + Eq + Send + Sync + Debug>(g: &UnGraphMap<N, ()>) -> DiGraphMap<N, ()> {
    let neighborhoods: DashMap<N, Vec<N>> = get_neighborhood(&g);
    println!("[Min Selection] neighborhoods: {:?}", neighborhoods);

    let v_mins: DashMap<N, N> = get_vmins(&neighborhoods);
    println!("[Min Selection] min neighborhoods: {:?}", v_mins);

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


fn get_outgoing_neighborhood<N: NodeTrait + Send + Sync>(h: &DiGraphMap<N, ()>) -> DashMap<N, Vec<N>>{
    let outgoing_neighborhoods: DashMap<N, Vec<N>> = DashMap::new();
    
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


pub fn prune<N: NodeTrait + Send + Sync + Copy + Debug>(h: DiGraphMap<N, ()>, tree: DiGraphMap<N, ()>) -> (UnGraphMap<N, ()>, DiGraphMap<N, ()>) {
    
    //get outgoing neighborhoods
    let outgoing_neighborhoods: DashMap<N, Vec<N>> = get_outgoing_neighborhood(&h);
    let min_outgoing_neighborhoods = get_vmins(&outgoing_neighborhoods);

    //probable to refactor into a clone-like function
    let mut pruned_graph = UnGraphMap::<N, ()>::with_capacity(h.node_count(), h.edge_count());
    for n in h.nodes(){  //prima del pruning: g_(i+1) ha gli stessi nodi di h(i)
        pruned_graph.add_node(n);
    }

    //add to G(t+1) + deactivation
    let deactivated_nodes_mutex: Mutex<Vec<N>> = Mutex::new(Vec::new()); 
    let entries: Vec<_> = outgoing_neighborhoods.iter().collect();
    let pruned_graph_mutex = Mutex::new(pruned_graph);

    let tree_mutex = Mutex::new(tree);

    entries.par_iter().for_each(|entry|{
        let (u, neighbors) = entry.pair();

        if neighbors.len() > 1 {
            let v_min = *min_outgoing_neighborhoods.get(&u).unwrap();
            
            for v in neighbors{
                if *v != v_min{
                    pruned_graph_mutex.lock().unwrap().add_edge(*v, v_min, ());
                }
            }
        }
        
        //deactivate nodes 
        //TODO: 3rd case (self-loop??)
        if !neighbors.contains(u) {
            let v_min = *min_outgoing_neighborhoods.get(&u).unwrap();
            tree_mutex.lock().unwrap().add_edge(v_min, *u, ());
            println!("Adding to tree: {:?} -> {:?}", v_min, *u);
            deactivated_nodes_mutex.lock().unwrap().push(*u);
        }
    });

    /*for (u, neighbors) in &outgoing_neighborhoods {
        //println!("Pruning @{:?}", *u);
        if neighbors.len() > 1 {
            let v_min = *min_outgoing_neighborhoods.get(&u).unwrap();
            
            for v in neighbors{
                if *v != v_min{
                    pruned_graph.add_edge(*v, v_min, ());
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
    */
    //TODO: unnecessary sort if StableGraph is used
    let mut deactivated_nodes = deactivated_nodes_mutex.into_inner().unwrap_or(Vec::new());
    deactivated_nodes.sort();
    deactivated_nodes.reverse();

    let mut pruned_graph = pruned_graph_mutex.into_inner().unwrap();
    let tree = tree_mutex.into_inner().unwrap();

    //println!("pruned_graph: {:?}", pruned_graph);

    for deactivated in deactivated_nodes{
        println!("Removing node: {:?}", deactivated);
        pruned_graph.remove_node(deactivated);
    }

    return (pruned_graph, tree);
}
