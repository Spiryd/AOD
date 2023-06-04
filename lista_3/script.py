import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

df = pd.read_csv("dijkstra_data.csv", header=0, sep=";")
print(df.value_counts("n"))
print(df.head())
df = df.astype({'n': 'int', "time": "float"})

sns.set_style("darkgrid")
sns.barplot(data=df, x="n", y="time", hue="algo")
plt.savefig("ok.png")
