Text data usually contains a lot of repetitive patterns. Indeed, in natural language texts, the sequences of characters, words, end even whole phrases are repeated very often. Using that fact, we can make the text much shorter, if we replace most commonly occurring patterns with shorter representations. To achieve that, we can compose a list of most frequent substrings, and use the index of the substring in the list to represent it in the encoded text. We'll also need a special marker as an indicator of the replacement, so that we can decode the text back to the original form.

Consider the following, rather synthetic, example: `low low low low low lowest lowest lower lower lowest`. Just by glancing through this text, I can come up with a simple list of frequent substrings: `[low, lowe]`. Using `#` as a marker, we can encode the text as `#0 #0 #0 #0 #0 #1st #1st #1r #1r #1st`. That would save us 53 - 37 = 16 bytes.

But indeed, we can do even better that that. For one, replacing one more substring, `low low `, we can compress the text even more. We need an algorithm to build the dictionary of common strings.

There's multiple ways to build such a dictionary. One such algorithm is [Byte-Pair Encoding (BPE)](https://en.wikipedia.org/wiki/Byte_pair_encoding), often used by LLMs for tokenization. This algorithm does multiple passes through the source text, merging the most frequent pairs of characters into a single token. By repeating this process multiple times, it discovers longer and longer substrings.

For this project, however, I decided to implement a different algorithm, which I've found in a classical book [Etudes for Programmers, by Charles Wetherell](https://www.amazon.com/Etudes-Programmers-Charles-Wetherell/dp/0132918072). Etude 11, "Ye Soule of Witte", is dedicated to the problem of text compression. It is based on the paper [Information compression by factorising common strings, by A. Mayne, E. B. James](https://academic.oup.com/comjnl/article/18/2/157/374138), which is available for free.

This algorithm has the following properties:

1. It builds the dictionary in a single pass through the source text.
2. It uses the dictionary of a limited size.

Overall, I found this algorithm to be more entertaining to play with, as it leaves more room for experimentation.
