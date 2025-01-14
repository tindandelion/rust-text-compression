---
layout: post
title: Tackling the performance bottleneck
date: 2025-01-13
---

TODO: Putting the summary here

# _HashMap_ and _BTreeMap_

Rust's standard library provides two different data structures to work with key-value pairs: [`HashMap`][hashmap] and [`BTreeMap`][btreemap]. 

`HashMap` is probably the default choice for most use cases. This data structure is optimized for fast lookups, effectively ensuring constant time performance for inserting, deleting, and accessing elements by key. The data type of the key is required to implement the `Hash` and `Eq` traits. Implementing the efficient hash function is a non-trivial task, but luckily, in most cases we can rely on the default implementations for the standard data types. For custom types, the magic `#derive` macro[^1] can be used to automatically generate the required implementations. It's quite common to see `#[derive(Hash, PartialEq, Eq)]` to supply the required implementations for the key type.

But `HashMap` has a significant drawback for my current use case: it doesn't preserve the order of the keys. When I iterate over the substrings with `HashMap::keys()` method, they will be returned to me in a somewhat random order, and so I always have to sort them by the substring length to find the longest match. [It's clear][profiling-experiment] that at the moment my program is spending most of the time doing that. So I need a different data structure, the one that keeps the map's keys in a sorted order at all times. Enter `BTreeMap`.

In contrast, `BTreeMap` keeps the keys ordanized in a tree structure, called [B-Tree][btree]. Essentially, B-Tree is a balanced tree that taks advantage of the CPU memory cache, so it works better than a regular binary tree on modern CPU architectures. `BTreeMap` provides logarithmic time complexity for accessing the elements by key, but more importantly for us, `BTreeMap::keys()`  always keeps the keys in a sorted order, hence eliminating the need for sorting them each time. `BTreeMap` requires the key type to be sortable, meaning that it must implement the `Ord` trait. Sometimes we can get away with default implementations, provided by the `#[derive]` macro, but in my case I have a custom sorting order, so I need to implement the `Ord` trait manually.

# Custom sorting order: _Ord_ trait

Describe the `Ord` trait here.

#### Ord and PartialOrd: which algorithms need which?

Looks like PartialOrd is needed for sorting.

# The _Tiny Type_ design pattern 


# The results after the change

Here are the results of running the main program after the change:

| File Name       | Source Length (chars) | Compression Ratio | Time (s) |
| --------------- | --------------------: | ----------------: | -------: |
| hamlet-100.txt  |                 2,763 |            41.40% |     0.05 |
| hamlet-200.txt  |                 7,103 |            33.31% |     0.16 |
| hamlet-400.txt  |                16,122 |            32.17% |     0.55 |
| hamlet-800.txt  |                32,894 |            30.04% |     1.77 |
| hamlet-1600.txt |                67,730 |            28.67% |     5.75 |
| hamlet-3200.txt |               136,410 |            28.79% |    18.73 |
| hamlet.txt      |               191,725 |            28.97% |    32.88 |

And the top 5 common substrings extracted from the entire text of "Hamlet" go as follows: 

```
1: ".\n                                                         Exeunt.\n\n\n\n\nScene II.\nElsinore. A"
2: "follow.\n                                                         Exeunt.\n\n\n\n\n"
3: ".\n                                                         Exeunt.\n\n\n\n\n"
4: ".\n                                                         Exeunt.\n\n"
5: ".\n                                                         "
```

![Running times]({{ site.baseurl }}/assets/images/switching-to-btreemap/running-times.png)

Clearly, the performance has become much better: encoding 1600 lines of "Hamlet" takes 5.75 seconds, versus 54.51 seconds from the [previous results][first-iteration]. However, the increase in execution time with the text length is still not ideal.
Despite the obvious improvement, the execution time growth still looks like a polynomial curve.

Since the execution time has fallen dramatically, I can now run the performance test on the entire text of "Hamlet". Here's how the flamegraph looks (click to enlarge):

[![Flamegraph][flamegraph]][flamegraph]

[`SubstringLedger::find_longest_match()`][substring-ledger-longest-match] is still the bottleneck, but the situation has changed. I got rid of the excessive sorting, and now most of the time is spent in trying to find the matching substring in the dictionary.

When trying to find the match, we iterate over all keys in the map. It's clear that, since currently the size of the substring dictionary is not limited, the time to go through all keys is going to grow linearly with the dictionary size. On top of that, at each step we check if the current substring matches the start of the text. That in itself is a linear operation, and it will take more and more time, as we accumulate progressively longer substrings as keys. I think that explains the polynomial growth of the execution time.

# The verdict 

So, the switch to `BTreeMap` was a clear benefit in terms of performance, but the polynomial growth of the execution time is still an issue. For now I can see two ways to further improve the performance:

1. Find a better data structure to store the substring ledger that won't involve the scan of the entire list of substrings to find the match, **or**:
2. Move to the next stage of the project, in which the size of the dictionary will be limited during construction, which by extension will reduce the time spent in `SubstringLedger::find_longest_match()`.

Option 1 is a very interesting problem to tackle, but I feel that I may be falling a victim to the premature optimization here. Since I haven't yet implemented the entire algorithm, I might be spending too much time optimizing the wrong thing. So for now, I'm more tempted to move forward with option 2. Once I have at least a simple way to limit the size of the dictionary, I can come back to tackling the performance issues again, but this time with all bits of the implementation in place.

# Next steps

[^1]: `#derive` macro is widely used in Rust programs, but it looks like magic to me. That deserves a separate exploration, so for the time being I assume that it "just works". 

[hashmap]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[btreemap]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
[first-iteration]: {{site.baseurl}}/{% post_url 2025-01-10-first-iteration %}
[profiling-experiment]: {{site.baseurl}}/{% post_url 2025-01-12-profiling-with-flamegraphs %}
[flamegraph]: {{ site.baseurl }}/assets/images/switching-to-btreemap/profile-flamegraph.svg
[substring-ledger-longest-match]: https://github.com/tindandelion/rust-text-compression/blob/0.0.3/src/encoder/substring_ledger.rs#L29
[btree]: https://en.wikipedia.org/wiki/B-tree

