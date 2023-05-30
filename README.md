# Introduction

This library contains independent investigations of a bootstrapped inference procedure for state-space models also supported by SMCP3. Currently this only supports a simple Kalman filter following a loopy 2D model. Currently Gen is mostly leveraged by scientific practitioners in Julia or advanced users of Jax. It may not look like much, but probabilistic programming has given a lot to me. I believe Rust could have the potential to dramatically expand the scope of Gen to a much broader community of hard-working and dedicated open-source developers that love computers and hacking as much as I do.

The vision is to implement a (memory-safe) fully-supported self-specializing sequential Monte Carlo process in Rust. If this "baby" experiment is successful, the next step would be to implement Langevin Monte Carlo moves to utilize gradients of the local posterior landscape. It may also just stand alone as inspiration for the next generation of probabilistic programming practitioners to prefer new languages in the "open world" rather than retreading old ground.


# Why Gen "Reflex"?

Circuit models represent many features of perception and neural networks, often using ambiguous or complex jargon that is inaccessible to laypeople. Nevertheless, progress in AI marches forward, providing ever-more detailed models of our inner thoughts. There are many unanswered questions about the ethics of modeling human perception using generative computer programs. One that is of particular interest to me is "how can an algorithm distinguish between an artificial and a human neural network?"

I believe to answer this question, we must first model what we share in common, and work backwards. The most common neural circuit model is a "reflex" -- an involuntary bodily response to environmental stimuli. The most common reflex in humans is breathing, which is why it's also a great target for habitual mindfulness practice and cultivating peaceful awareness of our subconscious cues. Coding probabilistic programs is also a mindful practice for me, in that it implements a method to bring my attention to latent seeds of suffering and inspires me to think of creative ways to help transform them.

I'm fundamentally a techno-optimist. This is despite the many difficult or frustrating experiences I've had with computers in the past. For many years, I abused social media and television as a means of temporary escape from my suffering. Over time, I have developed a perspective that if we open ourselves to the digital world with loving intent instead of greed, we can teach our programs what makes us tick. In turn, our (probabilistic) programs can be powerful partners to guide us toward making better decisions for ourselves, our loved ones, and our communities in the face of uncertainty.

Fundamentally, this means that this repo is what it could only ever be: a radical experiment at the edge of self-identity and human-computer interaction. Gen.jl has already inspired me to stop smoking weed and drinking alcohol, to meditate daily, and to reduce meat in my diet.


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