---
layout: post
title: Review and cleanup 
date: 2025-03-22
---

I'm close to finishing up the implementation of the text compression algorithm. Along the way, I did quite a few experiments with different parts of the program. To keep things open for experimentation, I introduced a few abstractions. These abstractions were useful while I was still tweaking the algorithm, but the downside was that the implementation became more complicated than needed. Once I'm done with the experiments and selected the best solutions, I no longer need to keep these abstractions in the code, or at least I don't need to expose them to the outside world. 

In this post, I'm going to review the work I've done so far, and see what parts of the program are going to be removed from the final version. 

# Working with substrings 

My [most recent task][substring-map-experiment] was to come up with an efficient data structure to manage the list of substrings and their counts, while we are learning the repeated substrings from the source text. 

At the beginning, I [started with a simple `HashMap`][hash-map-impl], but shortly after [switched to using `BTreeMap`][btree-map-impl] to avoid unnecessary key sorting. In the end, `BTreeMap` also proved to be an inefficient data structure for the task, so I [ended up using a trie data structure][substring-map-experiment]. Using the trie gave a huge performance boost to the compression algorithm, thanks to its efficient method for the substring search. 

For experimentation, I created the [`SubstringCounts`][substring-counts-0.0.10] trait, and two different implementations: [`BTreeSubstringCounts`][btree-substring-counts-0.0.10], and [`TrieSubstringCounts`][trie-substring-counts-0.0.10]. The latter [proved to be a winner][btree-trie-comparison], and I don't need the b-tree implementation anymore, so it's the first thing to be removed. But now, since I only end up with a single implementation for `SubstringCounts`, I don't think this abstraction is worth keeping either: it doesn't seem to give me any benefits. 

That became my first clean-up task. I removed the unused `BTreeSubstringCounts` implementation, and merged `SubstringCounts` trait with `TrieSubstringCounts`. I liked the name `SubstringCounts`, so I re-used it for the [final struct][substring-counts-0.1.0]. 

# Selecting substrings for encoding 

Another [subject for experiments][encoding-table-space] was to determine which substrings end up in the final encoding table. I had two options to choose from: 

* select the most frequently occurring substrings, looking solely on their counts; 
* select the substrings by their overall *compression gain*, that would take into account both frequency and the substring lengths. 

As it turned out, it didn't make much of a difference which approach to choose. Frequency-based selection [showed to be marginally better][encoding-table-space-result], and also simpler to reason about, so I decided to keep that approach. That led to the simplifications on the side of the [`SubstringSelector`][substring-selector-0.1.0] struct, since I don't need to maintain two different ways for substring selection.

I also played with the idea of getting rid of `SubstringSelector` altogether and moving its responsibilities to the [`SubstringLedger`][substring-ledger-0.1.0] struct, but eventually decided against it. I like to keep that logic separate from the rest of the code: it allows me to test it more easily in isolation.


[substring-map-experiment]: {{site.baseurl}}/{% post_url 2025-03-08-substring-map %}
[hash-map-impl]: {{site.baseurl}}/{% post_url 2025-01-10-first-iteration %}
[btree-map-impl]: {{site.baseurl}}/{% post_url 2025-01-17-tackling-the-performance-bottleneck %}
[substring-counts-0.0.10]: https://github.com/tindandelion/rust-text-compression/blob/0.0.10/src/encoder/substring_counts.rs#L9
[btree-substring-counts-0.0.10]: https://github.com/tindandelion/rust-text-compression/blob/0.0.10/src/encoder/substring_counts/btree.rs
[trie-substring-counts-0.0.10]: https://github.com/tindandelion/rust-text-compression/blob/0.0.10/src/encoder/substring_counts/tries.rs
[btree-trie-comparison]: {{site.baseurl}}/{% post_url 2025-03-19-optimize-encoding %}
[substring-counts-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder/substring_counts.rs
[encoding-table-space]: {{site.baseurl}}/{% post_url 2025-02-22-wasted-space-in-encoding-table %}
[encoding-table-space-result]: {{site.baseurl}}/{% post_url 2025-02-22-wasted-space-in-encoding-table %}#results
[substring-selector-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder/substring_selector.rs
[substring-ledger-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder/substring_ledger.rs



