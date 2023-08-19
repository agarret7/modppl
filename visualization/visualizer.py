import matplotlib.pyplot as plt
import json

def plot_importance_sampling():
    fig, ax = plt.subplots()
    ax.set_title("Importance Sampling on Uniform2D with MvNormal Likelihood")
    ax.set_xticks([])
    ax.set_yticks([])

    with open("data/initial_traces.json") as fp:
        initial_traces = json.load(fp)

    xs = [tr[0] for tr in initial_traces]
    ys = [tr[1] for tr in initial_traces]

    ax.scatter(xs, ys, c="blue", alpha=0.1, s=1, label="Prior Samples")

    with open("data/resampled_traces.json") as fp:
        resampled_traces = json.load(fp)

    xs = [tr[0] for tr in resampled_traces]
    ys = [tr[1] for tr in resampled_traces]

    ax.scatter(xs, ys, c='g', alpha=0.5, s=10, label="Importance Samples")

    observation = (0, 0)
    ax.scatter(observation[0], observation[1], facecolors="none", edgecolors='r', s=200, label="Obs")

    plt.legend()
    plt.savefig("visualization/importance.png")

def plot_metropolis_hastings():
    fig, ax = plt.subplots()
    ax.set_title("Metropolis-Hastings on Uniform2D with MvNormal Likelihood")
    ax.set_xticks([])
    ax.set_yticks([])

    latent_xs = []
    latent_ys = []

    for i in range(25000):
        with open("data/mh_trace_%i.json" % i) as fp:
            latent = json.load(fp)
            latent_xs.append(latent[0])
            latent_ys.append(latent[1])

    ax.scatter(latent_xs[0], latent_ys[0], c="blue", alpha=1, s=200, label="Init Latent")
    ax.plot(latent_xs, latent_ys, c="green", alpha=0.5, ls="dotted", label="MH Path")

    observation = (0, 0)
    ax.scatter(observation[0], observation[1], facecolors="none", edgecolors='r', s=200, label="Obs")

    plt.legend()
    plt.savefig("visualization/mh.png")

def plot_hierarchical_model():
    fig, ax = plt.subplots()
    ax.set_title("Hierarchical model")

    with open("data/hierarchical_data.json") as fp:
        data = json.load(fp)

    with open("data/hierarchical_model.json") as fp:
        all_coeffs = json.load(fp)
    
    ax.scatter(data[0], data[1])

    for coeffs in all_coeffs:
        c = "pink" if len(coeffs) == 2 else "blue"
        ax.plot(data[0], [sum(c * x**i for (i, c) in enumerate(coeffs)) for x in data[0]], c=c, alpha=0.1)

    plt.savefig("visualization/hierarchical.png")


if __name__ == "__main__":
    plot_importance_sampling()
    plot_metropolis_hastings()
    plot_hierarchical_model()