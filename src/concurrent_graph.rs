#![allow(dead_code)]
use dashmap::DashMap;
use dashmap::DashSet;
use std::{collections::HashSet, hash::Hash};
use rayon::prelude::*;

pub trait NodeTrait: Copy + Ord + Hash + Send + Sync {}
impl<N> NodeTrait for N where N: Copy + Ord + Hash + Send + Sync {}

#[derive(Clone)]
pub struct ConcurrentGraph<N: NodeTrait> {
    adj_list: DashMap<N, HashSet<N>>,  // Adjacency list without weights
    directed: bool
}

impl<N> ConcurrentGraph<N> 
where N: Eq + NodeTrait {
    // Create a new graph
    pub fn new(is_directed: bool) -> Self {
        ConcurrentGraph {
            adj_list: DashMap::new(),
            directed: is_directed
        }
    }

    pub fn from_edges(){
        
    }

    pub fn nodes(&self) -> Vec<N> {
        self.adj_list
            .par_iter()
            .map(|entry| *entry.key())
            .collect()        
    }

    pub fn get_neighborhoods(&self) -> DashMap<N, HashSet<N>> {
        self.adj_list.clone()
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

    pub fn add_node(&self, node: N){
        match self.adj_list.get(&node) {
            Some(_) => (),
            None => {self.adj_list.insert(node, HashSet::new());}
        }
    }

    pub fn remove_node(&self, node: N){
        self.adj_list.remove(&node);
    }

    pub fn node_count(&self) -> usize {
        self.adj_list.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adj_list.par_iter()
            .map(|entry| entry.value().len())
            .sum()
    }

    pub fn outgoing_edges(&self, node: N) -> HashSet<N> {
        assert!(self.directed);

        match self.adj_list.get(&node) {
            Some(v) => v.clone(),
            None => HashSet::new()
        }
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

    pub fn incoming_edges_V2(&self, node: N) -> DashSet<N>{
        //try to iterate over nodes and add to res if contains_edge(it, node)
        let res: DashSet<N> = DashSet::new();

        /*
        if edges are like {node: N, d: direction}
            self.adj_list[node].filter(d == Incoming)
        */

        /*
        another approach: 
            separating Incoming and Outgoing edges into different sets
        */


        res
    }

    fn is_directed(&self) -> bool {
        self.directed
    }

    /// Add an edge between two nodes; parallel edges not allowed, but self-loops are
    pub fn add_edge(&self, a: N, b: N) {
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

    /// Get neighbors of a node
    pub fn neighbors(&self, node: N) -> HashSet<N> {
        match self.adj_list.get(&node) {
            Some(vec) => vec.clone(),
            None => HashSet::new()
        }
    }

    /// Check if a node is contained in the graph
    pub fn contains_node(&self, node: N) -> bool {
        self.adj_list.contains_key(&node)
    }

    /// Check if an edge exists between two nodes ~ O(1)
    pub fn contains_edge(&self, node_a: N, node_b: N) -> bool {
        match self.adj_list.get(&node_a) {
            Some(vec) => vec.contains(&node_b),
            None => false
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