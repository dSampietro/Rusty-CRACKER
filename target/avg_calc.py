from matplotlib import pyplot as plt
import pandas as pd
import platform
import subprocess
from typing import List

EXTENSION = ".exe" if platform.platform() == "Windows" else ""

def calc_avg(prog: str, files: List[str], n_runs = 5) -> List[float]:
    avg_times = []

    for f in files:
        run_times = []
        # Run the program 5 times
        for _ in range(0, n_runs):
            # Execute the command and capture the output
            result = subprocess.run([f"./release/{prog}{EXTENSION}", "--f", f"../files/{f}"], capture_output=True, text=True)
            
            # Get the output
            time_output = result.stdout.strip()
            #print(f"Run {i} for file {f}: {time_output}")
            
            # Remove the "ms" suffix and add to list
            try:
                milliseconds = int(time_output)
                run_times.append(milliseconds)
            except ValueError:
                run_times.append(0)

        # Calculate the average time in milliseconds
        total = sum(run_times)
        average = total / len(run_times)
        
        # Output the average time in milliseconds
        #print(f"{f} average time: {average:.2f} ms")
        
        avg_times.append(average)

    return avg_times




progs = ["naive", 
         "par_main_base", "par_main", "par_main_opt",
         "rayon_main_base", "rayon_main", "rayon_main_opt"
        ]

files = ["bio-diseasome.mtx", "soc-wiki-vote.mtx", "bio-CE-GN.mtx", "bio-HS-CX.mtx", "bio-grid-yeast.mtx", "facebook_artist.mtx"]#, "notredame.mtx", "amazon.mtx", "rec-eachmovie.mtx"]
nodes = [516, 889, 2219, 4412, 6008, 50515]#, 325729, 334863, 74424] 
edges = [1188, 2914, 53683, 108818, 313890, 819306]#, 1497134, 925872, 2811717]


info = pd.DataFrame()
info["file"] = files
info["nodes"] = nodes
info["edges"] = edges
info["density"] = 2 * info["edges"] / (info["nodes"] * (info["nodes"] - 1))


N_RUNS = 5
info["naive"] = calc_avg(progs[0], files, N_RUNS)

#info["par_base"] = calc_avg(progs[1], files, N_RUNS)
#info["par_ep"] = calc_avg(progs[2], files, N_RUNS)
#info["par_ep+os"] = calc_avg(progs[3], files, N_RUNS)

info["rayon_base"] = calc_avg(progs[4], files, N_RUNS)
info["rayon_ep"] = calc_avg(progs[5], files, N_RUNS)
info["rayon_ep+os"] = calc_avg(progs[6], files, N_RUNS)



info = info.sort_values(by=["edges"])
print(info)


#Plotting
# vs edges

plt.xlabel("#edges")
plt.ylabel("time [ms]")
plt.xscale('log')

plt.plot(info["edges"], info["naive"], ":yo", label="Naive")

#plt.plot(info["edges"], info["par_base"], "--bo", label="(P)Base")
#plt.plot(info["edges"], info["par_ep"], "--ro", label="(P)EP")
#plt.plot(info["edges"], info["par_ep+os"], "--co", label="(P)EP+OS")

plt.plot(info["edges"], info["rayon_base"], "-.bx", label="(R)Base")
plt.plot(info["edges"], info["rayon_ep"],   "-.rx", label="(R)EP")
plt.plot(info["edges"], info["rayon_ep+os"],"-.cx", label="(R)EP+OS")
plt.legend(loc="best")
plt.show()



#vs nodes
info = info.sort_values(by=["nodes"])
plt.xlabel("#nodes")
plt.ylabel("time [ms]")
plt.xscale('log')

plt.plot(info["nodes"], info["rayon_base"], "-.bx", label="(R)Base")
plt.plot(info["nodes"], info["rayon_ep"],   "-.rx", label="(R)EP")
plt.plot(info["nodes"], info["rayon_ep+os"],"-.cx", label="(R)EP+OS")

plt.legend(loc="best")
plt.show()



'''
I tempi di ep+os sono leggermente maggiori di EP poichè OS aumenta il numero di iterazioni necessarie 
'''




# 3d plot 
ax = plt.axes(projection='3d')

import numpy as np
x, y = np.meshgrid(info["nodes"], info["edges"])
ax.plot_surface(x, y, info["rayon_base"], cmap='viridis')
ax.set_xlabel("#nodes")
ax.set_ylabel("#edges")
ax.set_title('surface')

plt.show()
