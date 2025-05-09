---
layout: post
title: Wrapping up the project 
date: 2025-05-09
---

Summary of what I've done in this training project.

# Main achievements 

* I started with a [simplified version][first-iteration] of the text compression solution, and later upgraded it to a [more advanced version][limit-ledger-size] that used an adaptive learning algorithm. 

* I had to tackle the performance issues. The first version started with a [naive implementation using HashMap[first-iteration], later I [replaced it with BTreeMap][btree-map-impl], which helped performance a bit. However, the algorithm still showed polynomial time complexity, so I [implemented a much more suitable _trie_ data structure][trie-impl], and finally [achieved the performance][final-performance] I was satisfied with. 

* While tackling performance issues, I [learned how I could use flamegraphs][flamegraphs] in Rust to identify the bottlenecks; 

* On the encoding side, I [started with a simple scheme][first-encoding-scheme] that only allowed me to encode 256 substrings, and later [optimized it][encoding-optimization] to 2816 substrings. That led to big improvements in the compression ratio. 

* Along the way, I've learned quite a few details about Rust itself, such as: [handling multiple crates][multiple-crates], [differences between HashMap and BTreeMap][rust-maps], details about implementing the [comparison traits][comparison-traits], and tidbits about the Rust way of [dealing with errors][error-handling].



