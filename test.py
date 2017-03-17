#pylint: disable=E,C

from shutil import copyfile
copyfile('target/release/libising_triangular.so', 'ising_triangular.so')

import ising_triangular as it
import numpy as np
import matplotlib.pyplot as plt
import threading
# import multiprocessing
import time
# import joblib

n = 100
a = 64
xs = [(-1)**i * np.ones((a, a), dtype=np.int32) for i in range(n)]
es = [-3 * a * a] * n
ts = np.linspace(1, 10, n)

start = time.time()

def sweep(k):
    es[k] = it.sweep(xs[k], ts[k], es[k])

for _ in range(15):
    # create threads
    def sweep(k):
        def foo():
            es[k] = it.sweep(xs[k], ts[k], es[k])
        return foo

    threads = [threading.Thread(target=sweep(i)) for i in range(n)]

    # start all the threads
    for t in threads:
        t.start()

    # wait threads to finish
    for t in threads:
        t.join()

    # pool = multiprocessing.Pool(4)
    # pool.map(sweep, range(n))

    # joblib.Parallel(n_jobs=4)(joblib.delayed(sweep)(k) for k in range(n))

    for _ in range(2 * n):
        [i, j] = np.random.choice(n, 2, replace=False)
        if np.random.uniform() < np.exp((es[i] - es[j]) * (1 / ts[i] - 1 / ts[j])):
            ts[i], ts[j] = ts[j], ts[i]

print(time.time() - start)

# reorder from cold to hot
order = np.argsort(ts)
ts = [ts[i] for i in order]
xs = [xs[i] for i in order]

for i in range(n):
    sq = np.floor(np.sqrt(n / 1.5))
    plt.subplot(sq, np.ceil(n / sq), i + 1)
    plt.imshow(xs[i], cmap='gray')
    plt.title("{:.2f}".format(ts[i]))
    plt.xticks([])
    plt.yticks([])

    print("{} => {}".format(i, np.sum(xs[i]) / (a*a)))

plt.show()
