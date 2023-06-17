import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np

ek = pd.read_csv("./data/edmonds_karp.csv", sep=";")
ek['flow'] = np.log2(ek["flow"])
sns.lineplot(data=ek, x="n", y="flow")
plt.show()
