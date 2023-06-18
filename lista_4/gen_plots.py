import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np

sns.set_style("darkgrid")

ek = pd.read_csv("./data/edmonds_karp.csv", sep=";")
ek['log2_flow'] = np.log2(ek["flow"])
ek['log2_time'] = np.log2(ek["time"])
ek['log2_aug_count'] = np.log2(ek["aug_count"])
sns.lineplot(data=ek, x="n", y="flow")
plt.title("edmonds-karp maxflow")
plt.savefig("./plots/ek_flow.png")
plt.clf()

sns.lineplot(data=ek, x="n", y="aug_count")
plt.title("edmonds-karp augmented paths")
plt.savefig("./plots/ek_aug.png")
plt.clf()

sns.lineplot(data=ek, x="n", y="time")
plt.title("edmonds-karp time")
plt.savefig("./plots/ek_time.png")
plt.clf()

sns.lineplot(data=ek, x="n", y="log2_flow")
plt.title("edmonds-karp log2 maxflow")
plt.savefig("./plots/ek_log2_flow.png")
plt.clf()

sns.lineplot(data=ek, x="n", y="log2_aug_count")
plt.title("edmonds-karp log2 augmented paths")
plt.savefig("./plots/ek_log2_aug.png")
plt.clf()

sns.lineplot(data=ek, x="n", y="log2_time")
plt.title("edmonds-karp log2 time")
plt.savefig("./plots/ek_log2_time.png")
plt.clf()


dinic = pd.read_csv("./data/dinic.csv", sep=";")
dinic['log2_flow'] = np.log2(dinic["flow"])
dinic['log2_time'] = np.log2(dinic["time"])
dinic['log2_aug_count'] = np.log2(dinic["aug_count"])
sns.lineplot(data=dinic, x="n", y="flow")
plt.title("dinic maxflow")
plt.savefig("./plots/d_flow.png")
plt.clf()

sns.lineplot(data=dinic, x="n", y="aug_count")
plt.title("dinic augmented paths")
plt.savefig("./plots/d_aug.png")
plt.clf()

sns.lineplot(data=dinic, x="n", y="time")
plt.title("dinic time")
plt.savefig("./plots/d_time.png")
plt.clf()

sns.lineplot(data=dinic, x="n", y="log2_flow")
plt.title("dinic maxflow log2")
plt.savefig("./plots/d_log2_flow.png")
plt.clf()

sns.lineplot(data=dinic, x="n", y="log2_aug_count")
plt.title("dinic log2 augmented paths")
plt.savefig("./plots/d_log2_aug.png")
plt.clf()

sns.lineplot(data=dinic, x="n", y="log2_time")
plt.title("dinic log2 time")
plt.savefig("./plots/d_log2_time.png")
plt.clf()

mcm = pd.read_csv("./data/mcm.csv", sep=";")
for k in range(3, 11):
    tmp = mcm[mcm['k'] == k]
    sns.lineplot(data=tmp, x="i", y="size")
    plt.title(f"hopcroft_karp matchings k={k}")
    plt.savefig(f"./plots/hk_size_{k}.png")
    plt.clf()

for i in range(1, 11):
    tmp = mcm[mcm['i'] == i]
    sns.lineplot(data=tmp, x="k", y="time")
    plt.title(f"hopcroft_karp time i={i}")
    plt.savefig(f"./plots/hk_time_{i}.png")
    plt.clf()

dinic["type"] = "dinic"
ek["type"] = "edmonds-karp"
max_flow_algs = pd.concat([dinic, ek])

sns.lineplot(data=max_flow_algs, x="n", y="aug_count", hue="type")
plt.title("augmented paths")
plt.savefig("./plots/aug.png")
plt.clf()

sns.lineplot(data=max_flow_algs, x="n", y="time", hue="type")
plt.title("time")
plt.savefig("./plots/time.png")
plt.clf()

sns.lineplot(data=max_flow_algs, x="n", y="log2_aug_count", hue="type")
plt.title("log2 augmented paths")
plt.savefig("./plots/log2_aug.png")
plt.clf()

sns.lineplot(data=max_flow_algs, x="n", y="log2_time", hue="type")
plt.title("log2 time")
plt.savefig("./plots/log2_time.png")
plt.clf()
