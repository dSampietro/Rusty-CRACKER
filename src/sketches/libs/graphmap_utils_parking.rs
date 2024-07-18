#![allow(dead_code)]
#![allow(clippy::needless_return)]

use dashmap::DashMap;
use petgraph::{
    graphmap::{DiGraphMap, GraphMap, NodeTrait, UnGraphMap},
    Direction::{Incoming, Outgoing},
    EdgeType,
};

use rayon::prelude::*;
use std::{collections::HashMap, fmt::Debug};
use parking_lot::Mutex;

/// Get the closed neighbourhood (neighborhood + node) of every node
fn get_neighborhood<V, E, Ty>(g: &GraphMap<V, E, Ty>) -> DashMap<V, Vec<V>>
where
    V: NodeTrait + Send + Sync,
    E: Send + Sync,
    Ty: EdgeType + Send + Sync,
{
    let neighbors = DashMap::<V, Vec<V>>::new();

    let nodes: Vec<V> = g.nodes().collect();

    // considerare come vicini nodi tc esiste arco uscente da n
    nodes.par_iter().for_each(|&node| {
        let mut node_neighbors: Vec<V> = g.neighbors_directed(node, Outgoing).collect();
        node_neighbors.push(node); //plus itself
        neighbors.insert(node, node_neighbors);
    });

    return neighbors;
}

/// Get the neighborhood of every node
fn get_neighborhood_base<V, E, Ty>(g: &GraphMap<V, E, Ty>) -> DashMap<V, Vec<V>>
where
    V: NodeTrait + Send + Sync,
    E: Send + Sync,
    Ty: EdgeType + Send + Sync,
{
    let neighbors = DashMap::<V, Vec<V>>::new();

    let nodes: Vec<V> = g.nodes().collect();

    nodes.par_iter().for_each(|&node| {
        let node_neighbors: Vec<V> = g.neighbors(node).collect();
        neighbors.insert(node, node_neighbors);
    });

    return neighbors;
}

/// Get the min neighbor of every node
pub fn get_vmins<V: NodeTrait + Send + Sync + Copy>(
    neighborhoods: &DashMap<V, Vec<V>>,
) -> DashMap<V, V> {
    let v_mins: DashMap<V, V> = DashMap::new();

    //TODO: direct iteration
    let entries: Vec<_> = neighborhoods.iter().collect();

    entries.par_iter().for_each(|entry| {
        let (&key, vec) = entry.pair();
        if let Some(&min_value) = vec.iter().min() {
            let min_value = key.min(min_value);
            v_mins.insert(key, min_value);
        }
    });

    return v_mins;
}

//DEPRECATED
//TODO: generalize edges
pub fn min_selection_base<N>(g: &UnGraphMap<N, ()>) -> DiGraphMap<N, ()>
where
    N: NodeTrait + Eq + Send + Sync + Debug,
{
    let neighborhoods: DashMap<N, Vec<N>> = get_neighborhood(g);
    let v_mins: DashMap<N, N> = get_vmins(&neighborhoods);

    // create directed graph h
    let mut h: DiGraphMap<N, ()> = DiGraphMap::new();

    //add edges
    for (u, neighbors) in neighborhoods {
        let v_min_option = v_mins.get(&u);

        if v_min_option.is_none() {
            continue;
        }

        let v_min = *v_min_option.unwrap();

        // base
        h.add_edge(u, v_min, ());
        for node in neighbors {
            //eprintln!("[h] adding: {:?} -> {:?}", node, v_min);
            h.add_edge(node, v_min, ());
        }
    }

    return h;
}

// with Edge Pruning
pub fn min_selection_ep<N, D>(g: &GraphMap<N, (), D>) -> DiGraphMap<N, ()>
where
    N: NodeTrait + Eq + Send + Sync + Debug,
    D: EdgeType + Send + Sync,
{
    let neighborhoods: DashMap<N, Vec<N>> = get_neighborhood_base(g);
    let v_mins: DashMap<N, N> = get_vmins(&neighborhoods);

    // create directed graph h
    let mut h: DiGraphMap<N, ()> = DiGraphMap::new();

    //add edges
    let mut neighborhoods_entries: Vec<_> = neighborhoods.iter().collect();
    neighborhoods_entries.sort_by(|a, b| a.key().cmp(b.key()));

    for entry in neighborhoods_entries {
        let &&n = &entry.key();
        let &neighbors = &entry.value();

        let n_min_opt = v_mins.get(&n);
        if n_min_opt.is_none() {
            continue;
        }
        let n_min = *n_min_opt.unwrap();

        //when a node is the minimum of its neighbourhood, it does not need to notify this information to its neighbours
        if n == n_min {
            for z in neighbors {
                let z_min = *v_mins.get(z).unwrap();

                //when a node u is the local minimum in NN(u), [u = u_min] there are two exclusive cases
                if z_min == n {
                    h.add_edge(*z, n, ());
                    //eprintln!("[caso A] adding edge {:?}->{:?}", *z, n);
                } else {
                    h.add_edge(*z, z_min, ());
                    //eprintln!("[caso B] adding edge {:?}->{:?}", *z, z_min);

                    h.add_edge(n, z_min, ());
                    //eprintln!("[caso B] adding edge {:?}->{:?}", n, z_min);
                }

                //eprintln!("removing {:?}", &z);
            }
        } else {
            h.add_edge(n, n_min, ()); // => get_neighborhood return <neighbors + node>
                                      //eprintln!("[caso C] adding edge {:?}->{:?}", n, n_min);
            for node in neighbors {
                //eprintln!("adding: {:?} -> {:?}", node, v_min);
                h.add_edge(*node, n_min, ());
                //eprintln!("[caso C] adding edge {:?}->{:?}", *node, n_min);
            }
        }
    }
    return h;
}

fn get_outgoing_neighborhood<N: NodeTrait + Send + Sync>(
    h: &DiGraphMap<N, ()>,
) -> DashMap<N, Vec<N>> {
    let outgoing_neighborhoods: DashMap<N, Vec<N>> = DashMap::new();

    /*for n in h.nodes(){
        //outgoing_neighbour = {v | (u->v) € H}
        let mut local_outgoing = Vec::<N>::new();

        for dest in h.neighbors_directed(n, Outgoing){
            local_outgoing.push(dest);
        }

        outgoing_neighborhoods.insert(n, local_outgoing);
    }*/

    let nodes: Vec<_> = h.nodes().collect();
    nodes.par_iter().for_each(|&n| {
        let mut local_outgoing: Vec<N> = Vec::new();

        for dest in h.neighbors_directed(n, Outgoing) {
            local_outgoing.push(dest);
        }

        outgoing_neighborhoods.insert(n, local_outgoing);
    });

    return outgoing_neighborhoods;
}

pub fn prune<N: NodeTrait + Send + Sync + Copy + Debug>(
    h: DiGraphMap<N, ()>,
    tree: DiGraphMap<N, ()>,
) -> (UnGraphMap<N, ()>, DiGraphMap<N, ()>) {
    //eprintln!("Pruning");
    //get outgoing neighborhoods
    let outgoing_neighborhoods: DashMap<N, Vec<N>> = get_outgoing_neighborhood(&h);

    let min_outgoing_neighborhoods = get_vmins(&outgoing_neighborhoods);

    let pruned_graph = UnGraphMap::<N, ()>::with_capacity(h.node_count(), h.edge_count());

    /*
    no need to add node to pruned_graph
    when par_iterating, every node will be visited => every node will be added
    */

    //add to G(t+1) + deactivation
    let deactivated_nodes_mutex: Mutex<Vec<N>> = Mutex::new(Vec::new());
    let entries: Vec<_> = outgoing_neighborhoods.iter().collect(); //TODO: direct iteration
    let pruned_graph_mutex = Mutex::new(pruned_graph);

    let tree_mutex = Mutex::new(tree);

    entries.par_iter().for_each(|entry| {
        let (u, neighbors) = entry.pair();

        if neighbors.len() > 1 {
            let v_min = *min_outgoing_neighborhoods.get(u).unwrap();

            for v in neighbors {
                if *v != v_min {
                    pruned_graph_mutex.lock().add_edge(*v, v_min, ());
                    //eprintln!("[g]: adding edge {:?} -> {:?}", *v, v_min);
                }
            }
        }

        //deactivate nodes
        if !neighbors.contains(u) {
            let v_min_opt = min_outgoing_neighborhoods.get(u);
            //eprintln!("v_min_opt: {:?}", v_min_opt);
            if v_min_opt.is_none() {
                //eprintln!("min_outgoing_neighborhoods: do not found u");
                return;
            }

            let v_min = *v_min_opt.unwrap();
            tree_mutex.lock().add_edge(v_min, *u, ());
            //eprintln!("Adding to tree: {:?} -> {:?}", v_min, *u);

            deactivated_nodes_mutex.lock().push(*u);
        }

        //TODO: 3rd case (node is seed: still active but NN(u) = {u})
        /*if (neighbors.len() == 1) && neighbors.contains(u) {
            deactivated_nodes_mutex.lock().unwrap()
                .push(*u);
        }*/
    });

    let deactivated_nodes = deactivated_nodes_mutex.into_inner();
    //deactivated_nodes.sort_unstable_by(|a, b| b.cmp(a));    //sort + reverse

    let mut pruned_graph = pruned_graph_mutex.into_inner();
    let tree = tree_mutex.into_inner();

    for deactivated in deactivated_nodes {
        //eprintln!("Removing node: {:?}", deactivated);
        pruned_graph.remove_node(deactivated);
    }

    return (pruned_graph, tree);
}

pub fn prune_os<N: NodeTrait + Send + Sync + Copy + Debug>(
    h: DiGraphMap<N, ()>,
    tree: DiGraphMap<N, ()>,
) -> (DiGraphMap<N, ()>, DiGraphMap<N, ()>) {
    //get outgoing neighborhoods
    let outgoing_neighborhoods: DashMap<N, Vec<N>> = get_outgoing_neighborhood(&h);

    let min_outgoing_neighborhoods = get_vmins(&outgoing_neighborhoods);

    let pruned_graph = DiGraphMap::<N, ()>::with_capacity(h.node_count(), h.edge_count());

    //add to G(t+1) + deactivation
    let deactivated_nodes_mutex: Mutex<Vec<N>> = Mutex::new(Vec::new());
    let entries: Vec<_> = outgoing_neighborhoods.iter().collect(); //TODO: direct iteration
    let pruned_graph_mutex = Mutex::new(pruned_graph);

    let tree_mutex = Mutex::new(tree);

    entries.par_iter().for_each(|entry| {
        let (u, neighbors) = entry.pair();

        if neighbors.len() > 1 {
            let v_min = *min_outgoing_neighborhoods.get(u).unwrap();

            for v in neighbors {
                if *v != v_min {
                    pruned_graph_mutex.lock().add_edge(*v, v_min, ());
                }
            }
        }

        //deactivate nodes
        if !neighbors.contains(u) {
            let v_min_opt = min_outgoing_neighborhoods.get(u);
            if v_min_opt.is_none() {
                return;
            }

            let v_min = *v_min_opt.unwrap();
            tree_mutex.lock().add_edge(v_min, *u, ());

            deactivated_nodes_mutex.lock().push(*u);
        }

        //TODO: 3rd case (node is seed: still active but NN(u) = {u})
        /*if (neighbors.len() == 1) && neighbors.contains(u) {
            deactivated_nodes_mutex.lock().unwrap()
                .push(*u);
        }*/
    });

    let mut deactivated_nodes = deactivated_nodes_mutex.into_inner();
    deactivated_nodes.sort_unstable_by(|a, b| b.cmp(a)); //sort + reverse

    let mut pruned_graph = pruned_graph_mutex.into_inner();
    let tree = tree_mutex.into_inner();

    //eprintln!("pruned_graph: {:?}", pruned_graph);

    for deactivated in deactivated_nodes {
        //eprintln!("Removing node: {:?}", deactivated);
        pruned_graph.remove_node(deactivated);
    }

    return (pruned_graph, tree);
}

pub fn seed_propagation<V: NodeTrait + Debug>(tree: DiGraphMap<V, ()>) -> HashMap<V, V> {
    let mut seeds_map: HashMap<V, V> = HashMap::new();

    let mut nodes: Vec<V> = tree.nodes().collect();
    nodes.sort_unstable(); //no duplicates => can use unstable sorting => more efficient

    //while + removal
    while !nodes.is_empty() {
        let min_node = nodes[0]; //sorted nodes => min node will always be the 1st
        let incoming_edge = tree.edges_directed(min_node, Incoming); //either 0 or 1 edge
                                                                     //eprintln!("{:?}", incoming_edge);

        for edge in incoming_edge {
            //eprintln!("Node {:?}, edge {:?}", min_node, edge);

            if seeds_map.contains_key(&edge.0) {
                let parent_seed = seeds_map.get(&edge.0).unwrap();
                seeds_map.insert(min_node, *parent_seed);
            } else {
                seeds_map.insert(min_node, edge.0);
            }
        }

        //no incoming edge into node => node is root of a tree
        seeds_map
            .entry(min_node) // if min_node not in seeds_map
            .or_insert(min_node); // insert

        nodes.remove(0);
    }

    return seeds_map;
}

/// transform an undirected graph into a directed one
pub fn as_directed<N: NodeTrait + Send + Sync>(g: &UnGraphMap<N, ()>) -> DiGraphMap<N, ()> {
    let mut res: DiGraphMap<N, ()> = DiGraphMap::with_capacity(g.node_count(), 2 * g.edge_count());

    for (a, b, _) in g.all_edges() {
        res.add_edge(a, b, ());
        res.add_edge(b, a, ());
    }

    //assert_eq!(res.edge_count(), 2 * g.edge_count()); //TODO: problem with big graphs (rec-eachmovie.mtx)
    return res;
    /*
    let nodes: Vec<_> = g.nodes().collect();
    let res_mutex = Mutex::new(res);
    nodes.par_iter().for_each(|&n| {
        for e in g.neighbors(n){
            res_mutex.lock().unwrap()
                .add_edge(n, e, ());

            res_mutex.lock().unwrap()
                .add_edge(e, n, ());
        }
    });

    return res_mutex.into_inner().unwrap();
    */
}
