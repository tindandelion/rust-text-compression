---
layout: post
title: Optimizing the encoding process
date: 2025-03-19
---

In the [previous post][prev-post] I implemented a trie data structure for fast substring lookups during the learning phase of the compression algorithm. I also mentioned that there was one more place where tries could improve the performance: the encoding phase. It didn't bother me much then, but now I decided to bite the bullet and eliminate this inefficiency. 

# The problem 

When we encode the input text, we scan through it and try to find the longest substring, matching the current beginning of the text. This job is done by [`EncodingTable::find_match()`][find-match-0.0.9] method. It's quite obvious that the current implementation isn't very efficient: to find a matching substring, we perform a linear search in the table of substrings. We already successfully tackled the same problem for `SubstringLedger`, by using tries and their efficient lookup algorithm. It's quite natural to apply the same method here. 

# Extracting `Trie` data structure 

By now, the trie data structure is implemented inside the [`TrieSubstringCounts`][trie-substring-counts-0.0.9] struct. It's going to be quite awkward to reuse that struct elsewhere, as a first step I decided to extract a more generic [`Trie`][trie-0.0.10] struct, and move most of the implementation there. The [new version of `TrieSubstringCounts`][trie-substring-counts-0.0.10] now becomes a simple wrapper around the `Trie` struct. 

# Use of trie in `EncodingTable`





![Compression time, by source length]({{ site.baseurl }}/assets/images/optimize-encoding/source-length-vs-time-comparison.svg)

[prev-post]: {{site.baseurl}}/{% post_url 2025-03-08-substring-map %}
[find-match-0.0.9]: https://github.com/tindandelion/rust-text-compression/blob/0.0.9/src/encoding_table.rs#L26
[trie-substring-counts-0.0.9]: https://github.com/tindandelion/rust-text-compression/blob/0.0.9/src/encoder/substring_counts/tries.rs
[trie-0.0.10]: https://github.com/tindandelion/rust-text-compression/blob/0.0.10/src/trie.rs
[trie-substring-counts-0.0.10]: https://github.com/tindandelion/rust-text-compression/blob/0.0.10/src/encoder/substring_counts/tries.rs
