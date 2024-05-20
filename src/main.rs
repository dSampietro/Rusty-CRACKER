use std::{cmp::min, collections::HashMap};
use petgraph::{ graph::{DiGraph, NodeIndex, UnGraph}, Direction::Outgoing, EdgeType, Graph};

fn get_neighborhood<V, E, Type>(g: &Graph<V, E, Type>) -> HashMap<NodeIndex, Vec<NodeIndex>> where Type: EdgeType {
    let mut neighborhoods: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();

    let nodes = g.node_indices();
    for n in nodes.into_iter(){
        let nn = g.neighbors(n);
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

//TODO: generalize edges
fn min_selection<N: Copy>(g: &UnGraph<N, ()>) -> DiGraph<N, ()> {
    let neighborhoods: HashMap<NodeIndex, Vec<NodeIndex>> = get_neighborhood(&g);
    let v_mins: HashMap<NodeIndex, NodeIndex> = get_vmins(&neighborhoods);

    // create directed graph h
    let mut h: DiGraph<N, ()> = DiGraph::new();
    
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


fn get_outgoing_neighborhood<N>(h: &DiGraph<N, ()>) -> HashMap<NodeIndex, Vec<NodeIndex>>{
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


fn prune<N: std::fmt::Debug + Copy>(h: DiGraph<N, ()>, mut T: DiGraph<N, ()>) -> (UnGraph<N, ()>, DiGraph<N, ()>) {
    
    //get outgoing neighborhoods
    let outgoing_neighborhoods: HashMap<NodeIndex, Vec<NodeIndex>> = get_outgoing_neighborhood(&h);
    let min_outgoing_neighborhoods = get_vmins(&outgoing_neighborhoods);

    let mut g2 = UnGraph::<N, ()>::new_undirected();
    for n in h.node_indices(){  //prima del pruning: g_(i+1) ha gli stessi nodi di h(i)
        g2.add_node(*h.node_weight(n).unwrap());
    }

    //add to G(t+1) + deactivation
    let mut deactivated_nodes: Vec<NodeIndex> = Vec::<NodeIndex>::new(); 

    for (u, neighbors) in &outgoing_neighborhoods {
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
            T.add_edge(v_min, *u, ());
            deactivated_nodes.push(*u);
        }
    }
    
    deactivated_nodes.sort();
    deactivated_nodes.reverse();

    println!("g2: {:?}", g2);

    for deactivated in deactivated_nodes{
        g2.remove_node(deactivated);
    }

    return (g2, T);
}


fn main() {
    let edges = [(0, 1), (1, 2), (2, 4), (2, 5), (3, 4), (3, 6), (3, 7), (5, 8), (7, 8)];
    let g = UnGraph::<(), ()>::from_edges(&edges);

    let mut T = DiGraph::<(), ()>::new();
    for _ in g.node_indices(){
        T.add_node(());
    }

    //min selection
    let h = min_selection(&g);
    //println!("{:?}", h);


    //pruning
    let (g2, t) = prune(h, T);
    println!("pruned g2: {:?}", g2);
    println!("T: {:?}", t);
}
