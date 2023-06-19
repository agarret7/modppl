import matplotlib.pyplot as plt
import json

if __name__ == "__main__":
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
    ax.scatter(observation[0], observation[0], facecolors="none", edgecolors='r', s=200)
    plt.show()


# if __name__ == "__main__":
# 
#     fig, ax = plt.subplots()
#     ax.set_xlim(-1,1)
#     ax.set_ylim(-1,1)
# 
#     with open("data/observations.json") as fp:
#         observations = json.load(fp)
# 
#     xs = [p['x'] for p in observations]
#     ys = [p['y'] for p in observations]
# 
#     # ax.scatter(xs, ys, facecolors="none", edgecolors='r', s=200)
# 
# 
#     with open("data/initial_traces.json") as fp:
#         initial_traces = json.load(fp)
# 
#     xs = [tr[0]['x'] for tr in initial_traces]
#     ys = [tr[0]['y'] for tr in initial_traces]
# 
#     ax.scatter(xs, ys, c="blue", alpha=0.1, s=2)
# 
#     with open("data/resampled_initial_traces.json") as fp:
#         resampled_initial_traces = json.load(fp)
# 
#     xs = [tr[0]['x'] for tr in resampled_initial_traces]
#     ys = [tr[0]['y'] for tr in resampled_initial_traces]
# 
#     ax.scatter(xs, ys, c='g', alpha=1, s=100)
#     ax.scatter(observations[0]['x'], observations[0]['y'], facecolors="none", edgecolors='r', s=200)
# 
#     mh_seq_xs = []
#     mh_seq_ys = []
#     t = 7
#     for iter in range(1,100):
#         with open("data/mh_t_%i_iter_%i.json" % (t, iter)) as fp:
#             traces = json.load(fp)
#         for tr in traces:
#             mh_seq_xs.append(tr[-1]['x'])
#             mh_seq_ys.append(tr[-1]['y'])
# 
#     # with open("data/mh_t_%i_iter_99.json" % t) as fp:
#     #     traces = json.load(fp)
#     # example_tr = traces[0]
#     # mh_seq_xs.append(example_tr[-1]['x'])
#     # mh_seq_ys.append(example_tr[-1]['y'])
# 
#     ax.scatter(mh_seq_xs, mh_seq_ys, c='purple', alpha=0.5, s=10)
#     ax.scatter(observations[t]['x'], observations[t]['y'], facecolors="none", edgecolors='b', s=200)
# 
#     plt.show()
# 
#     # for t in range(1,100):
#     #     for iter in range(1,100):
#     #         with open("data/mh_t_%i_iter_%i.json" % (t, iter)) as fp:
#     #             traces = json.load(fp)
# 
#     #         tr = traces[0]
#     #         print(len(tr))
