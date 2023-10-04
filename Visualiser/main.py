import matplotlib.pyplot as plt
import numpy as np
import json

with open("result_count.txt", "r") as file:
	points = json.load(file)
	points = {int(key):int(value) for key,value in points.items()}
	xpoints = np.array(list(points.keys()))
	ypoints = np.array(list(points.values()))

	plt.xlabel("s")
	plt.ylabel("Anzahl der möglichen Quersummen")
	plt.plot(xpoints, ypoints, "o")
	plt.savefig("result_count.png")
