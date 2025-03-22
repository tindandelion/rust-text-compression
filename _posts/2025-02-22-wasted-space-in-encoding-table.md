---
layout: post
title: Wasted space in the encoding table
date: 2025-02-22
---

The results of my [previous experiments][prev-post] puzzled me a bit, so I decided to spend some time analyzing the whole algorithm to see if I can spot any clues that would explain the results. It turned out that there's a chance that we are not using the space in the encoding table as efficiently as we could. Specifically, we may include some substrings into the table, that are never used in the encoding phase. 

In this post, I'm going to explain why that happens and do some more experiments to see if we can improve the situation.

# How we end up with wasted space

Let's take a look at the following example string: 

```
hello world hello world hello world hello world hello world hello hello world world
```

When we run the learning algorithm to find out the common substrings, we end up with the following result:

```
" hello world"
" hello w"
" hel"
" wor"
"lo w"
"orld"
```

Notice the second substring, `" hello w"`. It was learned by the algorithm, but in fact it's useless for the encoding process, because it's only a prefix of `" hello world"`. It was useful in the learning phase, as a stepping stone towards the `" hello world"`, but in the end we don't need it: the encoding algorithm will use `" hello world"` instead, and nowhere else in the text `" hello w"` will be applicable on its own.

My hypothesis is that we end up with quite a few such substrings in the final encoding table, and that's why the compression ratio degrades for longer ledger sizes. I'm going to explore it further.

# Tracking unused encoding table slots 

It's quite easy to track which slots in the encoding table were actually used in the process. With the small addition to the [`EncodingTable`][encoding-table] struct, I collected the data on the wasted space: 


| Max Ledger Size | Used Entries | Compression Ratio |
|----------------:|-------------:|------------------:|
| _CaptureAll_    |     251/256 |          25.36% |
|             256 |      21/23 |             6.89% |
|             512 |      85/86 |            13.20% |
|           1 024 |    228/228 |            20.26% |
|           2 048 |    256/256 |            26.07% |
|           4 096 |    254/256 |            27.16% |
|           8 192 |    255/256 |            27.28% |
|          16 384 |    253/256 |            26.87% |
|          32 768 |    251/256 |            26.20% |
|          65 536 |    250/256 |            26.02% |

Indeed, we can see that there are some unused slots in the encoding table. The number of unused slots isn't too big, but still I believe it can have impact on the overall compression ratio, given that we only have 256 available slots in the table. The number of unused slots correlates well with the drop in the compression ratio.

# Experimenting with a different way to select substrings

Initially, my idea was to pick substrings for the final encoding table by their "compression gain". The compression gain takes into account both substring size and its frequency in the string: 

```
compression_gain = (original_size - encoded_size) * count
```

It looked like a reasonable approach to do, but maybe it's not the best strategy. Both original articles suggest that we select substrings solely based on their occurrence frequency. I'm curious to see how switching to the frequency-based approach would perform, especially when combined with the limiting the dictionary size. My intuitive understanding is that this approach will select shorter substrings with higher frequencies, and that may mitigate the wasted space problem, since short strings are more likely to be used throughout the the text. 

I still would like to keep both approaches in the code, though, because I'd like to have the opportunity to experiment with both strategies in the future. To achieve that, I'm going to extract the substring selection code from [`SubstringLedger`][substring-ledger] struct, and put it into a separate entity called [`SubstringSelector`][substring-selector]. Separating the code makes it easier to work with, and it also looks cleaner in the codebase: 

- [`SubstringLedger`][substring-ledger] is now only responsible for managing the substrings during the learning phase;
- [`SubstringSelector`][substring-selector] is responsible for selecting the appropriate subset of substrings for the final encoding table. It also gives us a choice how to select the substrings: either by their compression gain, or by their frequency.

# The results 
{: #results }

[Having implemented][tag-0.0.7] the necessary changes, I ran my experiments again, only using the frequency-based approach. The results are presented in the table below: 

| Max Ledger Size | Used Entries | Compression Ratio |
|----------------:|-------------|------------------:|
| _CaptureAll_    |     256/256 |          26.46%  |
|             256 |      21/23 |             6.89% |
|             512 |      85/86 |            13.20% |
|           1 024 |    228/228 |            20.26% |
|           2 048 |    256/256 |            25.87% |
|           4 096 |    254/256 |            27.36% |
|           8 192 |    256/256 |            27.07% |
|          16 384 |    255/256 |            27.62% |
|          32 768 |    256/256 |            27.46% |
|          65 536 |    256/256 |            27.51% |

There's definitely some improvement here. It's not super dramatic, and I believe that for a variety of texts the difference between these two approaches will be negligible. 

It is remarkable, though, that the baseline result for [`CaptureAll`][capture-all] strategy shows worse results than the approach with the limited ledger size. However, I find it hard to reason about why that's so. My best guess is that the approach with the limited ledger size is picking up shorter, but more frequent substrings, eventually leading to a slightly better compression ratio.

The **main takeaway** from those experiments is that _it makes no sense to keep all found substrings in memory_. After a certain threshold, it doesn't benefit the compression ratio anymore, so the approach to limit the size of the ledger is the one to go.

# Next steps

My current progress is available on GitHub under the [tag 0.0.7][tag-0.0.7].

It looks to me that the size of the encoding table is a quite important parameter, as soon as eliminating wasted slots has led to improvements in the compression ratio. So the next step in the project will be expanding the encoding table, to fully utilize the benefits of the 2-byte encoding.

[prev-post]: {{site.baseurl}}/{% post_url 2025-02-16-experiments-with-ledger-limit %}
[encoding-table]: https://github.com/tindandelion/rust-text-compression/blob/0.0.7/src/encoding_table.rs
[substring-ledger]: https://github.com/tindandelion/rust-text-compression/blob/0.0.7/src/encoder/substring_ledger.rs
[substring-selector]: https://github.com/tindandelion/rust-text-compression/blob/0.0.7/src/encoder/substring_selector.rs
[capture-all]: https://github.com/tindandelion/rust-text-compression/blob/0.0.7/src/encoder/ledger_policies.rs#L6
[tag-0.0.7]: https://github.com/tindandelion/rust-text-compression/tree/0.0.7






