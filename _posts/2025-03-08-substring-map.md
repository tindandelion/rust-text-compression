---
layout: post
title: Optimizing substring map 
date: 2025-03-08
---

# Flamegraph before 

![Flamegraph before optimization]({{ site.baseurl }}/assets/images/optimize-substring-map/flamegraph-before.svg)


# Falemgraph after optimization 

![Flamegraph after optimization]({{ site.baseurl }}/assets/images/optimize-substring-map/flamegraph-after-optimization.svg)

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
