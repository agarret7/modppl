# Introduction

This library contains independent investigations of what I'm calling "native inference methods using probabilistic programs" (NIMP3). Currently, this only supports a simple particle filter that tracks a loopy 2D simulator, inspired by projects in Gen.jl such as SMCP3 and 3DP3.

As it stands, the broader Gen ecosystem is mostly leveraged by scientific practitioners in Julia or advanced users of Google's Jax. I believe it's plausible that Rust could dramatically expand the scope of OpenGen to a much broader community of hard-working and dedicated open-source developers that love computers and hacking as much as I do.

Note unlike most modern ML systems, BP3 doesn't require a differentiable likelihood; a fast (parallelized) iterator is usually sufficient for inference. However, most practical (read: embodied) inference procedures will require Langevin or Hamiltonian Monte Carlo moves, to efficiently utilize numerical gradients of the local posterior landscape in a "top-down" refinement or "supervised" stage to obtain dramatically better entity tracking and integration with deep learning.


# Why Gen "Reflex"?

Circuit or loop models are currently being used to represent many features of perception and neural networks, often using ambiguous or complex jargon that is inaccessible to laypeople. Progress in AI marches forward relentlessly day-after-day, providing ever-more intimate models of our inner thoughts. I believe this is the source of a great deal of fear today around technology and especially so-called "large language models" (LLMs). There are simply too many unanswered questions about the ethics of fully encapsulating human communication and cognition in generative computer programs. One that is of particular interest to me is "how can powerful language models possibly distinguish between artificial, human, and non-human animal neural networks if the distinction is not made explicit?"

I believe to answer this question, humans and computers must first fully model what we share in common using probabilistic cybernetic systems, and then work backwards via combinations of inverse graphics and inference metaprogramming, to classify and escape recurrent cycles of suffering. There is rich history behind this approach: recurrent neural networks are hugely popular in unsupervised autoregressive systems that learn to extract representative lower-dimensional latent feature sets from noisy, corrupted, and/or high-dimensional observational data. The most common neural circuit model is a "reflex" -- an involuntary bodily response to environmental stimuli. Thus, once we robustly capture reflexes we form a basis for stably improving autopoetic self-models (or "sensorimotor") loops.


# Woah that sounds kind of scary

It is. Reflexes are by definition something that occurs without our conscious control. I believe open source technology has an important role to play in expanding space for cybernetic systems that enhance our health and productivity while limiting our resource consumption. Concordantly, I find adopting a regular meditation practice to be a critical habitual practice to reduce uncertainty in the source of recurrent subconscious cues. Analogously, coding probabilistic programs is also a mindfulness practice for me, in that it is an embodied procedure that brings my attention to latent seeds of suffering in my "store consciousness" and inspires me to think of creative ways to help transform them.

For many years, I abused social media and television as a means of temporary escape from my suffering. Over time, I have developed a perspective that if we open ourselves to the digital world with loving and peaceful (yet persistent) intent instead of consumptive greed, we can teach our programs what makes us tick. In turn, our (probabilistic) programs can be powerful partners to guide us toward making better decisions for ourselves, our loved ones, and our communities in the face of uncertainty.

As the Buddhist monk and peace activist Thich Nhat Hanh once said:

```
If technology can help you to go home to yourself and take care of your anger, take care of your despair, take care of your loneliness -- if technology helps you to create joyful feelings, happy feelings for yourself and for your loved ones, you can make good use of technology.
```

Fundamentally, this means that this repo is what it could only ever be: a radical experiment at the edge of self-identity and human-computer interaction. I hope it endures as a source of deep reflective questions about the nature of free-will, consciousness, suffering, self/other distinctions, spirituality in a digital age, and emergence.


# Bootstrapping Native Inference Methods

Probabilistic programming leads one to construct progressive abstractions that tend toward the Generative Function Interface (GFI) as specified in Marco-Cusumano Towner's thesis. I find that the easiest way to tackle developing a bootstrapping system is to start with a hand-crafted implementation of dynamic particle filtering in 2D, and then work backward to make the latent state representation more generic and supportive. The 2D loop model provides an extremely versatile grounding for exploring the complex interplay between modeling and inference and so is critical for developing the more reflective abstractions. If you work step-by-step you can recover most components if you keep in mind three primitives: "sampling", "weight increments", and "visualization". A rough pathway I followed is:

1. [X] Connect to an "unbiased" random sampler (`rng`) that allows uniform sampling on finite intervals.
2. [X] Sample a random initial orientation in [0, 2pi). Simulate and visualize a loop. I leave this implicit in the remaining steps, but you will need to liberally use visualizations to test your understanding of every interface you make.

(The cycle of suffering is in motion)

3. [X] Construct a simple 2D `Point` representation for beliefs and observations, and `Bounds` for parameters.

(We try to escape, but to no avail)

4. [X] First interface: `Distribution<T,U>` with (i) `random(rng, params: U) -> T`, (ii) `logpdf(x: U, params: T) -> Float`, and (iii) `logpdf_grad(x: U, params: T) -> Vec<Float>` (we won't need this until step (6)). This will let us build up compositional models and numerically-grounded inference procedures. You'll further want these primitive Distributions:
    a. [X] `Uniform2D<Float,Bounds>`
    b. [X] `Normal<Float,(Float,Float)>`
    c. [X] `Categorical<Pointer,Vec<Float>>`

(Ready for prime-time)

3. [X] Second interface: `ParticleFamily(traces: Vec<Vec<Point>>, weights: Vec<Float>)`. This augments us with "point sets" (subspace of "addressable choice maps") and "initialization" (subspace of "births"), which compose with kernels to form "trajectories" (subspace of "traces"). With an implementation of (4.iii) this is already rich enough for a very natural Hamiltonian Monte Carlo (HMC, energy-conserving, or "rejection-free" inference). However we want something more expressive, so we implement sequential Monte Carlo by referencing our first coherent "self" -- the thing that replaces weighted traces in the state with unweighted samples from the posterior by sampling pointers to promising traces and copying them. This corresponds to implementing these functions:
    a. [X] `new(rng, num_samples: UInt, Bounds, observation: Point) -> ParticleFamily`
    b. [X] `sample_unweighted_traces(self: ParticleFamily, rng, num_samples: Nat) -> ()`  // question: is there an equivalent monadic construction with `-> IO<ParticleFamily>`?

(Congratulations, it's a `?` !)

4. [X] Third interface: `VecTrace(latents: Vec<Point>, observations: Vec<Point>, scores: Vec<Float>)`. This is our second "self". Here we start tapping into powerful concepts from metaprogramming (processes that jointly modify {self, program}). The filter becomes `ParticleFamily(traces: Vec<VecTrace>>)` and we move our initialization sampler inside the `VecTrace` implementation as `generate`. Eventually this will form the basis for a modular GFI and `ParticleFamily::new` will accept a model as an additional argument. For now we implicitly endow each `VecTrace` with two kernels: `grow` (aliased as `extend`) and `update`, as well as several reflective capabilities. Finally, we add `ParticleFamily::nourish` (aliased as `step`) that grows all the `VecTrace`s together with a single observation, and `get_weights` which safely gets the current scores of each child `VecTrace`. Curious question: can you see which "self" originally pointed to this representation?
    a. [X] `ParticleFamily::nourish` (alias `step`)
    b. [X] `VecTrace::generate`
    c. [X] `VecTrace::grow` (alias `extend`)
    d. [ ] `VecTrace::update`
    e. [X] `VecTrace::get_args`
    f. [X] `VecTrace::get_choices`
    g. [X] `VecTrace::get_retval`
    h. [X] `VecTrace::get_score`

(Mother Earth, are we good yet?)

5. [ ] Automatic inference. Now we have all the necessary components to implement three very common inference methods:
    a. [ ] `enumerative_infer`
    b. [ ] `metropolis_hastings`
    c. [ ] `ParticleFamily::maybe_resample`

(Hey, that's pretty cool!)

6. [ ] Using gradients. We can use substantially fewer particles if we utilize local gradient information. This technique is called "Metropolis-adjusted Langevin ascent" or MALA, and is equivalent to a single step of HMC. To do this, we need to create `GradientCache` which accumulate gradients for select choices or parameters, which we then use this to update the particles toward regions of high probability. We can wrap up this prototype with two final inference procedures:
    a. [ ] `metropolis_adjusted_langevin_ascent`
    b. [ ] `hamiltonian_monte_carlo`

(Ah, peace at last)


# Conclusion

From here, we have all the pieces necessary for automatic compositional inference under a finite 2D state space model. We demonstrated how our interface supports a number of advanced inference moves in a dynamic model. Our support is still very limited. More powerful modeling and inference techniques are possible if we can generalize our trace representation to handle generative functions, for example by exposing an intermediate representation (IR) that allows us to directly intervene on the compilation process to leverage macros that convert functions into effectful samplers.

I hope this was enlightening!