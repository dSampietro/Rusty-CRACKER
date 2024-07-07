use std::{collections::HashMap, sync::{Arc, Mutex}, thread};

use petgraph::graphmap::UnGraphMap;

fn main(){
    type V = u8;
    let n_thread = 8;
    let edges = [(0, 1), (1, 2), (2, 4), (2, 5), (3, 4), (3, 6), (3, 7), (5, 8), (7, 8)];
    let graph: UnGraphMap<V, ()> = UnGraphMap::from_edges(&edges);

    let nodes: Vec<V> = graph.nodes().collect();
    let chunk_size = nodes.len() / n_thread;
    
    
    let graph_arc = Arc::new(graph);
    let neighbors_arc: Arc<Mutex<HashMap::<V, Vec<V>>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles: Vec<_> = Vec::with_capacity(n_thread);


    for i in 0..n_thread{
        let shared_map = Arc::clone(&neighbors_arc);
        let shared_graph = Arc::clone(&graph_arc);

        let chunk = nodes[i * chunk_size..((i + 1) * chunk_size)].to_vec();

        let handle = thread::spawn( move || {
            let mut local_neigh = Vec::<V>::new();

            for n in chunk {
                for e in shared_graph.neighbors(n){
                    //println!("{n} -> {e}");
                    local_neigh.push(e)
                };
                shared_map.lock().unwrap().insert(n, local_neigh.clone());
            }

        });

        handles.push(handle);
    }


    for handle in handles{
        handle.join().unwrap();
    }

    let neighbors = neighbors_arc.lock().unwrap();
    println!("{:?}", neighbors);
    

    //single thread
    /*let mut neighbors: HashMap::<V, Vec<V>> = HashMap::new();

    for n in graph.nodes(){
        let mut neighs = Vec::<V>::new();
        
        for e in graph.neighbors(n){
            neighs.push(e)
        }
        neighbors.insert(n, neighs);
    }
    println!("{:?}", neighbors);*/

    

}