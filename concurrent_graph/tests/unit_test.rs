#[cfg(test)]
mod tests{
    use std::collections::HashSet;
    use concurrent_graph::{ConcurrentDiGraph, ConcurrentUnGraph};
    use concurrent_graph::GraphTrait;

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
    fn no_double_edge_insertion(){
        let g = ConcurrentDiGraph::new();
        g.add_edge(0, 1);
        g.add_edge(0, 1);

        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 1);
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


    //#[test]
    /*
    fn un_dir_closed_neigh(){
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

        let h = ConcurrentUnGraph::new();
        h.add_edge(0, 1);
        h.add_edge(0, 2);
        h.add_edge(0, 3);
        h.add_edge(1, 4);
        h.add_edge(0, 1);
        h.add_edge(2, 5);
        h.add_edge(2, 8);
        h.add_edge(3, 6);
        h.add_edge(3, 7);
        h.add_edge(5, 8);
        h.add_edge(9, 10);


        println!("di {:?}", g.get_closed_neighborhoods_undirected());
        println!("un {:?}", h.get_closed_neighborhoods_undirected());

        //assert_eq!(g.get_closed_neighborhoods_undirected(), h.get_closed_neighborhoods_undirected());
    }*/

    #[test]
    fn no_external_add(){
        let g = ConcurrentDiGraph::new();
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(10, 11);

        assert_eq!(g.node_count(), 5);
        assert_eq!(g.edge_count(), 3);

        let x = g.get_neighborhoods(true);
        let mut h = HashSet::new();
        h.insert(200);
        x.insert(100, h);

        //assert_eq!(g.node_count(), 5);
        //assert_eq!(g.edge_count(), 3);

    }
}