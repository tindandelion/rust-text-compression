---
layout: post
title: Limiting the ledger size
date: 2025-02-12
---

Now it's time to look into another aspect of building the substring dictionary: how do we keep its size limited? So far, I've been skipping this part, letting it grow indefinitely, but the original task is to put a limit on the size of the substring ledger while we're looking for common substrings. 

There are two aspects to the strategy of keeping the ledger size under control: 

- When do we add a new compound string to the ledger? The criteria here is that we only add a new compound string, when the counts of its components are above a certain threshold. 
- When we inevitably reach the size limit, we need to evict some substrings, freeing space for new ones. 

The [original article][mayne] suggests a few different strategies when it comes to adding new compound substrings. It doesn't go into much details about their experiments, but their main finding was that the best results were achieved when we take into account the current available space in the ledger. [Etudes][etudes] algorithm has a similar approach, and it dives us a straightforward policy: 

- The threshold for adding a new compound substring is the maximum size of the ledger divided by the number of free slots; 
- The initial count of a new compound substring is _1_.  
- To clean up, we remove the substrings whose counts are below the median value.

The complete algorithm looks like this: 

![Complete algorithm]({{ site.baseurl }}/assets/images/limit-ledger-size/complete-algo.svg)

# Ledger policy implementation

To make things more interesting, I've decided to separate these two aspects of the algorithm into different parts. The intricacies of keeping the ledger size are hidden from the main algorithm behind the `LedgerPolicy` trait: 

```rust
pub trait LedgerPolicy {
    fn should_merge(&self, x: &Substring, y: &Substring, substrings: &SubstringMap) -> bool;
    fn cleanup(&self, substrings: &mut SubstringMap);
}
```

This allows me to use different policies without having to change the main algorithm. In particular, in tests I can use a simpler `CaptureAll` policy, so that I can focus on testing the core logic. Similarly, for testing merging and eviction, I can use simpler test doubles for `LedgerPolicy`, to make tests simpler. The "production" implementation is called `LimitLedgerSize`, which I also exhaustively test separately. 

Essentially, the structure of this part of the program looks like this: 

![SubstringLedger relations]({{ site.baseurl }}/assets/images/limit-ledger-size/build-ledger-relations.svg)

# Next steps

#TODO: Write it  


[mayne]: https://academic.oup.com/comjnl/article/18/2/157/374138
[etudes]: https://www.goodreads.com/book/show/3924336-etudes-for-programmers


