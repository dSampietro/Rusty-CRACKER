import networkx as nx
import random

from scipy.io import mmwrite
from scipy.sparse import coo_matrix

def create_graph(num_nodes: int, num_edges: int) -> nx.Graph:
    return nx.gnm_random_graph(num_nodes, num_edges)


def save_mtx(graph: nx.Graph, filename: str) -> None:
    with open(filename, 'w') as f:
        # Write the Matrix Market header
        f.write("%%MatrixMarket matrix coordinate integer general\n")
        f.write(f"{graph.number_of_nodes()} {graph.number_of_nodes()} {graph.number_of_edges()}\n")
        
        # Write each edge as a pair of source and destination nodes
        for edge in graph.edges():
            f.write(f"{edge[0]} {edge[1]}\n")

'''def save_mtx(graph: nx.Graph, filename: str) -> None:
    adj_matrix = nx.to_scipy_sparse_array(graph, format='coo')
    mmwrite(filename, adj_matrix)
'''
# Example usage
num_nodes = 50_00

for num_edges in [50_000, 100_000, 200_000, 500_000, 1_000_000, 1_500_000, 2_000_000]:
    #num_edges = 1_000_000
    graph = create_graph(num_nodes, num_edges)


    #assert graph.number_of_nodes() == num_nodes
    #assert graph.number_of_edges() == num_edges


    save_mtx(graph, f"syn{graph.number_of_edges()}.mtx")

print("completed")