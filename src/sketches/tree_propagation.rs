use std::{collections::HashMap, fmt::Debug};

use petgraph::{graphmap::{DiGraphMap, NodeTrait}, Direction::Incoming};

fn propagate<V: NodeTrait + Debug>(tree: DiGraphMap<V, ()>) -> HashMap<V, V>{
    let mut res: HashMap<V, V> = HashMap::new();

    let mut nodes: Vec<V> = tree.nodes().collect();
    assert_eq!(nodes.len(), tree.node_count());
    println!("Nodes: {:?}", nodes);
    nodes.sort_unstable();

    while nodes.len() != 0 {
        //let min_node = *nodes.iter().min().unwrap();    //can safely unwrap; if nodes is empty => break from while
    
        let min_node = nodes[0];        //sorting nodes => min node will always be the 1st
        let incoming_edge = tree.edges_directed(min_node, Incoming);    //either 0 or 1 edge
        //println!("{:?}", incoming_edge);

        for edge in incoming_edge{
            println!("Node {:?}, edge {:?}", min_node, edge);

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


fn main(){
    type V = u16;

    let test_tree = DiGraphMap::<V, ()>::from_edges(&[(0, 1), (0, 2), (0, 3), (0, 5), (1, 4), (2, 8), (3, 6), (3, 7), (9, 10)]);
    //println!("Test tree: {:?}", test_tree);



    let edges_into = test_tree.edges_directed(10, Incoming);
    
    /*for edge in edges_into {
        println!("{:?}", edge);
        println!("{:?}", edge.1);
    }*/

    let res = propagate(test_tree);
    println!("{:?}", res);




    let mut test_seeds = HashMap::<V, V>::new();
    test_seeds.insert(0, 0);
    test_seeds.insert(1, 0);
    test_seeds.insert(2, 0);
    test_seeds.insert(3, 0);
    test_seeds.insert(4, 0);
    test_seeds.insert(5, 0);
    test_seeds.insert(6, 0);
    test_seeds.insert(7, 0);
    test_seeds.insert(8, 0);
    test_seeds.insert(9, 9);
    test_seeds.insert(10, 9);

    assert_eq!(res.len(), test_seeds.len());
    assert_eq!(res, test_seeds);




}