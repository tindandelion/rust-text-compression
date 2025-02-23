---
layout: post
title: Expanding the encoding table
date: 2025-02-23
---

Baseline result: 

| Substring Ledger Size | Compression Ratio | Time Elapsed |
|----------------------:|------------------:|-------------:|
|               210 453 |            51.07% |       96.85s |

Experimenting with different ledger limits: 

| Max Ledger Size | Learned Ledger Size | Compression Ratio | Time Elapsed |
| --------------: | ------------------: | ----------------: | -----------: |
|             256 |                 171 |             6.89% |        1.24s |
|             512 |                 405 |            13.20% |        1.76s |
|          1 024  |                 631 |            20.26% |        2.63s |
|          2 048  |               2 047 |            34.76% |        4.58s |
|          4 096  |               4 093 |            41.89% |        8.92s |
|          8 192  |               8 179 |            46.80% |       12.50s |
|         16 384  |              16 337 |            49.65% |       19.19s |
|         32 768  |              32 609 |            50.89% |       30.58s |
|         65 536  |              64 899 |            51.34% |       44.65s |

![Compression ratio and Time elapsed, by ledger limit]({{ site.baseurl }}/assets/images/expanding-encoding-table/comp-ratio-time-elapsed-by-limit.svg)
