#![allow(dead_code)]
use dashmap::{DashMap, DashSet};
use rayon::prelude::*;
use std::{collections::{HashMap, HashSet}, fmt::Debug};

//use crate::concurrent_graph::{ConcurrentDiGraph, ConcurrentGraph, ConcurrentUnGraph, NodeTrait};
use concurrent_graph::{ConcurrentDiGraph, ConcurrentUnGraph, NodeTrait, GraphTrait};

/// Get the min neighbor of every node
fn get_vmins<G, V>(graph: &G) -> DashMap<V, V> 
where G: GraphTrait<V>, V: NodeTrait + Send + Sync {
    let v_mins: DashMap<V, V> = DashMap::with_capacity(graph.node_count());

    //since get_vmins should be the same with Un/Direceted graphs, get_closed_neighborhoods should be the same in both cases
    // get_closed_neighborhoods => get_closed_neighborhoods_undirected
    graph.get_closed_neighborhoods_undirected().par_iter().for_each( |entry| {
        let key = entry.key();
        let v_min = entry.value().iter().min().unwrap();

        v_mins.insert(*key, *v_min);
    });

    v_mins
}

pub fn min_selection_base<N>(g: &ConcurrentUnGraph<N>) -> ConcurrentDiGraph<N>
where
    N: NodeTrait + Eq + Send + Sync + Debug,
{
    let neighborhoods: DashMap<N, HashSet<N>> = g.get_closed_neighborhoods();
    //println!("[MS]: neighborhoods = {neighborhoods:?}"); //debug

    let v_mins: DashMap<N, N> = get_vmins(g);

    // create directed graph h
    let h: ConcurrentDiGraph<N> = ConcurrentDiGraph::with_capacity(g.node_count(), g.edge_count());

    //add edges
    neighborhoods.par_iter().for_each(|entry| {
        let (u, neighbors) = entry.pair();
        //println!("[MS- Iter neighborhoods]: {u:?}");  //debug
        let v_min_option = v_mins.get(u);

        if v_min_option.is_none() {
            return;
        }

        let v_min = *v_min_option.unwrap();

        // base
        h.add_edge(*u, v_min);
        for node in neighbors {
            //eprintln!("[h] adding: {:?} -> {:?}", node, v_min);
            h.add_edge(*node, v_min);
        }

    });

    h
}


// with Edge Pruning
pub fn min_selection_ep<G, N>(g: &G) -> ConcurrentDiGraph<N>
where
    G: GraphTrait<N>,
    N: NodeTrait + Eq + Send + Sync + Debug,
{
    let neighborhoods: DashMap<N, HashSet<N>> = g.get_all_neighborhoods();
    let v_mins: DashMap<N, N> = get_vmins(g);

    // create directed graph h
    let h: ConcurrentDiGraph<N> = ConcurrentDiGraph::with_capacity(g.node_count(), g.edge_count());

    //add edges
    //let mut neighborhoods_entries: Vec<_> = neighborhoods.iter().collect();
    
    //why sort? 
    //neighborhoods_entries.sort_by(|a, b| a.key().cmp(b.key()));

    //can be par_iterated
    neighborhoods
    .par_iter()
    .for_each(|entry|{

        let &&n = &entry.key();
        let &neighbors = &entry.value();

        let n_min_opt = v_mins.get(&n);
        /*if n_min_opt.is_none() {
            continue;
        }*/
        let n_min = *n_min_opt.unwrap();

        //when a node is the minimum of its neighbourhood, it does not need to notify this information to its neighbours
        if n == n_min {
            for z in neighbors {
                let z_min = *v_mins.get(z).unwrap();    //can safely unwrap because all keys (nodes) are preseved (present) in v_mins

                //when a node u is the local minimum in NN(u), [u = u_min] there are two exclusive cases
                if z_min == n {
                    h.add_edge(*z, n);
                    //eprintln!("[caso A] adding edge {:?}->{:?}", *z, n);
                } else {
                    h.add_edge(*z, z_min);
                    //eprintln!("[caso B] adding edge {:?}->{:?}", *z, z_min);

                    h.add_edge(n, z_min);
                    //eprintln!("[caso B] adding edge {:?}->{:?}", n, z_min);
                }
                //eprintln!("removing {:?}", &z);
            }
        } else {
            h.add_edge(n, n_min); // => get_neighborhood return <neighbors + node>
            //eprintln!("[caso C] adding edge {:?}->{:?}", n, n_min);
            for node in neighbors {
                //eprintln!("adding: {:?} -> {:?}", node, v_min);
                h.add_edge(*node, n_min);
                //eprintln!("[caso C] adding edge {:?}->{:?}", *node, n_min);
            }
        }
    });
    h
}



pub fn prune<N: NodeTrait + Send + Sync + Debug>(
    h: ConcurrentDiGraph<N>,
    tree: ConcurrentDiGraph<N>,
) -> (ConcurrentUnGraph<N>, ConcurrentDiGraph<N>) {
    //eprintln!("Pruning");
    //get outgoing neighborhoods
    let outgoing_neighborhoods: DashMap<N, HashSet<N>> = h.get_neighborhoods(true);

    let min_outgoing_neighborhoods = get_vmins(&h);

    let pruned_graph = ConcurrentUnGraph::with_capacity(h.node_count(), h.edge_count());

    //add to G(t+1) + deactivation
    let deactivated_nodes: DashSet<N> = DashSet::new();

    outgoing_neighborhoods.par_iter().for_each(|entry| {
        let (u, neighbors) = entry.pair();

        if neighbors.len() > 1 {
            let v_min = *min_outgoing_neighborhoods.get(u).unwrap();

            for v in neighbors {
                if *v != v_min {
                    pruned_graph.add_edge(*v, v_min);
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
            tree.add_edge(v_min, *u);
            //eprintln!("Adding to tree: {:?} -> {:?}", v_min, *u);

            deactivated_nodes.insert(*u);
        }

        //TODO: 3rd case (node is seed: still active but NN(u) = {u})
        /*if (neighbors.len() == 1) && neighbors.contains(u) {
            deactivated_nodes_mutex.lock().unwrap()
                .push(*u);
        }*/
    });

    //let deactivated_nodes: Vec<N> = deactivated_nodes_mutex.into_inner().unwrap_or_default();
    //deactivated_nodes.sort_unstable_by(|a, b| b.cmp(a));    //sort + reverse


    for deactivated in deactivated_nodes {
        //eprintln!("Removing node: {:?}", deactivated);
        pruned_graph.remove_node(deactivated);
    }

    (pruned_graph, tree)
}


pub fn prune_os<N: NodeTrait + Debug>(
    h: ConcurrentDiGraph<N>,
    tree: ConcurrentDiGraph<N>,
) -> (ConcurrentDiGraph<N>, ConcurrentDiGraph<N>) {
    //get outgoing neighborhoods
    let outgoing_neighborhoods: DashMap<N, HashSet<N>> = h.get_neighborhoods(true);

    let min_outgoing_neighborhoods = get_vmins(&h);

    let pruned_graph = ConcurrentDiGraph::<N>::with_capacity(h.node_count(), h.edge_count());

    //add to G(t+1) + deactivation
    let deactivated_nodes: DashSet<N> = DashSet::new();

    //let tree_mutex = Mutex::new(tree);

    outgoing_neighborhoods.par_iter().for_each(|entry| {
        let (u, neighbors) = entry.pair();

        if neighbors.len() > 1 {
            let v_min = *min_outgoing_neighborhoods.get(u).unwrap();

            for v in neighbors {
                if *v != v_min {
                    pruned_graph.add_edge(*v, v_min);
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
            tree.add_edge(v_min, *u);

            deactivated_nodes.insert(*u);   //TODO: remove here instead of collecting?
        }

        //TODO: 3rd case (node is seed: still active but NN(u) = {u})
        /*if (neighbors.len() == 1) && neighbors.contains(u) {
            deactivated_nodes.insert(*u);
        }*/
    });

    //eprintln!("pruned_graph: {:?}", pruned_graph);

    //removing deactivated nodes
    deactivated_nodes
        .par_iter()
        .for_each(|n| {
            pruned_graph.remove_node(*n)
    });

    (pruned_graph, tree)
}



pub fn seed_propagation<V: NodeTrait + Debug>(tree: &ConcurrentDiGraph<V>) -> HashMap<V, V> {
    let start = std::time::Instant::now();

    let mut seeds_map: HashMap<V, V> = HashMap::with_capacity(tree.node_count());

    let mut nodes: Vec<V> = tree.nodes();
    nodes.sort_unstable(); //no duplicates => can use unstable sorting (more efficient)
    eprintln!("after sorting: {:?}", start.elapsed());

    assert_eq!(nodes.len(), tree.node_count());

    //while + removal
    while !nodes.is_empty() {
        
        let min_node = nodes[0]; //sorted nodes => min node will always be the 1st
        //println!("seeding {:?}", min_node);
        //let iteration_start = std::time::Instant::now();
        let incoming_edge = tree.incoming_edges(min_node); //either 0 or 1 edge
        //eprintln!("incoming duration: {:?}", iteration_start.elapsed());
        //eprintln!("{:?}", incoming_edge);


        //TODO: HashMap -> DashMap and par_iter
        //At this point every node should have only 0 or 1 incoming edge?
        //assert!((incoming_edge.len() == 0) ^ (incoming_edge.len() == 1));

        for from in incoming_edge {
            //eprintln!("Node {:?}, edge {:?}", min_node, edge);

            if seeds_map.contains_key(&from) {
                let parent_seed = seeds_map.get(&from).unwrap();
                seeds_map.insert(min_node, *parent_seed);
            } else {
                seeds_map.insert(min_node, from);
            }
        }

        //no incoming edge into node => node is root of a tree
        seeds_map
            .entry(min_node) // if min_node not in seeds_map
            .or_insert(min_node); // insert

        nodes.remove(0);
    }

    seeds_map
}


pub fn par_seed_propagation<V: NodeTrait + Debug>(tree: &ConcurrentDiGraph<V>) -> DashMap<V, V> {
    let seeds_map = DashMap::with_capacity(tree.node_count());

    tree.nodes().par_iter().for_each(|&n| {
        //println!("seeding {:?}", &n);
        //let incoming_edges = tree.incoming_edges(n);
        //assert!(incoming_edges.is_empty() ^ (incoming_edges.len() == 1));

        /*if incoming_edges.is_empty() {  //node is a root
            seeds_map.insert(n, n);
        }
        else {
            //get root
            seeds_map.insert(n, get_root(&tree, n));
        }*/
        seeds_map.insert(n, get_root(&tree, n));
    });

    seeds_map
}

fn get_root<V: NodeTrait>(tree: &ConcurrentDiGraph<V>, node: V) -> V{
    /*
    - get incoming
    - if incoming.empty -> node
    - else incoming.get -> recursive
    */

    let incoming = tree.incoming_edges(node);
    
    if incoming.is_empty() {
        node
    }
    else{
        get_root(tree, *incoming.iter().next().unwrap())
    }
}