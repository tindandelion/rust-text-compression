---
layout: post
title: Optimizing substring map 
date: 2025-03-08
---

So far, my primary goal was to optimize the compression algorithm for the best compression ratio. In the [last post][prev-post], I've arrived at the solution that gives quite satisfactory results. Now Id' like to shift the focus and address another elephant in the room: the time efficiency of the compression algorithm. Right now, it's quite slow on large amounts of input data. I'd like to tackle this issue and speed up the algorithm by introducing data structures that perform substring search in a more efficient manner. 

I already tackled the time performance issue once in one of the [previous posts][performance-bottleneck]. At that time, I was able to improve the performance of the substring learning stage by switching from `HashMap` to `BTreeMap`. However, despite this effort, we still saw the polynomial growth in therms of execution time. We see this effect if we [increase the size of the input text][performance-bottleneck-results], but also the same effect was seen when we [played with different ledger sizes][ledger-sizes-experiment-results] on fixed input data. 

Polynomial time growth is a performance killer.

# Identifying the bottleneck

Let's again discover where the algorithm spends most of the time, using our familiar tool, [the flamegraph][flamegraphs]. This time, I'm going to profile the _release_ version of the application. Compared to the debug version, when Rust compiles the release version of a program, it applies some performance optimizations and removes runtime checks. As a result, the release build runs significantly faster. 

However, the release build also doesn't include the debug information, which makes flamegraphs much less informative, since it doesn't show meaningful function names anymore. To avoid that, we can instruct Cargo to include the debug symbols into the release build, by adding the following configuration option to `.cargo/config.toml` file: 

```toml
[profile.release]
debug = true
```

[![Flamegraph before optimization][flamegraph-before]][flamegraph-before]


# Flamegraph after optimization 

[![Flamegraph after optimization][flamegraph-after]][flamegraph-after]

# Current results 

| Max Ledger Size | Learned Ledger Size | Compression Ratio | Time Elapsed |
| --------------: | ------------------: | ----------------: | -----------: |
|             256 |                 223 |             6.75% |        0.44s |
|             512 |                 490 |            13.91% |        0.61s |
|           1 024 |                 873 |            20.75% |        0.80s |
|           2 048 |               2 047 |            33.71% |        1.01s |
|           4 096 |               4 094 |            41.53% |        1.98s |
|           8 192 |               8 179 |            46.40% |        1.62s |
|          16 384 |              16 339 |            49.40% |        1.47s |
|          32 768 |              32 611 |            50.53% |        1.49s |
|          65 536 |              64 887 |            51.06% |        1.57s |
|         131 072 |             117 596 |            51.19% |        1.66s |
|         262 144 |             145 888 |            51.18% |        1.73s |

![Compression ratio and Time elapsed, by ledger limit]({{ site.baseurl }}/assets/images/optimize-substring-map/comp-ratio-time-elapsed-by-limit.svg)

[prev-post]: {{site.baseurl}}/{% post_url 2025-02-27-expand-encoding-table %}
[performance-bottleneck]: {{site.baseurl}}/{% post_url 2025-01-17-tackling-the-performance-bottleneck %}
[performance-bottleneck-results]: {{site.baseurl}}/{% post_url 2025-01-17-tackling-the-performance-bottleneck %}#results
[ledger-sizes-experiment-results]: {{site.baseurl}}/{% post_url 2025-02-27-expand-encoding-table %}#results
[flamegraphs]: {{site.baseurl}}/{% post_url 2025-01-12-profiling-with-flamegraphs %}
[flamegraph-before]: {{ site.baseurl }}/assets/images/optimize-substring-map/flamegraph-before.svg
[flamegraph-after]: {{ site.baseurl }}/assets/images/optimize-substring-map/flamegraph-after-optimization.svg