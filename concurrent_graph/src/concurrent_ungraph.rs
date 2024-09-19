#![allow(dead_code)]
use dashmap::DashMap;
use dashmap::DashSet;
use std::collections::HashSet;
use rayon::prelude::*;

use crate::GraphTrait;
use crate::NodeTrait;

#[derive(Clone, Debug)]
pub struct ConcurrentUnGraph<N: NodeTrait> {
    adj_list: DashMap<N, HashSet<N>>,  // Adjacency list without weights
    avg_edges: usize
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
        self.get_closed_neighborhoods()
    }

    fn get_all_neighborhoods(&self) -> DashMap<N, HashSet<N>> {
        self.adj_list.clone()
    }


    fn add_node(&self, node: N){
        match self.adj_list.get(&node) {
            Some(_) => (),
            None => {self.adj_list.insert(node, HashSet::with_capacity(self.avg_edges));}
        }
    }

    fn remove_node(&self, node: N){
        self.adj_list.remove(&node);
    }

    #[inline]
    fn node_count(&self) -> usize {
        self.adj_list.len()
    }

    fn edge_count(&self) -> usize {
        self.adj_list.par_iter()
            .map(|entry| entry.value().len())
            .sum()
    }

    fn outgoing_edges(&self, node: N) -> HashSet<N> {
        match self.adj_list.get(&node) {
            Some(v) => v.clone(),
            None => HashSet::new()
        }
    }

    /// Add an edge between two nodes; parallel edges not allowed, but self-loops are
    #[inline]
    fn add_edge(&self, a: N, b: N) {
        if !self.adj_list.contains_key(&b){
            self.add_node(b);
        }


        // Add (a -> b)
        match self.adj_list.get_mut(&a) {
            Some(mut vec) => {vec.insert(b);},
            None => {
                let mut new_neigh = HashSet::with_capacity(self.avg_edges);
                new_neigh.insert(b);
                self.adj_list.insert(a, new_neigh);
            }
        }
        
        // Since it's an undirected graph, add (b -> a)
        match self.adj_list.get_mut(&b) {
            Some(mut vec) => {vec.insert(a);},
            None => {
                let mut new_neigh = HashSet::with_capacity(self.avg_edges);
                new_neigh.insert(a);
                self.adj_list.insert(b, new_neigh);
            }
        }
        
    }

    /// Check if a node is contained in the graph
    #[inline]
    fn contains_node(&self, node: N) -> bool {
        self.adj_list.contains_key(&node)
    }

    /// Check if an edge exists between two nodes ~ O(1)
    #[inline]
    fn contains_edge(&self, node_a: N, node_b: N) -> bool {
        match self.adj_list.get(&node_a) {
            Some(vec) => vec.contains(&node_b),
            None => false
        }
    }
}


impl<N> Default for ConcurrentUnGraph<N>
where N: Eq + NodeTrait {
    fn default() -> Self {
        Self::new()
    }
}

impl<N> ConcurrentUnGraph<N> 
where N: Eq + NodeTrait {
    /// Create a new graph
    pub fn new() -> Self {
        ConcurrentUnGraph {
            adj_list: DashMap::new(),
            avg_edges: 1
        }
    }

    pub fn with_capacity(num_nodes: usize, num_edges: usize) -> Self {
        let avg_edges = num_edges / num_nodes; 

        ConcurrentUnGraph{
            adj_list: DashMap::with_capacity_and_shard_amount(num_nodes, num_nodes.next_power_of_two()), //DashMap::with_capacity(num_nodes),
            avg_edges
        }
    }

    pub fn from_edges(){}

    fn is_directed(&self) -> bool {
        false
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
