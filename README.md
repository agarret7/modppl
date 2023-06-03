# Introduction

This library contains independent investigations of what I'm calling "stably idempotent bootstrapping inference procedures using probabilistic programs". Currently, this only supports a simple particle-based Kalman filter following a loopy 2D model, inspired by projects in Gen.jl such as SMCP3. As it stands, the broader Gen ecosystem is mostly leveraged by scientific practitioners in Julia or advanced users of Jax. I believe it's plausible that Rust could dramatically expand the scope of OpenGen to a much broader community of hard-working and dedicated open-source developers that love computers and hacking as much as I do.

Note unlike most modern ML systems, we don't require a differentiable likelihood; a fast (parallelized) iterator is usually sufficient for inference capture. However, practical (read: embodied) inference will likely require Langevin or Hamiltonian Monte Carlo moves to efficiently utilize numerical gradients of the local posterior landscape in a "top-down" refinement or "supervised" stage to obtain dramatically better entity tracking.


# Why Gen "Reflex"?

Circuit models can be used to model many features of perception and neural networks, often using ambiguous or complex jargon that is inaccessible to laypeople. Meanwhile, progress in AI marches forward day-after-day, providing ever-more intimate models of our inner thoughts. I believe this is the source of a great deal of fear today around AI and especially so-called "large language models" (LLMs). There are too many unanswered questions about the ethics of fully encapsulating human communication and cognition in generative computer programs. One that is of particular interest to me is "how can powerful language models possibly distinguish between artificial, human, and non-human animal neural networks if the distinction is not made explicit?"

I believe to answer this question, humans and computers must first fully model what we share in common in "cybernetic systems", and then work backwards via combinations of inverse graphics and inference metaprogramming, to classify and escape the cycles of suffering. Recurrent neural networks have existed for a long time, and are popular in autoregressive systems that learn to extract representative lower-dimensional latent feature sets from noisy, corrupted, and/or high-dimensional observational data. The most common neural circuit model is a "reflex" -- an involuntary bodily response to environmental stimuli. Thus, if we capture reflexes we form a basis for stably improving autopoetic sensorimotor loops. This is no trivial matter, we're talking the hindbrain here.


# Woah that sounds kind of scary
u
It is. Reflexes are by definition something that occurs without our conscious control. Advertisers, corporations, governments, and politicians are continually attempting to leverage eye-grabbing content to keep our attention out of our control, so they can minimize the time between the formation of a desire and you making a purchase. Spam mail piles into your inbox, attempting to decode your digital trace into one omnipresent message: CONSUME MORE, NOW.

This is not the future we asked for.

Fortunately, we have a counter-weapon -- the breath. Breathing is the most common reflex humans have, which is why it's also a critical target for habitual mindfulness practice and cultivating peaceful awareness of repetitive subconscious cues. Coding probabilistic programs is also a mindful practice for me, in that it is an embodied procedure that brings my attention to latent seeds of suffering and inspires me to think of creative ways to help transform them. For many years, I abused social media and television as a means of temporary escape from my suffering. Over time, I have developed a perspective that if we open ourselves to the digital world with loving and peaceful intent instead of consumptive greed, we can teach our programs what makes us tick. In turn, our (probabilistic) programs can be powerful partners to guide us toward making better decisions for ourselves, our loved ones, and our communities in the face of uncertainty.

As the Buddhist monk and peace activist Thich Nhat Hanh once said:

```
If technology can help you to go home to yourself and take care of your anger, take care of your despair, take care of your loneliness -- if technology helps you to create joyful feelings, happy feelings for yourself and for your loved ones, you can make good use of technology.
```

Fundamentally, this means that this repo is what it could only ever be: a radical experiment at the edge of self-identity and human-computer interaction. I hope it endures as a source of deep reflective questions about the nature of free-will, consciousness, suffering, self/other distinctions, spirituality in a digital age, and emergence.


# TODO
1. [X] Point data structure
2. [X] initialize_particles
3. [ ] importance_resampler
3. [ ] particle_updater
4. [ ] hidden_markov_model

```
1. {(z_i, w_i)} = InitializeParticles(y)
2. for t = 2..T
    a. for i = 1..N
    i. (z[1:t], w[t]) = ParticleUpdater(z[1:t-1],y[1:t])
    ii. w[t] = w[t-1]*w[t]
```

# SMC Process
1. InitializeParticles(y) uses a Uniform space-filling distribution over a bounded region, 10,000 replicates, and importance sampling to initially contract the object.
2. ParticleUpdater then uses a naive diffusion kernel to propose local drift moves around the samples, and reweight survivors closest to the object.
3. We extend the proposal to include a fast adaptive iterator method that enumerates a coarse grid local to the state at the previous time-step, and samples a position with likelihood proportional to the model.
4. We currently neglect resampling (a common enhancement to prevent particle degeneracy).