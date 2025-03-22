---
layout: post
title: Review and cleanup 
date: 2025-03-22
---

I'm close to finishing up the implementation of the text compression algorithm. Along the way, I did quite a few experiments with different parts of the program. To keep things open for experimentation, I introduced a few abstractions in the code. These abstractions were useful while I was still tweaking the algorithm, but the downside was that the code became more complicated than needed. Once I'm done with the experiments and selected the best solutions, I no longer need to keep these abstractions in the code, or at least I don't need to expose them to the outside world.

In this post, I'm going to review the work I've done so far, and see what parts of the program can be removed from the final version. 

# Working with substrings 

My [most recent task][substring-map-experiment] was to come up with an efficient data structure to manage the list of substrings and their counts, while we are learning the common substrings from the source text. 

At the very beginning of the project, I [started with a simple `HashMap`][hash-map-impl], but shortly after [switched to using `BTreeMap`][btree-map-impl] to avoid unnecessary key sorting. In its turn, `BTreeMap` also proved to be an inefficient for the task, so I [ended up implementing a trie data structure][substring-map-experiment]. Using the trie gave a huge performance boost to the compression algorithm, thanks to its efficient method for the substring search. 

For experimentation, I created the [`SubstringCounts`][substring-counts-0.0.10] trait, and two different implementations: [`BTreeSubstringCounts`][btree-substring-counts-0.0.10], and [`TrieSubstringCounts`][trie-substring-counts-0.0.10]. The latter [proved to be a winner][btree-trie-comparison], and I don't need the b-tree implementation anymore, so it's the first thing to let go. Then, since I only end up with a single implementation for `SubstringCounts`, I don't think this abstraction is worth keeping either: it doesn't seem to give me any benefits. 

That became my first clean-up task. I removed the unused `BTreeSubstringCounts` implementation, and merged `SubstringCounts` trait with `TrieSubstringCounts`. I liked the name `SubstringCounts`, so I re-used it for the [final struct][substring-counts-0.1.0]. 

# Selecting substrings for encoding 

Another [subject for experiments][encoding-table-space] was to determine which substrings make their way to the final encoding table. I had two options to choose from: 

* select the most frequently occurring substrings, looking solely on their counts; 
* select the substrings by their overall *compression gain*, that would take into account both frequency and the substring lengths. 

As it turned out, it didn't make much of a difference which approach to choose. Frequency-based selection [showed to be marginally better][encoding-table-space-result], and also simpler to reason about, so I decided to keep that approach as a final version. That led to the simplifications on the side of the [`SubstringSelector`][substring-selector-0.1.0] struct, since I didn't need to maintain two different ways for substring selection.

I also played with the idea of getting rid of `SubstringSelector` altogether and moving its responsibilities to the [`SubstringLedger`][substring-ledger-0.1.0] struct, but eventually decided against it. I like to keep that logic separate from the rest of the code: it allows me to test it more easily in isolation.

# Substring ledger book-keeping 

Yet another area where I spent quite a bit of time and effort was [limiting the substring ledger size][limit-ledger-size]. As I usually do, I [started with a simple approach][first-iteration], where I kept all repeated substrings. Later I [applied a strategy][limit-ledger-size] to keep the ledger size limited. The strategy was to check the condition when a new substring is added to the ledger, and also to remove rare substrings when we needed to free up the space for new ones. 

I [experimented with different ledger sizes][ledger-size-experiments] and found that the the limit on the ledger size has a significant impact on the overall compression ratio. However, after a certain threshold, it didn't make much difference how many substrings we kept. 

The main takeaway from these experiments is that it makes sense to limit the ledger size to *65 536* entries, since bigger numbers don't impact the compression ratio much. I took this strategy into use in the final version of the main [`encode`][encode-0.1.0] function. 

As for further simplifications, it proved to be useful to be able to apply different ledger policies, particularly for testing. By keeping a level of flexibility here, I can apply different test versions of ledger policies, to test the [learning algorithm][build-ledger-0.1.0] with those simpler implementations: a technique called [test doubles][test-doubles]. 

However, I no longer expose the ledger policies to the outside world: all the details are hidden inside the [`encoder`][encoder-0.1.0] module. The only thing that's exported from this module is the [`encode()`][encode-0.1.0] function, that takes care of all the pesky details. 

# Next steps 

It looks like the core functionality of the text compression algorithm is ready and polished. To celebrate achieving this important milestone, I'm tagging the current state as [version 0.1.0][tag-0.1.0].  

One other thing I'd like to focus on next is to make `encode()` and `decode()` functions more robust, in terms of handling invalid inputs. That leads me to a yet unexplored territory: **error handling in Rust**. 


[substring-map-experiment]: {{site.baseurl}}/{% post_url 2025-03-08-substring-map %}
[hash-map-impl]: {{site.baseurl}}/{% post_url 2025-01-10-first-iteration %}
[btree-map-impl]: {{site.baseurl}}/{% post_url 2025-01-17-tackling-the-performance-bottleneck %}
[substring-counts-0.0.10]: https://github.com/tindandelion/rust-text-compression/blob/0.0.10/src/encoder/substring_counts.rs#L9
[btree-substring-counts-0.0.10]: https://github.com/tindandelion/rust-text-compression/blob/0.0.10/src/encoder/substring_counts/btree.rs
[trie-substring-counts-0.0.10]: https://github.com/tindandelion/rust-text-compression/blob/0.0.10/src/encoder/substring_counts/tries.rs
[btree-trie-comparison]: {{site.baseurl}}/{% post_url 2025-03-19-optimize-encoding %}#results
[substring-counts-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder/substring_counts.rs
[encoding-table-space]: {{site.baseurl}}/{% post_url 2025-02-22-wasted-space-in-encoding-table %}
[encoding-table-space-result]: {{site.baseurl}}/{% post_url 2025-02-22-wasted-space-in-encoding-table %}#results
[substring-selector-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder/substring_selector.rs
[substring-ledger-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder/substring_ledger.rs
[first-iteration]: {{site.baseurl}}/{% post_url 2025-01-10-first-iteration %}
[limit-ledger-size]: {{site.baseurl}}/{% post_url 2025-02-12-limit-ledger-size %}
[ledger-size-experiments]: {{site.baseurl}}/{% post_url 2025-03-08-substring-map %}#results
[encode-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder.rs#L20
[build-ledger-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder/build_ledger.rs#L103
[test-doubles]: https://martinfowler.com/bliki/TestDouble.html
[encoder-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder.rs
[tag-0.1.0]: https://github.com/tindandelion/rust-text-compression/tree/0.1.0