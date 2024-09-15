#[cfg(test)]
mod tests{
    use std::collections::HashSet;
    use concurrent_digraph::prelude::ConcurrentDiGraph;

    #[test]
    fn creation_test(){
        let g = ConcurrentDiGraph::new();

        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(0, 3);
        g.add_edge(1, 4);
        g.add_edge(0, 1);
        g.add_edge(2, 5);
        g.add_edge(2, 8);
        g.add_edge(3, 6);
        g.add_edge(3, 7);
        g.add_edge(5, 8);
        g.add_edge(9, 10);

        assert_eq!(g.node_count(), 11);
        assert_eq!(g.edge_count(), 10);

        assert!(g.contains_edge(0, 1));
        assert!(g.contains_edge(1, 0) == false);
    }

    #[test]
    fn edge_test(){
        let g = ConcurrentDiGraph::new();

        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(0, 3);
        g.add_edge(1, 4);
        g.add_edge(0, 1);
        g.add_edge(2, 5);
        g.add_edge(2, 8);
        g.add_edge(3, 6);
        g.add_edge(3, 7);
        g.add_edge(5, 8);
        g.add_edge(9, 10);

        //let h = HashSet::with_capacity(10);
        //h.insert()


        assert_eq!(g.incoming_edges(8), g.neighbors_incoming(8));
    }


    #[test]
    fn neighborhood_test(){
        let g = ConcurrentDiGraph::new();

        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(0, 3);
        g.add_edge(1, 4);
        g.add_edge(0, 1);
        g.add_edge(2, 5);
        g.add_edge(2, 8);
        g.add_edge(3, 6);
        g.add_edge(3, 7);
        g.add_edge(5, 8);
        g.add_edge(9, 10);


        let mut h = HashSet::new();
        h.insert(5);
        h.insert(8);

        assert_eq!(g.neighbors_outgoing(2), h);

        let mut j = HashSet::new();
        j.insert(0);
        assert_eq!(g.neighbors_incoming(2), j);
    }
}