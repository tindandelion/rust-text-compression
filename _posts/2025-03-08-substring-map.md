---
layout: post
title: More efficient substring search
date: 2025-03-08
---

So far, my primary goal was to optimize the compression algorithm for the best compression ratio. In the [last post][prev-post], I arrived at the solution that gives quite satisfactory results. Now I'd like to shift the focus and address another elephant in the room: **the time efficiency** of the compression algorithm. Right now, it's quite slow on long input texts. I'd like to tackle this issue and speed up the algorithm by introducing a data structure that executes substring search more efficiently. 

I already tackled the time performance issue once in one of the [previous posts][performance-bottleneck]. At that time, I was able to improve the performance by switching from `HashMap` to `BTreeMap`. However, despite this effort, we still saw the polynomial growth in therms of the execution time. We see this effect when we [increase the length of the input text][performance-bottleneck-results], but also the same effect was seen when we [played with different ledger sizes][ledger-sizes-experiment-results] on the fixed input data. 

**Polynomial time growth is a performance killer.**

# Identifying the bottleneck

Let's rediscover where the algorithm spends most of the time, using our familiar tool, [the flamegraph][flamegraphs]. This time, I'm going to profile the _release_ version of the application. In contrast with the debug version, when Rust compiles the release version of an executable, it applies some performance optimizations and removes runtime checks. As a result, the release build runs significantly faster. 

However, the release build doesn't include the debug information, which makes flamegraphs much less informative, since it doesn't show meaningful function names anymore. To avoid that, we can instruct Cargo to include the debug symbols into the release build, by adding the following configuration option into the `.cargo/config.toml` file: 

```toml
[profile.release]
debug = true
```

Here's the flamegraph that was generated (click to see the [interactive version][flamegraph-before]): 

[![Flamegraph before optimization][flamegraph-before]][flamegraph-before]

From the flamegraph, the bottleneck is apparent: most of the time is spent in [`SubstringLedger::find_longset_match()`][find-longest-match-0.0.8]: 

```rust
pub fn find_longest_match(&self, text: &str) -> Option<Substring> {
    self.substrings
        .keys()
        .find(|&substr| substr.matches_start(text))
        .map(|substr| substr.clone())
}
```  

Indeed, the culprit is quite obvious: we iterate over all keys in the `substrings` collection to find the substring that matches the beginning of the text. In the majority of calls, it has to go through the entire collection, to register the miss. As we accumulate more and more substrings, we have to go through progressively longer list of keys, and do it countless number of times, as we scan through the input text. Hence, the polynomial growth in the execution time. 

To alleviate the problem, we need a more clever data structure, that would allow us to find a matching substring in the collection more efficiently. Enter _tries_. 

# The trie data structure 

A [trie][trie-wikipedia] (pronounced as "try") is a data structure that's designed specifically for efficient string search. The data in the trie is organized in a tree, where each node represents a single character in a string, and the path from the root to a particular node represents the string itself. Each node can store a value, associated with the particular string, or it can be "empty" for intermediate nodes. A key characteristic of tries is that all children of a node share the same prefix, formed by the characters from the root to their parent node.

Here's, for example, how we would store the word counts of the phrase _she sells shells by the sea_ in the trie:  

![Trie data structure]({{ site.baseurl }}/assets/images/optimize-substring-map/trie-structure.svg)


The power of the trie structure is that allows for efficient prefix search. To check if the input text starts with a known substring, we simply need to follow the structure from the root to a particular node, guided by the characters from the input text, until we reach a leaf node. Along the way, we keep track of the values, stored in the nodes we visited. Once we've reached the leaf node, the last seen value is the one that's associated with the longest match. If none of the nodes we visited contain a value, then it means that the input text doesn't start with any known substring. 

The time complexity for searching a substring at the beginning of the input text is _O(m)_, where _m_ is the length of the longest substring in the trie.

# Implementing the trie 

#### Refactor to abstraction

Unfortunately, the current state of the code doesn't allow me to switch underlying data structures easily. The knowledge about the substring collection being a `BTreeMap` has penetrated into different parts of the code. To switch to a different data structure, I need to first create an abstraction layer that would hide the details of the underlying implementation. 

To do that, I walked through the code, and analyzed what methods of `BTreeMap` were used. With a bit of refactoring, I was able to narrow it down to just a handful of methods, which I extracted into a new trait, called [`SubstringCounts`][substring-counts-0.0.9]: 

```rust
pub trait SubstringCounts {
    fn len(&self) -> usize;
    fn contains_key(&self, substr: &Substring) -> bool;
    fn find_match(&self, text: &str) -> Option<SubstringCount>;    

    fn insert(&mut self, substring: Substring, count: usize);
    fn get_count_mut(&mut self, substr: &Substring) -> Option<&mut usize>;

    fn iter(&self) -> impl Iterator<Item = (&Substring, usize)>;
    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&Substring, usize) -> bool;
}
```

I altered the code to work with the substrings via that new abstraction, and created a default implementation, [`BTreeSubstringCounts`][btree-counts-0.0.9], which is a simple wrapper around a `BTreeMap`. 

#### Implementing `TrieSubstringCounts`

Now I can make a second implementation of `StringCounts` that implements the trie data structure, called [`TrieSubstringCounts`][trie-counts-0.0.9]. It follows the "canonical" trie implementation, with a few adjustments: 

* We store children as a `HashMap`, indexed by the character. The canonical implementation is to use a pre-allocated array of links to the child nodes, but in case of UTF-8 encoding such arrays would be huge. `HashMap` is the best second option, in terms of access time / memory efficiency. 

* In the original trie implementation, the string itself is stored _implicitly_ in the structure. I've found that it's more convenient for my purposes to also store the string _explicitly_ in the nodes, along with the count values. It makes it easier to return references to substrings in the `iter` method, and also generally helps to avoid reconstructing the substrings every time.  

The structure for the trie node looks like this: 

```rust 
#[derive(Debug)]
struct TrieNode {
    count: Option<SubstringCount>,
    children: HashMap<char, TrieNode>,
}
```

The intermediate nodes will contain `None` in their `count` field, and the nodes that represent substrings will keep both the string and its count, packed into the `SubstringCount` struct. 

#### My struggles with `retain_if`

One particular function that gave me a few headaches was `retain_if`. It is called when we need to clean up the substring ledger, to remove all substrings with counts lower than the threshold. It turned out, it's not that easy to implement this function, and satisfy Rust's borrow checker at the same time. 

In a nutshell, the algorithm for removing the nodes from a trie is a bottom-up recursion. We start from the leaf nodes. If the leaf node doesn't satisfy the condition for retention, we can safely remove that node from the tree. As we go upwards from the bottom to the top of the tree, we apply the same procedure to the upper levels. Eventually, we end up with a tree where all empty nodes (the ones that don't keep a value and don't have children) are removed. 

As it turned out, this is not a straightforward thing to implement in Rust, and keep the borrow checker satisfied. In essence, it boils down to creating a _postorder mutable iterator_, where you need to store multiple mutable references to the same node, and Rust's borrow checker doesn't allow it. To implement such an algorithm, one needs to rethink the way the tree structure is stored in memory (I've found [an article][tree-traversal-arena] that goes into the details). Another way to achieve the same effect could be to avoid mutable references altogether, and build a copy of the trie without empty nodes.  

In the end, I decided to proceed with a simpler implementation that doesn't remove empty nodes from the trie. It only removes the values from the nodes, but doesn't alter the tree structure. Sure, this incurs some penalty in terms of used memory and the lookup speed, but for now it seems to work reasonably well. I've decided put off a "proper" implementation for a later time.

# Running the optimized version 
 
 Now with the implementations in place, we can run our [previous experiments][ledger-sizes-experiment-results] with different ledger sizes using `BTreeSubstringCounts` as the workhorse, and see how it impacts the performance: 


| Max Ledger Size | Learned Ledger Size | Compression Ratio | Time Elapsed |
| --------------: | ------------------: | ----------------: | -----------: |
|             256 |                 223 |             6.75% |        0.44s |
|             512 |                 490 |            13.91% |        0.61s |
|           1 024 |                 873 |            20.75% |        0.80s |
|           2 048 |               2 047 |            33.71% |        1.01s |
|           4 096 |               4 094 |            41.53% |        1.98s |
|           8 192 |               8 179 |            46.40% |        1.62s |
|          16 384 |              16 339 |            49.40% |        1.47s |
|          32 768 |              32 611 |            50.53% |        1.49s |
|          65 536 |              64 887 |            51.06% |        1.57s |
|         131 072 |             117 596 |            51.19% |        1.66s |
|         262 144 |             145 888 |            51.18% |        1.73s |

![Compression ratio and Time elapsed, by ledger limit]({{ site.baseurl }}/assets/images/optimize-substring-map/comp-ratio-time-elapsed-by-limit.svg)

That's a huge performance boost, compared to the [previous results][ledger-sizes-experiment-results]! We went from 35 seconds for 64K ledger, to less than 2 seconds! In addition, there's no increase in the execution time when we increase the maximum ledger size, it stays below 2 seconds. That's remarkable! 

Using the new improved version, I was able to experiment with even larger substring ledger sizes beyond 64K. As the results above show, we don't get significant compression gains beyond the size of 32K. For now, I'm going to stick with the maximum ledger size of 64K. 

# Flamegraph after optimization 

Finally, let's have a look at the [flamegraph][flamegraph-after] to see the difference: 

[![Flamegraph after optimization][flamegraph-after]][flamegraph-after]

As we can see, `SubstringLedger::find_longset_match()` has completely disappeared from the picture. In fact the entire `build_ledger()` function is barely noticeable here, taking up only 3% of the total execution time. On the other hand, `encode_string()` is now becoming much more prominent, as a next potential performance bottleneck. 

If we look closer to the implementation of [`encode_string()`][encode-string-0.0.9], we can see that is has the issue similar to our previous `SubstringLedger`. Inside, it relies on the `EncodingTable` instance to search for matches during the encoding phase, which in turn uses a `Vec<Substring>` inside. This is also a good candidate to switching to a trie structure for efficiency. 

For the time being, however, I'm quite satisfied with the current results, so I've decided to keep the current implementation of `EncodingTable`. Since the size of the encoding table [is fixed to 2816 entries][expand-encoding-table], I don't expect the execution time to slip into polynomial growth because of the inefficiency here. I might revisit it later and switch to the trie implementation as well. 

# Next steps

Current progress is available on GitHub under the [tag 0.0.9][tag-0.0.9]. 

The core algorithm is done now, to be determined what to do next. 

[trie-wikipedia]: https://en.wikipedia.org/wiki/Trie
[prev-post]: {{site.baseurl}}/{% post_url 2025-02-27-expand-encoding-table %}
[performance-bottleneck]: {{site.baseurl}}/{% post_url 2025-01-17-tackling-the-performance-bottleneck %}
[performance-bottleneck-results]: {{site.baseurl}}/{% post_url 2025-01-17-tackling-the-performance-bottleneck %}#results
[ledger-sizes-experiment-results]: {{site.baseurl}}/{% post_url 2025-02-27-expand-encoding-table %}#results
[flamegraphs]: {{site.baseurl}}/{% post_url 2025-01-12-profiling-with-flamegraphs %}
[flamegraph-before]: {{ site.baseurl }}/assets/images/optimize-substring-map/flamegraph-before.svg
[flamegraph-after]: {{ site.baseurl }}/assets/images/optimize-substring-map/flamegraph-after-optimization.svg
[find-longest-match-0.0.8]: https://github.com/tindandelion/rust-text-compression/blob/0.0.8/src/encoder/substring_ledger.rs#L36
[substring-counts-0.0.9]: https://github.com/tindandelion/rust-text-compression/blob/0.0.9/src/encoder/substring_counts.rs#L9
[btree-counts-0.0.9]: https://github.com/tindandelion/rust-text-compression/blob/0.0.9/src/encoder/substring_counts/btree.rs
[trie-counts-0.0.9]: https://github.com/tindandelion/rust-text-compression/blob/0.0.9/src/encoder/substring_counts/tries.rs
[tree-traversal-arena]: https://sachanganesh.com/programming/graph-tree-traversals-in-rust/
[encode-string-0.0.9]: https://github.com/tindandelion/rust-text-compression/blob/0.0.9/src/encoder/encode_string.rs
[expand-encoding-table]: {{site.baseurl}}/{% post_url 2025-02-27-expand-encoding-table %}
[tag-0.0.9]: https://github.com/tindandelion/rust-text-compression/tree/0.0.9