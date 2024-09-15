pub mod util_traits {
    use std::{collections::HashSet, hash::Hash};

    use dashmap::DashMap;

    pub trait GraphTrait<N>{
        fn nodes(&self) -> Vec<N>;
        fn add_node(&self, node: N);
        fn remove_node(&self, node: N);
        fn node_count(&self) -> usize;
        fn contains_node(&self, node: N) -> bool;

        fn add_edge(&self, a: N, b: N);
        fn edge_count(&self) -> usize;
        fn outgoing_edges(&self, node: N) -> HashSet<N>;
        //fn incoming_edges(&self, node: N) -> HashSet<N>;        
        fn contains_edge(&self, node_a: N, node_b: N) -> bool;

        fn get_closed_neighborhoods_undirected(&self) -> DashMap<N, HashSet<N>>;
        fn get_all_neighborhoods(&self) -> DashMap<N, HashSet<N>>;
    }


    pub trait NodeTrait: Copy + Ord + Hash + Send + Sync {}
    impl<N> NodeTrait for N where N: Copy + Ord + Hash + Send + Sync {}

}