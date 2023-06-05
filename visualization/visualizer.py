import matplotlib.pyplot as plt
import json


if __name__ == "__main__":

    fig, ax = plt.subplots()
    ax.set_xlim(-1,1)
    ax.set_ylim(-1,1)

    with open("data/observations.json") as fp:
        observations = json.load(fp)

    xs = [p['x'] for p in observations]
    ys = [p['y'] for p in observations]

    ax.scatter(xs, ys, facecolors="none", edgecolors='r', s=200)


    with open("data/initial_particles.json") as fp:
        initial_particles = json.load(fp)

    xs = [tr[0]['x'] for tr in initial_particles]
    ys = [tr[0]['y'] for tr in initial_particles]

    ax.scatter(xs, ys, c="blue", alpha=0.1, s=2)

    with open("data/resampled_initial_particles.json") as fp:
        resampled_initial_particles = json.load(fp)

    xs = [tr[0]['x'] for tr in resampled_initial_particles]
    ys = [tr[0]['y'] for tr in resampled_initial_particles]

    ax.scatter(xs, ys, c='g', alpha=0.1, s=2)
    ax.scatter(observations[0]['x'], observations[0]['y'], facecolors="none", edgecolors='r', s=200)
    plt.show()