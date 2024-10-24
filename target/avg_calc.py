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
            #print(f"Run {_} for file {f}: {time_output}")
            
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
         "par_main", "par_main_ep", "par_main_epos",
         "rayon_main", "rayon_main_ep", "rayon_main_epos"
        ]
'''
files = ["soc-wiki-vote.mtx", "bio-CE-GN.mtx", "bio-HS-CX.mtx", "bio-grid-yeast.mtx", "facebook_artist.mtx", "notredame.mtx", "amazon.mtx", "rec-eachmovie.mtx"]
nodes = [889, 2219, 4412, 6008, 50515, 325729, 334863, 74424] 
edges = [2914, 53683, 108818, 313890, 819306, 1497134, 925872, 2811717]
'''

files = ["syn50000.mtx", "syn100000.mtx", "syn200000.mtx", "syn500000.mtx", "syn1000000.mtx", "syn1500000.mtx", "syn2000000.mtx"]
files = ["syn/" + f for f in files]
nodes = 7 * [50000] 
edges = [50_000, 100_000, 200_000, 500_000, 1_000_000, 1_500_000, 2_000_000]


info = pd.DataFrame()
info["file"] = files
info["nodes"] = nodes
info["edges"] = edges
#info["density"] = 2 * info["edges"] / (info["nodes"] * (info["nodes"] - 1))


N_RUNS = 5
info["naive"] = calc_avg(progs[0], files, N_RUNS)

info["par_base"] = calc_avg(progs[1], files, N_RUNS)
info["par_ep"] = calc_avg(progs[2], files, N_RUNS)
info["par_ep+os"] = calc_avg(progs[3], files, N_RUNS)

info["rayon_base"] = calc_avg(progs[4], files, N_RUNS)
info["rayon_ep"] = calc_avg(progs[5], files, N_RUNS)
info["rayon_ep+os"] = calc_avg(progs[6], files, N_RUNS)



info = info.sort_values(by=["edges"])
print(info)
info.to_csv(path_or_buf="info_parCmp.csv", index=False)



#Plotting
fig, ax = plt.subplots(1, 2, sharey=True)

#vs nodes
info = info.sort_values(by=["nodes"])
ax[0].plot(info["nodes"], info["par_base"], "-bx", label="(P)Base")
ax[0].plot(info["nodes"], info["par_ep"],   "-rx", label="(P)EP")
ax[0].plot(info["nodes"], info["par_ep+os"],"-cx", label="(P)EP+OS")
ax[0].plot(info["nodes"], info["rayon_base"], "--bx", label="(R)Base")
ax[0].plot(info["nodes"], info["rayon_ep"],   "--rx", label="(R)EP")
ax[0].plot(info["nodes"], info["rayon_ep+os"],"--cx", label="(R)EP+OS")

ax[0].set_xlabel("#nodes")
ax[0].set_ylabel("time[ms]")
ax[0].legend(loc="best")

#vs edges
info = info.sort_values(by=["edges"])
ax[1].plot(info["edges"], info["par_base"], "-bx", label="(P)Base")
ax[1].plot(info["edges"], info["par_ep"],   "-rx", label="(P)EP")
ax[1].plot(info["edges"], info["par_ep+os"],"-cx", label="(P)EP+OS")
ax[1].plot(info["edges"], info["rayon_base"], "--bx", label="(R)Base")
ax[1].plot(info["edges"], info["rayon_ep"],   "--rx", label="(R)EP")
ax[1].plot(info["edges"], info["rayon_ep+os"],"--cx", label="(R)EP+OS")

ax[1].set_xlabel("#edges")


plt.show()



'''
I tempi di ep+os sono leggermente maggiori di EP poichè OS aumenta il numero di iterazioni necessarie 
'''