---
layout: post
title: Widening the encoding table
date: 2025-02-27
---

Until now, I've been using a simplified encoding scheme, which limited the size of the encoding table to 256 entries. It can be made larger, though. In this post, I'm describing how I've done it, and what impact it's made on the compression ratio. 

# Using the full range of 2-byte encoding 

At the moment, the encoding scheme is rather simple. Each substring we've found at the learning phase is replaced with 2-byte value `0xF5NN`, where `NN` is the index of the entry in the encoding table. The value `0xF5` never occurs in the valid UTF-8 string, so it can safely be used as a marker for the replacement. Since I'm only using 1 byte for the index, it limits the size of encoding table to 256 entries.

However, by the [UTF-8 specification][utf-8-spec], all bytes `0xF5`-`0xFF` are invalid in UTF-8. It means I can use the entire range `0xF500..0xFFFF` to represent the substring entries, still using only 2 bytes for encoding. Using this scheme, I can encode up to `0xFFFF - 0xF500 == 2816` substrings! That should improve the compression ratio, because with the wider encoding table I can remove much more repetitions from the source text. 

# The impact of expanding the encoding table 

To implement the advanced encoding scheme, I only had to make small adjustments to the function [encode_string][encode-string-0.0.8], and its counterpart, [decode_string][decode-string-0.0.8], to make use of the full range of `0xF500..0xFFFF`. 

Having done that, I ran the compression on my usual test subject, the [excerpt from "War and peace"][test-file], using the `CaptureAll` policy, and got the following results:  

| Substring Ledger Size | Compression Ratio | Time Elapsed |
|----------------------:|------------------:|-------------:|
|               210 453 |            51.07% |       73.64s |

That's quite an improvement in the compression ratio: 51% versus 25% from my [previous experiment][prev-post]!

# Trying out different ledger limits 

Now, let's have a look at the results of compressing the test file with different substring ledger limits: 

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

The results are quite impressive here as well. Apart from better compression ratios, another difference from the [previous results][prev-post] is that it doesn't peak at ledger size 8192, but still continues to grow, though not so rapidly. The graph indicates that it plateaus around ledger sizes 32768...65536. Unfortunately, the execution time becomes an issue, in trying out even larger ledger sizes. 

# Next steps 

By utilizing the full range of 2-byte encoding, I was able to improve the compression ratio of my algorithm up to 51%, compared to 25% from the previous experiments. The results are published on GitHub under the [tag 0.0.8][tag-0.0.8]. 

It's clear from the data though, that the execution time of the current implementation grows polynomially, as we increase the ledger size. I'm going to work on improving this situation next.

[utf-8-spec]: https://en.wikipedia.org/wiki/UTF-8
[encode-string-0.0.8]: https://github.com/tindandelion/rust-text-compression/blob/0.0.8/src/encoder/encode_string.rs#L13
[decode-string-0.0.8]: https://github.com/tindandelion/rust-text-compression/blob/0.0.8/src/decoder.rs#L5
[test-file]: https://github.com/tindandelion/rust-text-compression/blob/0.0.8/test-data/wap-25600.txt
[prev-post]: {{site.baseurl}}/{% post_url 2025-02-16-experiments-with-ledger-limit %}
[tag-0.0.8]: https://github.com/tindandelion/rust-text-compression/tree/0.0.8substrin