# Performance optimization

As I've mentioned in the [previous post](TODO: Link), I'm not happy with the performance of my implementation. I'm ready to dive into exploring the bottlenecks. But before I do that, I need to create a second executable that I could use in performance testing. The goal is to create simpler executable that would run on a single file, which I could run multiple times while experimenting.

## Multiple binary crates
