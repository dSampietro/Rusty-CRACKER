mod concurrent_graph;
use concurrent_graph::{ConcurrentGraph, ConcurrentUnGraph};
use rand::Rng;
use rayon::{iter, ThreadPoolBuilder};

fn main() {
    let num_threads = 4;
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();


    println!("w/ {num_threads} threads");
    let graph: ConcurrentUnGraph<i32> = ConcurrentUnGraph::new_undirected();

    // Add edges
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(4, 5);

    // Check neighbors
    /*if let Some(neighbors) = graph.neighbors(2) {
        println!("Node 2's neighbors: {:?}", neighbors);
    }*/

    assert_eq!(graph.contains_node(1), true);
    assert_eq!(graph.contains_node(4), true);
    assert_eq!(graph.contains_node(10), false);


    /*let mut edges: Vec<(u8, u8)> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..1_000_000{
        let a = rng.gen();
        let b = rng.gen();

        edges.push((a,b));
    }

    //println!("{edges:?}");

    let now = std::time::Instant::now();

    edges.par_iter().for_each(|(a, b)| {
        graph.add_edge(*a, *b);
    });*/


    println!("{:?}", graph.neighbors(1));
    println!("{:?}", graph.get_neighborhoods());
    //println!("Duration: {:?}", now.elapsed());


}