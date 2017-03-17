#pylint: disable=E,C

from shutil import copyfile
copyfile('target/debug/libising_triangular.so', 'ising_triangular.so')

import ising_triangular as it
import numpy as np
import matplotlib.pyplot as plt

n = 24
xs = [-np.ones((32, 32), dtype=np.int32) for i in range(n)]
ts = np.linspace(2, 14, n)

for _ in range(15):
    for i in range(n):
        it.sweep(xs[i], ts[i])

    for i in range(n):
        [i, j] = np.random.choice(n, 2, replace=False)
        if np.random.uniform() < np.exp((it.energy(xs[i]) - it.energy(xs[j])) * (1 / ts[i] - 1 / ts[j])):
            ts[i], ts[j] = ts[j], ts[i]

# reorder from cold to hot
order = np.argsort(ts)
ts = [ts[i] for i in order]
xs = [xs[i] for i in order]

for i in range(n):
    plt.subplot(3, np.ceil(n / 3), i + 1)
    plt.imshow(xs[i])
    plt.title("{:.2f}".format(ts[i]))
    plt.xticks([])
    plt.yticks([])

plt.show()
