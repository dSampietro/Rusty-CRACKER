#[allow(dead_code)]
use dashmap::DashMap;
use std::{hash::Hash, sync::Arc};

//type Node = i32;  // Define the node type

pub trait NodeTrait: Copy + Ord + Hash  {}
impl<N> NodeTrait for N where N: Copy + Ord + Hash {}


/*
/// Marker type for a directed graph.
#[derive(Clone, Copy, Debug)]
pub enum Directed {}

/// Marker type for an undirected graph.
#[derive(Clone, Copy, Debug)]
pub enum Undirected {}

pub trait Direction {
    fn is_directed() -> bool;
}

impl Direction for Directed {
    #[inline]
    fn is_directed() -> bool {
        true
    }
}

impl Direction for Undirected {
    #[inline]
    fn is_directed() -> bool {
        false
    }
}*/
/*
pub trait DirectionType {}

struct Undirected;
impl DirectionType for Undirected {}
struct Directed;
impl DirectionType for Directed {}

struct Direction<D: DirectionType> {
    dir: D
}

impl<D> Direction<D> where D: DirectionType {
    fn new(d: D) -> Self{
        Direction {dir: d}
    }
}*/

pub struct ConcurrentGraph<N> {
    adj_list: Arc<DashMap<N, DashMap<N, ()>>>,  // Adjacency list without weights
    directed: bool
}

impl<N> ConcurrentGraph<N> 
where N: Eq + NodeTrait {
    // Create a new graph
    pub fn new(is_directed: bool) -> Self {
        ConcurrentGraph {
            adj_list: Arc::new(DashMap::new()),
            directed: is_directed
        }
    }

    pub fn get_neighborhoods(&self) -> Arc<DashMap<N, DashMap<N, ()>>> {
        self.adj_list.clone()
    }

    pub fn node_count(&self) -> usize {
        self.adj_list.len()
    }

    pub fn edge_count(&self) -> usize {
        0
    }

    fn is_directed(&self) -> bool {
        self.directed
    }

    // Add an edge between two nodes; parallel edges not allowed, but self-loops are
    pub fn add_edge(&self, node_a: N, node_b: N) {
        // Add (a -> b)
        self.adj_list
            .entry(node_a)
            .or_insert_with(DashMap::new)
            .insert(node_b, ());
        
        // Since it's an undirected graph, add (b -> a)
        self.adj_list
            .entry(node_b)
            .or_insert_with(DashMap::new)
            .insert(node_a, ());
    }

    // Get neighbors of a node
    pub fn neighbors(&self, node: N) -> Vec<N> {
        let r = self.adj_list
            .get(&node)
            .map(|neighbors| {
                neighbors.iter().map(|kv| *kv.key()).collect()
        });

        r.unwrap_or_default()
    }

    pub fn contains_node(&self, node: N) -> bool {
        self.adj_list.contains_key(&node)
    }

    /// Check if an edge exists between two nodes
    pub fn contains_edge(&self, node_a: N, node_b: N) -> bool {
        if let Some(neighbors) = self.adj_list.get(&node_a) {
            neighbors.contains_key(&node_b)
        } 
        else {
            false
        }
    }
}




pub type ConcurrentUnGraph<N> = ConcurrentGraph<N>;
impl<N: NodeTrait> ConcurrentUnGraph<N> {
    pub fn new_undirected() -> Self {
        ConcurrentGraph::new(false)
    }
}
pub type ConcurrentDiGraph<N> = ConcurrentGraph<N>;
impl<N: NodeTrait> ConcurrentDiGraph<N> {
    pub fn new_directed() -> Self {
        ConcurrentGraph::new(true)
    }
}