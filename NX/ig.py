import time
import igraph as ig
import numpy as np

# Load the graph from a .mtx file
def load_graph_from_mtx(file_path):
    edges = np.loadtxt(file_path, skiprows=2, dtype=int)
    graph = ig.Graph.TupleList(edges.tolist(), directed=False)

    return graph


if __name__ == "__main__":
    file_path = '../files/syn/bn-human-Jung2015_M87124670.mtx'
    graph = load_graph_from_mtx(file_path)
    
    start = time.time()
    
    # Compute connected components
    components = graph.connected_components("weak")
    #print(components)
    
    # Print the number of connected components and their sizes
    print(f"Number of connected components: {len(components)}")
    for i, component in enumerate(components):
        print(f"Component {i}: Size = {len(component)}")

    print(f"duration: {time.time() - start} s")