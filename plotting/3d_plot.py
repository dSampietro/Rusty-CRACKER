from matplotlib import cm
import matplotlib.pyplot as plt
import pandas as pd

info = pd.read_csv("info.csv")
print(info)

'''ax = plt.axes(projection='3d')

ax.set_xlabel("#nodes")
ax.set_ylabel("#edges")
ax.set_zlabel("time")


ax.plot_trisurf(info["nodes"], info["edges"], info["rayon_base"], cmap=cm.jet)
plt.show()'''


'''fig, ax = plt.subplots(1, 2, sharey=True)

info = info.sort_values(by=["nodes"])
ax[0].plot(info["nodes"], info["rayon_base"], "--bx", label="(R)Base")
ax[0].plot(info["nodes"], info["rayon_ep"],   "--rx", label="(R)EP")
ax[0].plot(info["nodes"], info["rayon_ep+os"],"--cx", label="(R)EP+OS")
ax[0].set_xlabel("#nodes")
ax[0].set_ylabel("time")
ax[0].legend(loc="best")

info = info.sort_values(by=["edges"])
ax[1].plot(info["edges"], info["rayon_base"], "--bx", label="(R)Base")
ax[1].plot(info["edges"], info["rayon_ep"],   "--rx", label="(R)EP")
ax[1].plot(info["edges"], info["rayon_ep+os"],"--cx", label="(R)EP+OS")
ax[1].set_xlabel("#edges")

plt.show()'''


#grouped bars: par_base vs rayon_base
info\
    .sort_values(by=["nodes"])\
    .filter(items=["file", "par_base", "rayon_base"])\
    .plot(x="file", kind="bar", stacked=False)

plt.show()