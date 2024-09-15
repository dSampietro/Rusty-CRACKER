#![allow(dead_code)]
use dashmap::DashMap;
use dashmap::DashSet;
use std::collections::HashSet;
use rayon::prelude::*;

use crate::GraphTrait;
use crate::NodeTrait;

#[derive(Clone)]
pub struct ConcurrentUnGraph<N: NodeTrait> {
    adj_list: DashMap<N, HashSet<N>>,  // Adjacency list without weights
    directed: bool
}

impl<N> GraphTrait<N> for ConcurrentUnGraph<N> 
where N: Eq + NodeTrait {
    fn nodes(&self) -> Vec<N> {
        self.adj_list
            .par_iter()
            .map(|entry| *entry.key())
            .collect()        
    }

    fn get_closed_neighborhoods_undirected(&self) -> DashMap<N, HashSet<N>> {
        self.adj_list.clone()
    }

    fn get_all_neighborhoods(&self) -> DashMap<N, HashSet<N>> {
        self.adj_list.clone()
    }


    fn add_node(&self, node: N){
        match self.adj_list.get(&node) {
            Some(_) => (),
            None => {self.adj_list.insert(node, HashSet::new());}
        }
    }

    fn remove_node(&self, node: N){
        self.adj_list.remove(&node);
    }

    fn node_count(&self) -> usize {
        self.adj_list.len()
    }

    fn edge_count(&self) -> usize {
        self.adj_list.par_iter()
            .map(|entry| entry.value().len())
            .sum()
    }

    fn outgoing_edges(&self, node: N) -> HashSet<N> {
        assert!(self.directed);

        match self.adj_list.get(&node) {
            Some(v) => v.clone(),
            None => HashSet::new()
        }
    }

    /// Add an edge between two nodes; parallel edges not allowed, but self-loops are
    fn add_edge(&self, a: N, b: N) {
        if !self.adj_list.contains_key(&b){
            self.add_node(b);
        }


        // Add (a -> b)
        match self.adj_list.get_mut(&a) {
            Some(mut vec) => {vec.insert(b);},
            None => {
                let mut new_neigh = HashSet::new();
                new_neigh.insert(b);
                self.adj_list.insert(a, new_neigh);
            }
        }
        
        // Since it's an undirected graph, add (b -> a)
        if !self.is_directed() {
            match self.adj_list.get_mut(&b) {
                Some(mut vec) => {vec.insert(a);},
                None => {
                    let mut new_neigh = HashSet::new();
                    new_neigh.insert(a);
                    self.adj_list.insert(b, new_neigh);
                }

            }
        }
    }

    /// Check if a node is contained in the graph
    fn contains_node(&self, node: N) -> bool {
        self.adj_list.contains_key(&node)
    }

    /// Check if an edge exists between two nodes ~ O(1)
    fn contains_edge(&self, node_a: N, node_b: N) -> bool {
        match self.adj_list.get(&node_a) {
            Some(vec) => vec.contains(&node_b),
            None => false
        }
    }
}


impl<N> ConcurrentUnGraph<N> 
where N: Eq + NodeTrait {
    /// Create a new graph
    pub fn new() -> Self {
        ConcurrentUnGraph {
            adj_list: DashMap::new(),
            directed: false
        }
    }

    pub fn from_edges(){}

    fn is_directed(&self) -> bool {
        self.directed
    }

    /// Get neighbors of a node
    pub fn neighbors(&self, node: N) -> HashSet<N> {
        match self.adj_list.get(&node) {
            Some(vec) => vec.clone(),
            None => HashSet::new()
        }
    }

    ///Get the closed neighbourhood (neighborhood + node) of every node
    pub fn get_closed_neighborhoods(&self) -> DashMap<N, HashSet<N>> {
        let res = self.adj_list.clone();

        res
            .par_iter_mut()
            .for_each( |mut entry| {
                let key = *entry.key();
                entry.value_mut().insert(key);
            });

        res
    }

    // ~ O(|V|)
    pub fn incoming_edges(&self, node: N) -> DashSet<N> {
        assert!(self.directed); //unnecessary

        // {n € N | node € adj_list[n]}

        let res: DashSet<N> = DashSet::new();

        self.adj_list.par_iter()
            .for_each(|entry| {
                let (key, v) = entry.pair();

                if v.contains(&node) {
                    res.insert(*key);
                }
        });

        res
    }

}
