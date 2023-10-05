import sys
import json
import numpy as np
import matplotlib.pyplot as plt

if len(sys.argv) == 1:
	try:
		with open(sys.argv[1], "r") as file:
			points = json.load(file)
			points = {int(key):int(value) for key,value in points.items()}
			xpoints = np.array(list(points.keys()))
			ypoints = np.array(list(points.values()))
		
			plt.xlabel("s")
			plt.ylabel("Anzahl der möglichen Quersummen")
			plt.plot(xpoints, ypoints, "o")
			plt.savefig("result_count.png")
	except FileNotFoundError:
		print(f"Die Datei {sys.argv[1]} existiert nicht.")
else:
	print(f"Benutzung: {sys.argv[0]} <file>")
