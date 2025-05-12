---
layout: post
title: Wrapping up the project 
date: 2025-05-09
---

Summary of what I've done in this training project.

# Main achievements 

* I started with a [simplified version][first-iteration] of the text compression solution, and later upgraded it to a [more advanced version][limit-ledger-size] that used an adaptive learning algorithm. I also experimented with [different substring ledger limits][ledger-limit-experiments] to see how it impacts the compression efficiency, to come up with the optimal value.

* I had to tackle the performance issues. The first version started with a [naive implementation using HashMap][first-iteration-results], later I [replaced it with BTreeMap][btree-map-impl], which helped performance a bit. However, the algorithm still showed polynomial time complexity, so I [implemented a much more suitable _trie_ data structure][trie-impl], and finally [achieved the performance][final-performance] I was satisfied with. 

* While tackling performance issues, I [learned how I could use flamegraphs][flamegraphs] in Rust to identify the bottlenecks; 

* On the encoding side, I [started with a simple scheme][first-encoding-scheme] that only allowed me to handle 256 substrings, and later [optimized it][encoding-optimization] to be able to encode 2816 substrings. That led to [big improvements in the compression ratio][encoding-optimization-results]. 

* Along the way, I've learned quite a few details about Rust itself, such as: [handling multiple crates][multiple-crates], [differences between HashMap and BTreeMap][rust-maps], details about implementing the [comparison traits][comparison-traits], and tidbits about the Rust way of [dealing with error conditions][error-handling].

[first-iteration]: {{site.baseurl}}/{% post_url 2025-01-10-first-iteration %}
[limit-ledger-size]: {{site.baseurl}}/{% post_url 2025-02-12-limit-ledger-size %}
[ledger-limit-experiments]: {{site.baseurl}}/{% post_url 2025-02-16-experiments-with-ledger-limit %}
[first-iteration-results]: {{site.baseurl}}/{% post_url 2025-01-10-first-iteration %}#results
[btree-map-impl]: {{site.baseurl}}/{% post_url 2025-01-17-tackling-the-performance-bottleneck %}
[trie-impl]: {{site.baseurl}}/{% post_url 2025-03-08-substring-map %}
[final-performance]: {{site.baseurl}}/{% post_url 2025-03-19-optimize-encoding %}#results
[flamegraphs]: {{site.baseurl}}/{% post_url 2025-01-12-profiling-with-flamegraphs %}
[first-encoding-scheme]: {{site.baseurl}}/{% post_url 2025-01-10-first-iteration %}#encoding-scheme
[encoding-optimization]: {{site.baseurl}}/{% post_url 2025-02-27-expand-encoding-table %}
[encoding-optimization-results]: {{site.baseurl}}/{% post_url 2025-02-27-expand-encoding-table %}#results
[multiple-crates]: {{site.baseurl}}/{% post_url 2025-01-11-splitting-crates %}
[rust-maps]: {{site.baseurl}}/{% post_url 2025-01-17-tackling-the-performance-bottleneck %}#rust-maps
[comparison-traits]: {{site.baseurl}}/{% post_url 2025-01-17-tackling-the-performance-bottleneck %}#comparison-traits
[error-handling]: {{site.baseurl}}/{% post_url 2025-05-02-handling-invalid-inputs %}