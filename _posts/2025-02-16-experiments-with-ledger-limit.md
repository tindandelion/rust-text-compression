---
layout: post
title: Experimenting with substring ledger limits
date: 2025-02-16
---

Having introduced the substring ledger limit, I can now experiment with different values of the limit, to see how it affects the compression ratio and the time performance of the compression algorithm. My expectation is that the time performance will be better, because smaller ledger size means that we'll need to sieve through fewer substrings to find the longest match. With compression ratio, it's not so clear yet how it's going to be affected. 

# Establishing the baseline

Let's start by establishing the baseline. I'm going to use an excerpt from the novel "War and Peace" by Leo Tolstoy. The full text is available [here][tolstoy-gutenberg]. To make a test dataset, I'm taking the first 25600 lines of the text. I can't use the entire text yet, because the algorithm is too slow to compress it in all entirety. 

Also, in time measurements, I'm switching to using the _release_ build. Release builds use more aggressive optimizations, and remove some runtime checks. As a result, the release build runs significantly faster, which allows me to use larger input dataset.

So, to establish the baseline, I've run the compression of the input text using the `CaptureAll` ledger policy. This policy doesn't put a limit on the ledger size during construction: it captures all substrings that repeated at least once, and lets the ledger grow as much as needed. 

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

The questions I'm getting: 
- Is it a specific aberration in the input text? 
- Would the result be different if I selected the strings simply by their frequency? 

[tolstoy-gutenberg]: https://www.gutenberg.org/cache/epub/2600/pg2600.txt