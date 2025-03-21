---
layout: post
title: Optimizing the encoding process
date: 2025-03-19
---

In the [previous post][prev-post] I implemented a *trie* data structure for fast substring lookups during the learning phase of the compression algorithm. I also mentioned that there was one more place where tries could improve the performance: the encoding phase. It didn't bother me much then, but now I've decided to bite the bullet and eliminate this inefficiency. 

# The piece to optimize 

When we encode the input text, we scan through it and try to find the longest substring that matches the current beginning of the text. This job is done by [`EncodingTable::find_match()`][find-match-0.0.9] method. It's quite obvious that the current implementation isn't very efficient: to find a matching substring, we perform a linear search in the table of substrings. It's not a huge problem, because the encoding table is limited in size, but still we could do a better job here.  

We already successfully tackled the same problem for `SubstringLedger`, by using tries and their efficient lookup algorithm. It's quite natural to apply the same method for `EncodingTable` as well. 

# Extracting `Trie` data structure 

By now, the trie data structure is implemented inside the [`TrieSubstringCounts`][trie-substring-counts-0.0.9] struct. It's going to be rather awkward to reuse that struct elsewhere, so as a first step I decided to extract a more generic [`Trie`][trie-0.0.10] struct, and move most of the implementation there. The [new version of `TrieSubstringCounts`][trie-substring-counts-0.0.10] has now become a simple wrapper around the [`Trie`][trie-0.0.10] struct. 

# Use of trie in `EncodingTable`

The [`EncodingTable`][encoding-table-0.0.10] struct is essentially a two-way mapping of substrings to their respective indices: 

* during encoding, we try to find an index of the longest substring that matches the beginning of the input text; 
* when decoding, we perform a reverse operation, by looking up a relevant substring by its index from the encoded data. 

To make it work efficiently in both situations, we have to maintain two different data structures internally. One is a simple `Vec` of substrings, to quickly look up the substring by its index. Another data structure is a trie that provides the mapping in the opposite direction: from substrings to their respective indices. This structure comes into play when we search for the longest substring at the beginning of the input text. 

We build both structures when we create the instance of `EncodingTable`. Luckily, their content doesn't change, so there's no additional effort to keep them both in sync.

# Comparing implementations: final experiment

To round up all the work I've done so far with optimizing the compression algorithm speed, let's do one final experiment. 
I'm going to run the compression on input texts of different lengths, and see how our implementation with tries beats the b-tree performance. 

As before, I'm using [the excerpts][test-data] of *War and Peace* novel:  

| File name              | Source length (chars) |   B-tree |  Tries |
|----------------------|---------------------:|--------:|-------:|
| wap-1600.txt         |              45 832 |    0.18 |   0.02 |
| wap-3200.txt         |             119 763 |    0.88 |   0.03 |
| wap-6400.txt         |             276 154 |    3.72 |   0.11 |
| wap-12800.txt        |             589 815 |   12.84 |   0.16 |
| wap-25600.txt        |           1 229 811 |   35.61 |   0.25 |
| *war-and-peace.txt*  |           *3 293 615* |  *114.17* |   *0.52* |
| war-and-peace-dbl.txt|           6 587 230 |  227.45 |   0.93 |
| war-and-peace-quad.txt|         13 174 460 |  454.30 |   1.73 |

![Compression time, by source length]({{ site.baseurl }}/assets/images/optimize-encoding/source-length-vs-time-comparison.svg)

The difference in speed is astonishing! It's almost useless to put that data on the same plot: you can barely see the plot line for tries implementation in the picture. 

To marvel on the numbers one more time: it takes **almost 2 minutes** for the b-tree implementation to process the entire text of *War And Peace* novel. Implemented with tries, it takes **less than 1 second**! I couldn't be more satisfied with the payoff for the efforts spent on optimizing the algorithm.

# Next steps 

Current version of the program is available on GitHub under the [tag 0.0.10][tag-0.0.10].

While I was experimenting with different optimizations for the core algorithm, I introduced quite a bit of complexity to the code, trying to support different implementations, collecting additional data, etc. Now that I'm done with those experiments, it's time to revisit the code and do some housekeeping, removing the pieces that are no longer needed.  

[prev-post]: {{site.baseurl}}/{% post_url 2025-03-08-substring-map %}
[find-match-0.0.9]: https://github.com/tindandelion/rust-text-compression/blob/0.0.9/src/encoding_table.rs#L26
[trie-substring-counts-0.0.9]: https://github.com/tindandelion/rust-text-compression/blob/0.0.9/src/encoder/substring_counts/tries.rs
[trie-0.0.10]: https://github.com/tindandelion/rust-text-compression/blob/0.0.10/src/trie.rs
[trie-substring-counts-0.0.10]: https://github.com/tindandelion/rust-text-compression/blob/0.0.10/src/encoder/substring_counts/tries.rs
[encoding-table-0.0.10]: https://github.com/tindandelion/rust-text-compression/blob/0.0.10/src/encoding_table.rs
[test-data]: https://github.com/tindandelion/rust-text-compression/tree/0.0.10/test-data
[tag-0.0.10]: https://github.com/tindandelion/rust-text-compression/tree/0.0.10