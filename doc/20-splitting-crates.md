# Performance optimization

As I've mentioned in the [previous post](TODO: Link), I'm not happy with the performance of my implementation. I'm ready to dive into exploring the bottlenecks. But before I do that, I need to create a second executable that I could use in performance testing. The goal is to create simpler executable that would run on a single file, which I could run multiple times while experimenting.

## Multiple binaries

- Cargo package can have multiple binaries
- Introduce a library crate `lib.rs`
- Move binary crate `main.rs` to `src/bin/` directory
- Create a new binary crate `performance-test.rs` in `src/bin/` directory
- Set up the default run crate in `Cargo.toml`
- Run `cargo run --bin performance-test` to run the performance test
