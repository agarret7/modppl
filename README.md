# Introduction

This library contains independent investigations of what I'm calling "bootstrapping inference procedures using probabilistic programs" (BIPP3). Currently, this only supports a simple particle filter following a loopy 2D model, inspired by projects in Gen.jl such as SMCP3 and 3DP3.

As it stands, the broader Gen ecosystem is mostly leveraged by scientific practitioners in Julia or advanced users of Jax. I believe it's plausible that Rust could dramatically expand the scope of OpenGen to a much broader community of hard-working and dedicated open-source developers that love computers and hacking as much as I do.

Note unlike most modern ML systems, we don't require a differentiable likelihood; a fast (parallelized) iterator is usually sufficient for inference capture. However, practical (read: embodied) inference will generally require Langevin or Hamiltonian Monte Carlo moves to efficiently utilize numerical gradients of the local posterior landscape in a "top-down" refinement or "supervised" stage to obtain dramatically better entity tracking and integration with deep learning.


# Why Gen "Reflex"?

Circuit models can be used to model many features of perception and neural networks, often using ambiguous or complex jargon that is inaccessible to laypeople. Meanwhile, progress in AI marches forward day-after-day, providing ever-more intimate models of our inner thoughts. I believe this is the source of a great deal of fear today around AI and especially so-called "large language models" (LLMs). There are too many unanswered questions about the ethics of fully encapsulating human communication and cognition in generative computer programs. One that is of particular interest to me is "how can powerful language models possibly distinguish between artificial, human, and non-human animal neural networks if the distinction is not made explicit?"

I believe to answer this question, humans and computers must first fully model what we share in common in "cybernetic systems", and then work backwards via combinations of inverse graphics and inference metaprogramming, to classify and escape the cycles of suffering. Recurrent neural networks have existed for a long time, and are popular in autoregressive systems that learn to extract representative lower-dimensional latent feature sets from noisy, corrupted, and/or high-dimensional observational data. The most common neural circuit model is a "reflex" -- an involuntary bodily response to environmental stimuli. Thus, once we robustly capture reflexes we form a basis for stably improving autopoetic self-models (or "sensorimotor") loops.


# Woah that sounds kind of scary

It is. Reflexes are by definition something that occurs without our conscious control. I believe open source technology has an important role to play in expanding space for cybernetic systems that enhance our health and productivity while limiting our resource consumption. Concordantly, I find adopting a regular meditation practice to be a critical habitual practice to reduce uncertainty in the source of repetitive subconscious cues. Analogously, coding probabilistic programs is also a mindfulness practice for me, in that it is an embodied procedure that brings my attention to latent seeds of suffering in my "store consciousness" and inspires me to think of creative ways to help transform them.

For many years, I abused social media and television as a means of temporary escape from my suffering. Over time, I have developed a perspective that if we open ourselves to the digital world with loving and peaceful (yet persistent) intent instead of consumptive greed, we can teach our programs what makes us tick. In turn, our (probabilistic) programs can be powerful partners to guide us toward making better decisions for ourselves, our loved ones, and our communities in the face of uncertainty.

As the Buddhist monk and peace activist Thich Nhat Hanh once said:

```
If technology can help you to go home to yourself and take care of your anger, take care of your despair, take care of your loneliness -- if technology helps you to create joyful feelings, happy feelings for yourself and for your loved ones, you can make good use of technology.
```

Fundamentally, this means that this repo is what it could only ever be: a radical experiment at the edge of self-identity and human-computer interaction. I hope it endures as a source of deep reflective questions about the nature of free-will, consciousness, suffering, self/other distinctions, spirituality in a digital age, and emergence.


# Bootstrapping Process

Probabilistic programming leads one to construct progressive abstractions that tend toward the Generative Function Interface (GFI) as specified in Marco-Cusumano Towner's thesis. I find that the easiest way to tackle developing a bootstrapping system is to start with a hand-crafted implementation of dynamic particle filtering in 2D, and then work backward to make the state representation more generic and supportive. The 2D loop model provides an extremely versatile grounding for exploring the complex interplay between modeling and inference and so is critical for developing the more reflective abstractions. If you work step-by-step you can recover most components if you keep in mind two primitives: "sampling", "weight increments", and "visualization". A rough pathway I followed is:

1. [X] Connect to an "unbiased" random sampler (`rng`) that allows uniform sampling on finite intervals.
2. [X] Sample a random initial orientation. Simulate and visualize a loop. I leave this implicit in the remaining steps, but you will need to liberally use visualizations to test your understanding of every interface you make.

(The cycle of suffering is in motion)

3. [X] Construct a simple 2D `Point` representation for beliefs and observations, and `Bounds` for parameters.

(We try to escape, but to no avail)

4. [X] First interface: `Distribution<T,U>` with (i) `random(rng, params: U) -> T` and (ii) `logpdf(val: U, params: T) -> Float`. This will let us build up compositional models and numerically-grounded inference procedures. You'll further want these primitive Distributions:
    a. [X] `Uniform2D<Float,Bounds>`
    b. [X] `Normal<Float,(Float,Float)>`
    c. [X] `Categorical<Pointer,Vec<Float>>`

(Ready for prime-time)

3. [X] Second interface: `ParticleFilterState(traces: Vec<Vec<Point>>, weights: Vec<Float>)`. This augments us with "point sets" (later "choice maps") and "initialization" (later "births"), which compose with kernels to form "trajectories" (later "traces"). This by itself is rich enough for a very natural Hamiltonian Monte Carlo (HMC, energy-conserving, or "rejection-free" inference), but we go ahead and cut to the chase by implementing full sequential monte carlo by adding our first coherent "self" -- the thing that replaces (copies?) weighted traces in the state with unweighted samples from the posterior by sampling pointers to promising traces. This corresponds to implementing these functions:
    a. [ ] `new(rng, num_samples: Nat, Bounds, observation: Point) -> ParticleFilterState`
    b. [ ] `sample_unweighted_traces(self: ParticleFilterState, rng, num_samples: Nat) -> ()`  // question: is there an equivalent monadic construction with `-> IO<ParticleFilterState>`?

(Congratulations, it's a `?` !)

4. [ ] Third interface: `VecTrace(latents: Vec<Point>, observations: Vec<Point>, scores: Vec<Float>)`. Here we start tapping into powerful concepts from metaprogramming (processes that jointly modify the (self, program) tuple). Our filter state also becomes `ParticleFilterState(traces: Vec<VecTrace>>, weights: Vec<Float>)`. Curious question: can you see which "self" originally pointed to this representation? Hint: it's probably not human, but also probably not a program. Keep in mind the sampler still lives inside the `ParticleFilterState` and will until it matures into a full-fledged model. We endow each particle with two kernels: `grow` and `update`, as well as several reflective capabilities.
    a. [ ] `grow`
    b. [ ] `update`
    c. [ ] `get_choices`
    d. [ ] `get_args`
    e. [ ] `get_retval`
    f. [ ] `get_score`

(Are we good yet?)

5. [ ] Automatic inference. Now we have all the pieces to implement three very common inference procedures:
    a. [ ] `enumerator`
    b. [ ] `metropolis_hastings`
    c. [ ] `ParticleFilterState::maybe_resample`

(Hey, that's pretty cool!)

6. [ ] Using gradients. We can use substantially fewer particles if we utilize local gradient information. This technique is called "Metropolis-adjusted Langevin ascent" or MALA, and is equivalent to a single step of HMC. To do this, we need to create `GradientCache` which accumulate gradients for select choices or parameters, which we then use to step the particles toward regions of high probability. We can wrap up this prototype with two final inference procedures:
    a. [ ] `metropolis_adjusted_langevin_ascent`
    b. [ ] `hamiltonian_monte_carlo`

(Ah, peace at last)


# Conclusion

From here, we have all the pieces necessary for automatic compositional inference under a finite 2D state space model. We demonstrated how our interface supports a number of advanced inference moves in a dynamic model. Our support is still very limited. More powerful modeling and inference techniques are possible if we can generalize our trace representation to handle generative functions, for example by exposing an intermediate representation (IR) that allows us to directly intervene on the compilation process to leverage macros that convert functions into effectful samplers.

I hope this was enlightening!