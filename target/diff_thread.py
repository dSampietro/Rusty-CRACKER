from matplotlib import pyplot as plt
import pandas as pd
import platform
import subprocess
from typing import List

EXTENSION = ".exe" if platform.platform() == "Windows" else ""

def calc_times_with_nThreads(prog: str, file: str, n_threads: List[int], n_runs = 5) -> List[float]:
    avg_times = []

    for n in n_threads:
        run_times = []
        # Run the program 5 times
        for _ in range(0, n_runs):
            # Execute the command and capture the output
            result = subprocess.run([f"./release/{prog}{EXTENSION}", "--f", f"../files/{file}", "--num_thread", f"{n}"], capture_output=True, text=True)
            
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
         "par_main_base", "par_main", "par_main_opt",
         "rayon_main_base", "rayon_main", "rayon_main_opt"
        ]


files = "facebook_artist.mtx"
nodes = 50000
edges = 500_000


N_THREADS = [0, 1, 2, 4, 8, 16]
N_RUNS = 5


info = pd.DataFrame()
#info["density"] = 2 * info["edges"] / (info["nodes"] * (info["nodes"] - 1))


info["num_threads"] = N_THREADS
#info["naive"] = calc_times_with_nThreads(progs[0], files, N_RUNS)
info["par_base"] = calc_times_with_nThreads(progs[1], files, N_THREADS, N_RUNS)
info["par_ep"] = calc_times_with_nThreads(progs[2], files, N_THREADS, N_RUNS)
info["par_ep+os"] = calc_times_with_nThreads(progs[3], files, N_THREADS, N_RUNS)
info["rayon_base"] = calc_times_with_nThreads(progs[4], files, N_THREADS, N_RUNS)
info["rayon_ep"] = calc_times_with_nThreads(progs[5], files, N_THREADS, N_RUNS)
info["rayon_ep+os"] = calc_times_with_nThreads(progs[6], files, N_THREADS, N_RUNS)



#info = info.sort_values(by=["edges"])
print(info)
info.to_csv(path_or_buf="facebook_diff_thread.csv", index=False)

'''
#Plotting
fig, ax = plt.subplots(1, 2, sharey=True)

#vs nodes
info = info.sort_values(by=["nodes"])
ax[0].plot(info["nodes"], info["rayon_base"], "--bx", label="(R)Base")
ax[0].plot(info["nodes"], info["rayon_ep"],   "--rx", label="(R)EP")
ax[0].plot(info["nodes"], info["rayon_ep+os"],"--cx", label="(R)EP+OS")
ax[0].set_xlabel("#nodes")
ax[0].set_ylabel("time[ms]")
ax[0].legend(loc="best")

#vs edges
info = info.sort_values(by=["edges"])
ax[1].plot(info["edges"], info["rayon_base"], "--bx", label="(R)Base")
ax[1].plot(info["edges"], info["rayon_ep"],   "--rx", label="(R)EP")
ax[1].plot(info["edges"], info["rayon_ep+os"],"--cx", label="(R)EP+OS")
ax[1].set_xlabel("#edges")


plt.show()'''



'''
I tempi di ep+os sono leggermente maggiori di EP poichè OS aumenta il numero di iterazioni necessarie 
'''