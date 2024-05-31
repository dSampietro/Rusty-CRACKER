/*use petgraph::{graph::NodeIndex, stable_graph::{StableDiGraph, StableUnGraph}, Graph};

fn main(){
    /*let mut deps = Graph::<&str, &str>::new();
    let pg = deps.add_node("petgraph");
    let fb = deps.add_node("fixedbitset");
    let qc = deps.add_node("quickcheck");
    let rand = deps.add_node("rand");
    let libc = deps.add_node("libc");
    
    deps.extend_with_edges(&[
        (pg, fb), (pg, qc),
        (qc, rand), (rand, libc), (qc, libc),
        ]);

    println!("{:?}", deps);
    
    let neigh: Vec<NodeIndex> = deps.neighbors(NodeIndex::new(0)).collect();

    println!("{:?}", neigh);*/

    let edges = [(0, 1), (1, 2), (2, 4), (2, 5), (3, 4), (3, 6), (3, 7), (5, 8), (7, 8)];
    let graph: StableUnGraph<(), ()> = StableUnGraph::<(), ()>::from_edges(&edges);

    println!("{:?}", graph);
    for n in graph.node_indices(){
        println!("{:?}", n);
    }

    let mut g1 = graph.clone();
    g1.remove_node(NodeIndex::from(4));

    println!("{:?}", g1);
    for n in g1.node_indices(){
        println!("{:?}", n);
    }

    let mut h: StableDiGraph<(), ()> = StableDiGraph::new();
    
    for _ in g1.node_indices(){
        h.add_node(()); //iterating on g nodes implies they exist => safe unwrap
    }

    println!("{:?}", h);
    for n in h.node_indices(){
        println!("{:?}", n);
    }
}*/


use petgraph::graph::{Graph, NodeIndex};

fn main() {
    // Example graph `g` with some missing NodeIndex
    let mut g = Graph::<&str, ()>::new();
    let _n0 = g.add_node("node0");
    let n1 = g.add_node("node1");
    let _n2 = g.add_node("node2");
    g.remove_node(n1); // Remove node1 to create a gap

    // Create a new graph `h` with the same NodeIndex as `g`
    let mut h = Graph::<&str, ()>::new();
    
    // Vector to hold the NodeIndex mapping from g to h
    let mut node_mapping = vec![None; g.capacity().0];

    // Create nodes in h with same indices as g
    for node_index in g.node_indices() {
        let new_node_index = h.add_node(g[node_index]);
        node_mapping[node_index.index()] = Some(new_node_index);
    }

    // Verification: print nodes in h
    for node_index in h.node_indices() {
        println!("Node {:?}: {:?}", node_index.index(), h[node_index]);
    }

    // Print node mappings to verify the indices
    for (old_index, new_index) in node_mapping.iter().enumerate() {
        if let Some(new_node) = new_index {
            println!("g index {:?} -> h index {:?}", old_index, new_node.index());
        } else {
            println!("g index {:?} -> None", old_index);
        }
    }

    println!("g {:?}", g);

    h.add_edge(NodeIndex::from(0), NodeIndex::from(1), ());
    println!("h {:?}", h);
}