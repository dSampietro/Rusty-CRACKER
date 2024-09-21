#![allow(dead_code)]
use dashmap::DashMap;
use std::collections::HashSet;
use rayon::prelude::*;

use crate::NodeTrait;


#[derive(Clone)]
pub struct ConcurrentDiGraph<N: NodeTrait> {
    incoming: DashMap<N, HashSet<N>>,  // Adjacency list without weights
    outgoing: DashMap<N, HashSet<N>>,
}

impl<N> ConcurrentDiGraph<N> 
where N: Eq + NodeTrait {
    // Create a new graph
    pub fn new() -> Self {
        ConcurrentDiGraph {
            incoming: DashMap::new(),
            outgoing: DashMap::new()
        }
    }

    pub fn nodes(&self) -> Vec<N> {
        self.outgoing
            .par_iter()
            .map(|entry| entry.key().clone())
            .collect()        
    }

    pub fn get_incoming_neighborhoods(&self) -> DashMap<N, HashSet<N>> {
        self.incoming.clone()
    }

    pub fn get_neighborhoods(&self) -> DashMap<N, HashSet<N>> {
        self.outgoing.clone()
    }

    ///Get the closed neighbourhood (neighborhood + node) of every node
    pub fn get_closed_neighborhoods(&self) -> DashMap<N, HashSet<N>> {
        let res = self.outgoing.clone();

        res
            .par_iter_mut()
            .for_each( |mut entry| {
                let key = entry.key().clone();
                entry.value_mut().insert(key);
            });

        res
    }

    pub fn add_node(&self, node: N){
        match self.incoming.get(&node) {
            Some(_) => (),
            None => {self.incoming.insert(node, HashSet::new());}
        }

        match self.outgoing.get(&node) {
            Some(_) => (),
            None => {self.outgoing.insert(node, HashSet::new());}
        }
    }

    pub fn remove_node(&self, node: N){
        self.incoming.remove(&node);
        self.outgoing.remove(&node);

        /*
        //filtering edges is too expensive ~O(|V|)
        self.adj_list.iter_mut().for_each(|mut entry| {
            (*entry).remove(&node);
        });
        */
    }

    pub fn node_count(&self) -> usize {
        self.outgoing.len()
    }

    pub fn edge_count(&self) -> usize {
        self.outgoing.par_iter()
            .map(|entry| entry.value().len())
            .sum()
    }

    pub fn outgoing_edges(&self, node: N) -> HashSet<N> {
        match self.outgoing.get(&node) {
            Some(v) => v.clone(),
            None => HashSet::new()
        }
    }

    pub fn incoming_edges(&self, node: N) -> HashSet<N>{
        match self.incoming.get(&node) {
            Some(v) => v.clone(),
            None => HashSet::new()
        }
    }

    fn is_directed(&self) -> bool {
        true
    }

    // Add an edge between two nodes; parallel edges not allowed, but self-loops are
    pub fn add_edge(&self, a: N, b: N) {
        // Add (a -> b)
        match self.outgoing.get_mut(&a) {
            Some(mut vec) => {vec.insert(b);},
            None => {
                let mut new_neigh = HashSet::new();
                new_neigh.insert(b);
                self.outgoing.insert(a, new_neigh);
            }
        }
        
        // Since it's an undirected graph, add (b -> a)
        match self.incoming.get_mut(&b) {
            Some(mut vec) => {vec.insert(a);},
            None => {
                let mut new_neigh = HashSet::new();
                new_neigh.insert(a);
                self.incoming.insert(b, new_neigh);
            }
        }
    }

    /// Get (outgoing) neighbors of a node
    pub fn neighbors(&self, node: N) -> HashSet<N> {
        match self.outgoing.get(&node) {
            Some(vec) => vec.clone(),
            None => HashSet::new()
        }
    }

    /// Check if a node is contained in the graph
    pub fn contains_node(&self, node: N) -> bool {
        self.outgoing.contains_key(&node) || self.incoming.contains_key(&node)
    }

    /// Check if an edge exists between two nodes
    pub fn contains_edge(&self, node_a: N, node_b: N) -> bool {
        match self.outgoing.get(&node_a) {
            Some(vec) => vec.contains(&node_b),
            None => false
        }
    }
}