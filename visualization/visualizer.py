import matplotlib.pyplot as plt
import json

def plot_importance_sampling():
    fig, ax = plt.subplots()
    ax.set_xlim(-1,1)
    ax.set_ylim(-1,1)

    observation = (0, 0)
    ax.scatter(observation[0], observation[1], facecolors="none", edgecolors='r', s=200)

    with open("data/initial_traces.json") as fp:
        initial_traces = json.load(fp)

    xs = [tr['x'] for tr in initial_traces]
    ys = [tr['y'] for tr in initial_traces]

    ax.scatter(xs, ys, c="blue", alpha=0.1, s=1)

    with open("data/resampled_traces.json") as fp:
        resampled_traces = json.load(fp)

    xs = [tr['x'] for tr in resampled_traces]
    ys = [tr['y'] for tr in resampled_traces]

    ax.scatter(xs, ys, c='g', alpha=1, s=10)
    plt.show()

def plot_metropolis_hastings():
    fig, ax = plt.subplots()
    ax.set_xlim(-1,1)
    ax.set_ylim(-1,1)

    observation = (0, 0)
    ax.scatter(observation[0], observation[1], facecolors="none", edgecolors='r', s=200)

    latent_xs = []
    latent_ys = []

    for i in range(25000):
        with open("data/mcmc_latent_%i.json" % i) as fp:
            latent = json.load(fp)
            latent_xs.append(latent["x"])
            latent_ys.append(latent["y"])

    ax.plot(latent_xs, latent_ys, c="blue", alpha=0.5, ls="dotted")
    ax.scatter(latent_xs[0], latent_ys[0], c="green", alpha=1, s=200)

    plt.show()


if __name__ == "__main__":
    # plot_importance_sampling()
    plot_metropolis_hastings()