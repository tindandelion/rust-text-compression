---
layout: post
title: Experimenting with substring ledger limits
date: 2025-02-16
---

Having introduced the substring ledger limit, I can now experiment with different values, to see how it affects the compression ratio and the time performance of the compression algorithm. My expectation is that the time performance will be better, because smaller ledger size means that we'll need to sieve through fewer substrings to find the longest match. With compression ratio, it's not so clear yet how it's going to be affected. 

# Establishing the baseline

Let's start by establishing the baseline. I'm going to use an excerpt from the novel "War and Peace" by Leo Tolstoy. The full text is available [here][tolstoy-gutenberg]. To make a [test dataset][tolstoy-excerpt], I'm taking the first 25600 lines of the text. I can't use the entire text yet, because the algorithm is still too slow to compress it in all entirety. 

Also, for time measurements, I'm switching to using the _release_ build. Release builds use more aggressive optimizations, and remove some runtime checks. As a result, the release build runs significantly faster, which allows me to use larger input dataset.

For the experiments with different ledger limits, I've set up one more binary crate, called [`ledger-size-test`][ledger-size-test-0.0.6]. This executable first runs the compression of the input text using the `CaptureAll` ledger policy. This policy doesn't put a limit on the ledger size during construction: it captures all substrings that repeated at least once, and lets the ledger grow as much as needed. 

Here's the result of running the compression: 

| Substring Ledger Size | Compression Ratio | Time Elapsed |
|----------------------:|------------------:|-------------:|
|               210 453 |            25.36% |       67.02s |

And the top 10 most impactful substrings are: 

```
"EN: 1812\n\n    CHAPTER I\n\n    CHAPTER II\n\n    CHAPTER III\n\n    CHAPTER IV\n\n    CHAPTER V\n\n    CHAPTER VI\n\n    CHAPTER VII\n\n    CHAPTER VIII\n\n    CHAPTER IX\n\n    CHAPTER X\n\n    CHAPTER XI\n\n    CHAPTER XII\n\n    CHAPTER XIII\n\n    CHAPTER XIV\n\n    CHAPTER XV\n\n    CHAPTER XVI\n\n    CHAPTER X"
"\n\n    CHAPTER I\n\n    CHAPTER II\n\n    CHAPTER III\n\n    CHAPTER IV\n\n    CHAPTER V\n\n    CHAPTER VI\n\n    CHAPTER VII\n\n    CHAPTER VIII\n\n    CHAPTER IX\n\n    CHAPTER X\n\n    CHAPTER XI\n\n    CHAPTER XII\n\n    CHAPTER XIII\n\n    CHAPTER XIV\n\n    CHAPTER X"
"EN: 1812\n\n    CHAPTER I\n\n    CHAPTER II\n\n    CHAPTER III\n\n    CHAPTER IV\n\n    CHAPTER V\n\n    CHAPTER VI\n\n    CHAPTER VII\n\n    CHAPTER VIII\n\n    CHAPTER IX\n\n    CHAPTER X\n\n    CHAPTER XI\n\n    CHAPTER XII\n\n    CHAPTER X"
"2\n\n    CHAPTER I\n\n    CHAPTER II\n\n    CHAPTER III\n\n    CHAPTER IV\n\n    CHAPTER V\n\n    CHAPTER VI\n\n    CHAPTER VII\n\n    CHAPTER VIII\n\n    CHAPTER IX\n\n    CHAPTER X\n\n    CHAPTER XI\n\n    CHAPTER XII\n\n    CHAPTER X"
"\n\n    CHAPTER II\n\n    CHAPTER III\n\n    CHAPTER IV\n\n    CHAPTER V\n\n    CHAPTER VI\n\n    CHAPTER VII\n\n    CHAPTER VIII\n\n    CHAPTER IX\n\n    CHAPTER X\n\n    CHAPTER XI\n\n    CHAPTER XII\n\n    CHAPTER X"
"III\n\n    CHAPTER XIV\n\n    CHAPTER XV\n\n    CHAPTER XVI\n\n    CHAPTER XVII\n\n    CHAPTER XVIII\n\n    CHAPTER XIX\n\n    CHAPTER XX\n\n    CHAPTER XXI\n\n    CHAPTER XXII\n\n    CHAPTER X"
"\n\n    CHAPTER I\n\n    CHAPTER II\n\n    CHAPTER III\n\n    CHAPTER IV\n\n    CHAPTER V\n\n    CHAPTER VI\n\n    CHAPTER VII\n\n    CHAPTER VIII\n\n    CHAPTER IX\n\n    CHAPTER X"
"I\n\n    CHAPTER VIII\n\n    CHAPTER IX\n\n    CHAPTER X\n\n    CHAPTER XI\n\n    CHAPTER XII\n\n    CHAPTER XIII\n\n    CHAPTER XIV\n\n    CHAPTER X"
"III\n\n    CHAPTER XIV\n\n    CHAPTER XV\n\n    CHAPTER XVI\n\n    CHAPTER XVII\n\n    CHAPTER XVIII\n\n    CHAPTER XIX\n\n    CHAPTER X"
"VI\n\n    CHAPTER VII\n\n    CHAPTER VIII\n\n    CHAPTER IX\n\n    CHAPTER X\n\n    CHAPTER XI\n\n    CHAPTER XII\n\n    CHAPTER X"
```

# Experiments with ledger limits 

Next, I'm experimenting with setting the different ledger limits on the same input text. I'm running the compression with different ledger limits, and measuring the compression ratio and the time elapsed. 

The results here are very curious and somewhat unexpected: 

| Max Ledger Size | Learned Ledger Size | Compression Ratio | Time Elapsed |
|----------------:|--------------------:|------------------:|-------------:|
|             256 |                 171 |             6.89% |        0.86s |
|             512 |                 405 |            13.20% |        1.22s |
|           1 024 |                 631 |            20.26% |        1.82s |
|           2 048 |               2 047 |            26.07% |        2.73s |
|           4 096 |               4 093 |            27.16% |        4.69s |
|           8 192 |               8 179 |            27.28% |        7.58s |
|          16 384 |              16 337 |            26.87% |       12.14s |
|          32 768 |              32 609 |            26.20% |       20.18s |
|          65 536 |              64 899 |            26.02% |       31.64s |

![Compression ratio and Time elapsed, by ledger limit]({{ site.baseurl }}/assets/images/experiments-with-ledger-limits/comp-ratio-time-elapsed-by-limit.svg)

Something very curious is going on here. The time grows polynomially as I increase the ledger limit, which is consistent with the previous observations. However, the compression ratio shows a different pattern. It first grows, but then hits a plateau and even starts to decrease slightly, as the ledger size grows! This is not what I expected to see, and I don't fully understand it yet. At ledger sizes after 2048, it's actually better than the baseline result, when we were capturing all substrings. 

Here are the top 10 substrings for the ledger limit of 8192, the winner of the compression ratio contest: 

```
"2\n\n    CHAPTER I\n\n    CHAPTER II\n\n    CHAPTER III\n\n    CHAPTER IV\n\n    CHAPTER V\n\n    CHAPTER VI\n\n    CHAPTER VII\n\n    CHAPTER VIII\n\n    CHAPTER IX\n\n    CHAPTER X\n\n    CHAPTER XI\n\n    CHAPTER XII\n\n    CHAPTER XIII\n\n    CHAPTER XIV\n\n    CHAPTER XV\n\n    CHAPTER XV"
"\n\n    CHAPTER I\n\n    CHAPTER II\n\n    CHAPTER III\n\n    CHAPTER IV\n\n    CHAPTER V\n\n    CHAPTER VI\n\n    CHAPTER VII\n\n    CHAPTER VIII\n\n    CHAPTER IX\n\n    CHAPTER X\n\n    CHAPTER XI\n\n    CHAPTER XII\n\n    CHAPTER XIII\n\n    CHAPTER X"
"II\n\n    CHAPTER III\n\n    CHAPTER IV\n\n    CHAPTER V\n\n    CHAPTER VI\n\n    CHAPTER VII\n\n    CHAPTER VIII\n\n    CHAPTER IX\n\n    CHAPTER X\n\n    CHAPTER X"
"I\n\n    CHAPTER XII\n\n    CHAPTER XIII\n\n    CHAPTER XIV\n\n    CHAPTER XV\n\n    CHAPTER XVI\n\n    CHAPTER XVII\n\n    CHAPTER XVIII\n\n    CHAPTER XIX"
"," said Prince A"
"," said the "
"Prince Andrew "
"Prince Vasíli"
"Prince Andre"
" Prince And"
```

Compared to the the baseline, we can see that shorter substrings start to appear in the top 10. By all accounts, they look legitimate: the names of the characters are expected to be repeated often in the text. The first few, though, are surprising. I didn't expect them to survive pruning, because they don't look like they repeat that often throughout the text. 

# Reflecting on the results 

To be honest, that's quite puzzling. I expected that if I don't limit the ledger size, I'd get the best possible compression result, but that doesn't seem to be the case, and I wonder why that's so. 

Let's consider the algorithm one more time. What we do is the following: 

1. We scan the input text, searching for repeated substrings: 
   - The new compound substring is added to the ledger when the counts of the components are above the threshold; 
   - We prune the ledger when it reaches the size limit, removing all substrings with counts below the median value. 
2. Once all text is scanned, we sort the ledger by the impact each substring has on the compression. 
3. From sorted result, we take the first top 256 substrings, and use them to compress the text. 

The questions I'm getting are: 

- _Is there a bug somewhere in the implementation?_ Specifically, why do I get the long substrings in the top 10, that don't look like they appear that often in the text? 
- _Will I get better results if I include more substrings into the resulting dictionary?_ At the moment, I'm taking only 256 substrings, but we can take up to 2816, still using 2 bytes for encoding. If I use 3 byte encoding, I can use a much bigger substring dictionary. Will that improve the result? 
- _Am I correctly selecting top substrings that go to the resulting dictionary?_ Would it be better if I selected the substrings merely by their frequency, instead of the impact they have on the compression? Am I assessing the compression impact correctly? 

# Next steps

The current state of the project is available on GitHub under the [tag 0.0.6][tag-0.0.6].
In the next steps, I'm going to tackle the questions I listed above. 

[tolstoy-excerpt]: https://github.com/tindandelion/rust-text-compression/blob/0.0.6/test-data/wap-25600.txt
[tolstoy-gutenberg]: https://www.gutenberg.org/cache/epub/2600/pg2600.txt
[ledger-size-test-0.0.6]: https://github.com/tindandelion/rust-text-compression/blob/0.0.6/src/bin/ledger-size-test.rs
[tag-0.0.6]: https://github.com/tindandelion/rust-text-compression/tree/0.0.6