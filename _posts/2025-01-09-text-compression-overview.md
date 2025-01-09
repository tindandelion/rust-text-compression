---
layout: post
title:  "Text compression: an overview"
---

Text data usually contains a lot of redundancy. Indeed, in natural language texts, the sequences of characters, words, end even whole phrases are repeated very often throughout the text. Using that fact, we can make the text much shorter, if we replace most commonly occurring patterns with shorter representations. 

To achieve that, we first need to build a dictionary of most frequent substrings occurring in the text. Then, we can use the index of the substring in that dictionary to represent it in the encoded text. To indicate that a substring is replaced by its index, we'll need a special marker to be placed in the encoded text, so that we can decode it back to the original form.

Consider the following, rather synthetic, example: 

```
low low low low low lowest lowest lower lower lowest
```

Just by glancing through this text, I can come up with a simple list of frequent substrings: `[low, lowe]`. Using `#` as a marker, we can encode the text as 

```
#0 #0 #0 #0 #0 #1st #1st #1r #1r #1st
```

That would save us 53 - 37 = 16 bytes. Of course, the substring dictionary will also need to be stored along with the encoded text, so the overall gain will be lower, but you get the idea.

But indeed, we can do even better that that. For one, extracting one more substring, `low low `, we can achieve better compression. That's the purpose of the dictionary building algorithm: it needs to go through the text, and find the longest common substrings for extraction. 

There's multiple ways to build a substring dictionary. One such algorithm is [Byte-Pair Encoding (BPE)][bpe], often used in machine learning for tokenization. This algorithm does multiple passes through the source text, merging the most frequent pairs of characters into a single token. By repeating this process multiple times, it discovers longer and longer substrings.

For this project, however, I decided to implement a different approach, described in a paper [Information compression by factorising common strings][mayne], by A. Mayne, E. B. James. It was also popularized by Charles Wetherell in his book [Etudes for Programmers][etudes], with some modifications. In contrast with BPE, this algorithm has the following properties:

1. It builds the dictionary in a single pass through the source text.
2. It limits the size of the dictionary to a fixed number of entries. To maintain the dictionary size, it applies some strategy to add new entries, and prune the dictionary when it's close to the limit.

Overall, I found this algorithm to be more entertaining to play with, because it leaves more room for experimentation. Once it's done, it would be interesting to compare the results of this algorithm with BPE, and also with the binary compression algorithms.

[bpe]: https://en.wikipedia.org/wiki/Byte_pair_encoding
[mayne]: https://academic.oup.com/comjnl/article/18/2/157/374138
[etudes]: https://www.goodreads.com/book/show/3924336-etudes-for-programmers