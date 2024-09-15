use dashmap::DashMap;
use std::{collections::HashSet, hash::Hash};
use rayon::prelude::*;


pub trait NodeTrait: Copy + Ord + Hash + Send + Sync {}
impl<N> NodeTrait for N where N: Copy + Ord + Hash + Send + Sync {}

/// Adjacency list without weights
#[derive(Clone)]
pub struct ConcurrentDiGraph<N: NodeTrait> {
    outgoing_edges: DashMap<N, HashSet<N>>,
    incoming_edges: DashMap<N, HashSet<N>>,
    directed: bool
}

impl<N> ConcurrentDiGraph<N> 
where N: Eq + NodeTrait {
    // Create a new graph
    pub fn new() -> Self {
        ConcurrentDiGraph {
            outgoing_edges: DashMap::new(),
            incoming_edges: DashMap::new(),
            directed: true
        }
    }

    pub fn from_edges(){}

    pub fn nodes(&self) -> Vec<N> {
        self.outgoing_edges
            .par_iter()
            .map(|entry| *entry.key())
            .collect()        
    }

    pub fn get_neighborhoods(&self, outgoing: bool) -> DashMap<N, HashSet<N>> {
        if outgoing {
            self.outgoing_edges.clone()
        }
        else {
            self.incoming_edges.clone()
        }
    }

    ///Get the closed neighbourhood (neighborhood + node) of every node
    pub fn get_closed_neighborhoods(&self, outgoing: bool) -> DashMap<N, HashSet<N>> {

        let res = if outgoing {self.outgoing_edges.clone()} else {self.incoming_edges.clone()};

        res.par_iter_mut()
            .for_each( |mut entry| {
                let key = *entry.key();
                entry.value_mut().insert(key);
            });

        res
    }

    pub fn add_node(&self, node: N){
        match self.outgoing_edges.get(&node) {
            Some(_) => (),
            None => {self.outgoing_edges.insert(node, HashSet::new());}
        }

        match self.incoming_edges.get(&node) {
            Some(_) => (),
            None => {self.incoming_edges.insert(node, HashSet::new());}
        }
    }

    pub fn remove_node(&self, node: N){
        self.outgoing_edges.remove(&node);
        self.incoming_edges.remove(&node);
    }

    pub fn node_count(&self) -> usize {
        self.outgoing_edges.len()
    }

    pub fn edge_count(&self) -> usize {
        self.outgoing_edges.par_iter()
            .map(|entry| entry.value().len())
            .sum()
    }

    pub fn outgoing_edges(&self, node: N) -> HashSet<N> {
        match self.outgoing_edges.get(&node) {
            Some(v) => v.clone(),
            None => HashSet::new()
        }
    }

    pub fn incoming_edges(&self, node: N) -> HashSet<N> {
        match self.incoming_edges.get(&node) {
            Some(v) => v.clone(),
            None => HashSet::new()
        }
    }

    fn is_directed(&self) -> bool {
        self.directed
    }

    /// Add an edge between two nodes; parallel edges not allowed, but self-loops are
    pub fn add_edge(&self, a: N, b: N) {
        //maybe use a nodes hashset to keep track of nodes
        if !self.outgoing_edges.contains_key(&b){
            self.add_node(b);
        }

        if !self.incoming_edges.contains_key(&a){
            self.add_node(a);
        }


        // Add (a -> b) to outgoing_edges
        match self.outgoing_edges.get_mut(&a) {
            Some(mut vec) => {vec.insert(b);},
            None => {
                let mut new_neigh = HashSet::new();
                new_neigh.insert(b);
                self.outgoing_edges.insert(a, new_neigh);
            }
        }
        
        // Add (b <- a) to incoming_edges
        match self.incoming_edges.get_mut(&b) {
            Some(mut vec) => {vec.insert(a);},
            None => {
                let mut new_neigh = HashSet::new();
                new_neigh.insert(a);
                self.incoming_edges.insert(b, new_neigh);
            }
        }
    
    }

    /// Get neighbors of a node
    fn neighbors(&self, node: N, outgoing: bool) -> HashSet<N> {
        let neigh = if outgoing {self.outgoing_edges.get(&node)} else {self.incoming_edges.get(&node)};

        match neigh {
            Some(vec) => vec.clone(),
            None => HashSet::new()
        }
    }

    pub fn neighbors_incoming(&self, node: N) -> HashSet<N> {
        self.neighbors(node, false)
    }

    pub fn neighbors_outgoing(&self, node: N) -> HashSet<N> {
        self.neighbors(node, true)
    }

    /// Check if a node is contained in the graph
    pub fn contains_node(&self, node: N) -> bool {
        self.outgoing_edges.contains_key(&node)
    }

    /// Check if an edge exists between two nodes ~ O(1)
    pub fn contains_edge(&self, node_a: N, node_b: N) -> bool {
        match self.outgoing_edges.get(&node_a) {
            Some(vec) => vec.contains(&node_b),
            None => false
        }
    }
}
