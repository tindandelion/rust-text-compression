---
layout: post
title: Expanding the encoding table
date: 2025-02-23
---

Until now, I've been using a simplified encoding scheme, which limited the size of the encoding table to 256 entries. It can be made larger, though. In this post, I'm describing how I've done it, and share the impact on the compression ratio. 

# Using the full range of 2-byte encoding 

At the moment, the encoding scheme is rather simple. Each substring we find during the encoding process is replaced with 2 bytes `0xF5 NN`, where `NN` is the index of the entry in the encoding table. I use the fact that value `0xF5` never occurs in the valid UTF-8 string, so it can safely be used as a marker for the replaced substring. Since I'm only using 1 byte for the index, it limits the size of encoding table to 256 entries.

However, by the [UTF-8 specification][utf-8-spec], all bytes `0xF5`-`0xFF` are invalid in UTF-8. It means I can use the entire range `0xF500..0xFFFF` to represent the substring entries, still using 2 bytes for encoding. Using this scheme, I can represent `0xFFFF - 0xF500 == 2816` substrings! That should improve the compression ratio, because now I can remove much more repetitions from the source text. 

# The impact of expanding the encoding table 

To implement the advanced encoding scheme, I only had to make small adjustments to the function [encode-string][encode-string-0.0.8], and its counterpart, [decode][decode-0.0.8]. 

Having done that, I ran the compression on my usual test subject, the [excerpt from "War and peace"][test-file], using the `CaptureAll` policy, and got the following results:  

| Substring Ledger Size | Compression Ratio | Time Elapsed |
|----------------------:|------------------:|-------------:|
|               210 453 |            51.07% |       73.64s |

That's quite an improvement in the compression ratio: 51% over 25% from my [previous experiment][prev-post]!



Experimenting with different ledger limits: 

| Max Ledger Size | Learned Ledger Size | Compression Ratio | Time Elapsed |
| --------------: | ------------------: | ----------------: | -----------: |
|             256 |                 171 |             6.89% |        0.94s |
|             512 |                 405 |            13.20% |        1.33s |
|          1 024  |                 631 |            20.26% |        2.00s |
|          2 048  |               2 047 |            34.76% |        3.48s |
|          4 096  |               4 093 |            41.89% |        6.76s |
|          8 192  |               8 179 |            46.80% |        9.43s |
|         16 384  |              16 337 |            49.65% |       14.45s |
|         32 768  |              32 609 |            50.89% |       23.14s |
|         65 536  |              64 899 |            51.34% |       35.52s |

![Compression ratio and Time elapsed, by ledger limit]({{ site.baseurl }}/assets/images/expanding-encoding-table/comp-ratio-time-elapsed-by-limit.svg)
