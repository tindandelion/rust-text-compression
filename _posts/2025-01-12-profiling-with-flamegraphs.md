---
layout: post
title: "Profiling with Flamegraphs"
date: 2025-01-12
---

To analyze where my program is spending most of its time, I'm going to need a profiler to collect the execution data. It seems that there are a few options existing for Rust, notably:

- [cargo-flamegraph][cargo-flamegraph]
- [cargo-instruments](https://crates.io/crates/cargo-instruments)
- [valgrind](https://valgrind.org/)

The tool that pops out for me immediately is `cargo-flamegraph`. I used flamegraphs as a visualization tool before in other projects, so that was is my natural first choice. It looks like the latter two tools offer a lot more functionality, e.g. profiling the memory allocations, but they require use of external tools (namely, Apple's Instruments or Valgrind). At the moment they look like an overkill for the task at hand. So, I'm proceeding with `cargo-flamegraph`.

# Generating the flamegraph

The installation and usage instructions are thoroughly documented in the [cargo-flamegraph repository][cargo-flamegraph]. Having it installed, I ran the following command to generate the `flamegraph.svg` in the project's root directory:

```bash
cargo flamegraph --bin=performance-test --dev
```

Here's the flamegraph that was generated. Click on it to see the full-size interactive version: 

[![Generated flamegraph][generated-flamegraph]][generated-flamegraph]

# Analyzing the flamegraph

[Flamegraphs][flamegraph-docs] were popularized by Brendan Gregg, as a tool to visualize the call stack traces. One remarkable thing I like about them is that they are very easy to interpret. Even if I don't understand a single thing about what I'm looking at, it's clear that something fishy is going on in [`SubstringLedger::values()`][substring-ledger-values] method. That's just enough to start digging, but I'd like to spend a few minutes to describe what flamegraphs represent.

The way `cargo-flamegraph` works is that it launches the target program and starts sampling stack traces at regular intervals. On Mac OS, it uses a tool called [dtrace](https://dtrace.org/about/) to do the sampling. The sampling process generates a ton of data points, which are then processed by merging the similar stack traces together, and generating the interactive SVG image.

On the Y-axis of the graph, we see the stack frames, bottom to top. Down below is the bottom of the call stack, i.e. the function that was called first, and the tip of each "flame" is the function that was being executed on the CPU at the moment the sample was taken.

On the X-axis, same function calls in stack traces are grouped together into boxes. The width of each box is proportional to the number of samples that contained that function call. So wide boxes represent the functions that take long time to execute, or are called more often than others.

The left-to-right order of function calls on the X-axis doesn't actually represent anything. In particular, it **doesn't show** the passing of time or the order of execution. It's just a visual grouping of function calls. Also, the color of each box isn't significant. They are chosen somewhat at random, just to make the graph more readable.

# The verdict

So, what does this flamegraph tell us? Well, the picture is pretty clear: [`SubstringLedger::find_longest_match()`][substring-ledger-longest-match] is the culprit. Essentially, we're spending most of the time sorting the substrings by length. There must be a better way to do that, and I'm going to tackle that as the [next step][next-step].




[cargo-flamegraph]: https://github.com/flamegraph-rs/flamegraph
[generated-flamegraph]: {{ site.baseurl }}/assets/images/first-iteration/profile-flamegraph.svg
[flamegraph-docs]: https://www.brendangregg.com/flamegraphs.html
[substring-ledger-values]: https://github.com/tindandelion/rust-text-compression/blob/0.0.2/src/encoder/substring_ledger.rs#L42
[substring-ledger-longest-match]: https://github.com/tindandelion/rust-text-compression/blob/0.0.2/src/encoder/substring_ledger.rs#L27
[next-step]: {{site.baseurl}}/{% post_url 2025-01-17-tackling-the-performance-bottleneck %}