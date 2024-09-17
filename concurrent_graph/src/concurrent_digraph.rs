use dashmap::DashMap;
use std::collections::HashSet;
use rayon::prelude::*;

use crate::{GraphTrait, NodeTrait};

/// Adjacency list without weights
#[derive(Clone, Debug)]
pub struct ConcurrentDiGraph<N: NodeTrait> {
    outgoing_edges: DashMap<N, HashSet<N>>,
    incoming_edges: DashMap<N, HashSet<N>>,
    avg_edges: usize
}

impl<N> GraphTrait<N> for ConcurrentDiGraph<N> 
where N: Eq + NodeTrait {
    fn nodes(&self) -> Vec<N> {
        self.outgoing_edges
            .par_iter()
            .map(|entry| *entry.key())
            .collect()        
    }


    fn get_closed_neighborhoods_undirected(&self) -> DashMap<N, HashSet<N>> {
        // join incoming and outgoing edges
        let res: DashMap<N, HashSet<N>> = DashMap::new();

        self.nodes().par_iter().for_each(|&n| {
            let mut neighs = self.incoming_edges(n);
            neighs.extend(self.outgoing_edges(n));
            neighs.insert(n);
            
            res.insert(n, neighs);
        });

        res
    }

    fn get_all_neighborhoods(&self) -> DashMap<N, HashSet<N>> {
        // join incoming and outgoing edges
        let res: DashMap<N, HashSet<N>> = DashMap::new();

        self.nodes().par_iter().for_each(|&n| {
            let mut neighs = self.incoming_edges(n);
            neighs.extend(self.outgoing_edges(n));
            
            res.insert(n, neighs);
        });

        res
    }


    fn add_node(&self, node: N){
        match self.outgoing_edges.get(&node) {
            Some(_) => (),
            None => {self.outgoing_edges.insert(node, HashSet::with_capacity(self.avg_edges));}
        }

        match self.incoming_edges.get(&node) {
            Some(_) => (),
            None => {self.incoming_edges.insert(node, HashSet::with_capacity(self.avg_edges));}
        }
    }

    fn remove_node(&self, node: N){
        self.outgoing_edges.remove(&node);
        self.incoming_edges.remove(&node);
    }

    fn node_count(&self) -> usize {
        self.outgoing_edges.len()
    }

    fn edge_count(&self) -> usize {
        self.outgoing_edges.par_iter()
            .map(|entry| entry.value().len())
            .sum()
    }

    fn outgoing_edges(&self, node: N) -> HashSet<N> {
        match self.outgoing_edges.get(&node) {
            Some(v) => v.clone(),
            None => HashSet::new()
        }
    }


    /// Add an edge between two nodes; parallel edges not allowed, but self-loops are
    fn add_edge(&self, a: N, b: N) {
        //maybe use a nodes hashset to keep track of nodes
        
        //is this even useful?
        /*if !self.outgoing_edges.contains_key(&b){
            self.add_node(b);
        }

        if !self.incoming_edges.contains_key(&a){
            self.add_node(a);
        }*/


        // Add (a -> b) to outgoing_edges
        match self.outgoing_edges.get_mut(&a) {
            Some(mut vec) => {vec.insert(b);},
            None => {
                let mut new_neigh = HashSet::with_capacity(self.avg_edges);
                new_neigh.insert(b);
                self.outgoing_edges.insert(a, new_neigh);
            }
        }
        
        // Add (b <- a) to incoming_edges
        match self.incoming_edges.get_mut(&b) {
            Some(mut vec) => {vec.insert(a);},
            None => {
                let mut new_neigh = HashSet::with_capacity(self.avg_edges);
                new_neigh.insert(a);
                self.incoming_edges.insert(b, new_neigh);
            }
        }
    
    }

    /// Check if a node is contained in the graph
    fn contains_node(&self, node: N) -> bool {
        self.outgoing_edges.contains_key(&node)
    }

    /// Check if an edge exists between two nodes ~ O(1)
    fn contains_edge(&self, node_a: N, node_b: N) -> bool {
        match self.outgoing_edges.get(&node_a) {
            Some(vec) => vec.contains(&node_b),
            None => false
        }
    }
}


impl<N> ConcurrentDiGraph<N> 
where N: Eq + NodeTrait {
    /// Create a new graph
    pub fn new() -> Self {
        ConcurrentDiGraph {
            outgoing_edges: DashMap::new(),
            incoming_edges: DashMap::new(),
            avg_edges: 1
        }
    }

    pub fn with_capacity(num_nodes: usize, num_edges: usize) -> Self {
        let avg_edges = num_edges / num_nodes; 

        ConcurrentDiGraph {
            outgoing_edges: DashMap::with_capacity(num_nodes),
            incoming_edges: DashMap::with_capacity(num_nodes),
            avg_edges: avg_edges
        }
    }

    fn from_edges(){}

    fn is_directed(&self) -> bool {
        true
    }

    pub fn get_neighborhoods(&self, outgoing: bool) -> DashMap<N, HashSet<N>> {
        if outgoing {
            self.outgoing_edges.clone()
        }
        else {
            self.incoming_edges.clone()
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

    pub fn incoming_edges(&self, node: N) -> HashSet<N> {
        match self.incoming_edges.get(&node) {
            Some(v) => v.clone(),
            None => HashSet::new()
        }
    }
}