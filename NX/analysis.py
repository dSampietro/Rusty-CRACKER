import igraph as ig
import numpy as np

def load_graph_from_mtx(file_path):
    edges = np.loadtxt(file_path, skiprows=2, dtype=int)
    graph = ig.Graph.TupleList(edges.tolist(), directed=False)

    return graph

def analyze_graph(mtx_file):
    # Load the graph from the MTX file
    graph = load_graph_from_mtx(mtx_file)
    
    print("Graph Analysis Results:")
    
    # Calculate metrics
    node_count = graph.vcount()  # Number of nodes
    print(f"Node Count: {node_count}")
    
    edge_count = graph.ecount()  # Number of edges
    print(f"Edge Count: {edge_count}")
    
    avg_degree = sum(graph.degree()) / node_count if node_count > 0 else 0  # Average degree
    print(f"Average Degree: {avg_degree}")
    
    connected_components = graph.components()  # Connected components
    num_connected_components = len(connected_components)  # Number of connected components
    print(f"Number of Connected Components: {num_connected_components}")

    # Optionally, you can return the results as a dictionary
    return {
        "node_count": node_count,
        "edge_count": edge_count,
        "avg_degree": avg_degree,
        "num_connected_components": num_connected_components
    }

if __name__ == "__main__":
    mtx_file_path = '../files/bn-human-Jung2015_M87124670.mtx'
    analyze_graph(mtx_file_path)