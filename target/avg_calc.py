import subprocess
from matplotlib import pyplot as plt
import pandas as pd
from typing import List


def calc_avg(prog: str, files: List[str], n_runs = 5) -> List[float]:
    avg_times = []

    for f in files:
        run_times = []
        # Run the program 5 times
        for _ in range(0, n_runs):
            # Execute the command and capture the output
            result = subprocess.run([f"./{prog}.exe", "--f", f"../../files/{f}"], capture_output=True, text=True)
            
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




progs = ["main_base", "main", "main_opt"]
files = ["bio-diseasome.mtx", "soc-wiki-vote.mtx", "bio-CE-GN.mtx", "bio-HS-CX.mtx", "bio-grid-yeast.mtx", "rec-eachmovie.mtx"]
edges = [1188, 2914, 53683, 108818, 313890, 2811717]

info = pd.DataFrame()
info["file"] = files
info["edges"] = edges
info["base"] = calc_avg(progs[0], files)
info["ep"] = calc_avg(progs[1], files)
info["ep+os"] = calc_avg(progs[2], files)


print(info)

plt.plot(info["edges"], info["base"], "--bo", label="Base")
plt.plot(info["edges"], info["ep"], "--ro", label="EP")
plt.plot(info["edges"], info["ep+os"], "--co", label="EP+OS")


plt.xlabel("#edges")
plt.ylabel("time [ms]")
plt.legend(loc="best")
plt.xscale('log')

plt.show()

'''
I tempi di ep+os sono leggermente maggiori di EP poichè OS aumenta il numero di iterazioni necessarie 
'''