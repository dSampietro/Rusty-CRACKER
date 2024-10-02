import networkx as nx
#from scipy.io import mmread
import fast_matrix_market as fmm
import time

start = time.time()
BASE_PATH = "../files"
filename = f"{BASE_PATH}/syn/V2_syn_2M_10M.mtx"


#read graph
a = fmm.mmread(filename)
graph = nx.Graph(a)

num_cc = nx.number_connected_components(graph)

#connected components
cc = nx.connected_components(graph)

#force CC creation
for _ in range(0, num_cc):
    next(cc)

print(f"duration: {1000 * (time.time() - start)} ms")