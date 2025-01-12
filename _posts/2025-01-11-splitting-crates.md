---
layout: post
title: "Splitting crates"
date: 2025-01-11 
---

As I mentioned in the [previous post][prev-post], I'm not happy with the performance of my initial implementation, and I'm eager to dive into exploring the bottlenecks. But to make experiments more convenient, I would like to create a second, simpler executable that I could use in performance testing. It will peform the compression on a single input file, so the execution time will be much shorter, compared to the main executable that processes many files.

# More about crates

When considering projects with multiple executables, we need to dive into the concept of _crates_. Simply put, a crate is a unit of compilation for the Rust compiler. The crate can consist of multiple modules, and the modules may be defined in different files in the source directory structure. It's up to me as a developer to decide how to structure the code. There is, however, a special source file identified as the _crate root_, which acts as the starting point for the compiler. For simple projects, the crate root is the file named `src/main.rs`. In more complex projects, there will be multiple crate roots.

Crates come in two flavors: _library crates_ and _binary crates_. A binary crate is a program that will be compiled into an executable file. The crate root for the binary crate must contain the the entry point for the program: `main` function.

A library crate, on the other hand, does not produce an executable file. Instead, it contains the shared code that can be used by other crates.

Your package can consist of multiple crates, both library and binary. There can be only one library crate in the package, but multiple binary crates. By default, there are some conventions that Cargo uses to determine crate roots. For example, if the crate is binary, the root is the file named `src/main.rs`. If the crate is a library, the root will be `src/lib.rs`. When working with several binary crates, it makes sense to put them all into `src/bin/` directory: each file there will be considered a separate binary crate root by Cargo. 

The `src/bin/` is a resaonable default convention that Cargo uses. It can be overridden in the `Cargo.toml` file, but that's beyond the scope: for now, I'm quite satisfied with the defaults.

# Reorganizing the project into multiple crates

At the starting point, my project contained only one crate, with the root file in [`src/main.rs`][main-0.0.1]. My goal is to create two binary crates that would share the same core functionality, so I have to create a library crate first, by creating a [`src/lib.rs`][lib-0.0.2] and moving module definitions from `src/main.rs` to that new file. I noticed that `lib.rs` becomes a very natural place to export the high-level API functions that would be used by the binary crates. It also became a place where I could move the high-level tests:

```rust
mod decoder;
mod encoder;
mod substring_dictionary;

pub use decoder::decode_string as decode;
pub use encoder::encode;

#[cfg(test)]
mod tests {
    // ...
}
```

This is a feature of Rust module system: the `mod` keyword is used to declare a new module, and the `pub use` keyword can be used to re-export the items from the module and hide from the outside world where exactly they are defined.

Once I had the library crate ready, I could import `encode` and `decode` functions from it in the [`src/bin/main.rs`][main-0.0.2] file as follows:

```rust
use text_compression::decode;
use text_compression::encode;
```

Notice the use of `text_compression` module name. By default, the library crate has the same name as the package, specified in the `Cargo.toml` file:

```toml
[package]
name = "text_compression"
```

Having done that preliminary refactoring, I could now create a second binary crate for performance testing, called [`performance-test.rs`][performance-test-0.0.2]. This is a simplified version of the main executable, that only executes the encoding / decoding round on a single file `hamlet-800.txt`. It only takes a few seconds to execute, compared to more than 1 minute for the main executable.

Finally, I moved both `main.rs` and `performance-test.rs` to the [`src/bin/`][tag-0.0.2] directory. Each of them is now a separate binary crate, and I could run them using `cargo run --bin performance-test` or `cargo run --bin main`. In order to simplify launching the main executable, I added the following line to the `Cargo.toml` file, so I can run it by simply executing `cargo run` without the `--bin` parameter:

```toml
[package]
default-run = "main"
```

The final result is now available [here][tag-0.0.2].

# Final thoughts

In the hindsight, I think it's worth splitting the project into a library crate and a binary crate, even if there will be only one executable. Introducing a library crate has the following benefits for me:

- I can export the high-level functions and types from the library crate root. That way, I don't need to expose the details of the module structure to the binary crate. It also makes me think more about the design of the public API;
- I can place the tests for the public API into the library crate.

# Next steps

With all that preliminary work done, I can now [start exploring][next-step] the performance bottlenects, using the [`performance-test`][performance-test-0.0.2] executable. 

[prev-post]: {{site.baseurl}}/{% post_url 2025-01-10-first-iteration %}
[main-0.0.1]: https://github.com/tindandelion/rust-text-compression/blob/0.0.1/src/main.rs
[lib-0.0.2]: https://github.com/tindandelion/rust-text-compression/blob/0.0.2/src/lib.rs
[main-0.0.2]: https://github.com/tindandelion/rust-text-compression/blob/0.0.2/src/bin/main.rs
[performance-test-0.0.2]: https://github.com/tindandelion/rust-text-compression/blob/0.0.2/src/bin/performance-test.rs
[tag-0.0.2]: https://github.com/tindandelion/rust-text-compression/tree/0.0.2
[next-step]: {{site.baseurl}}/{% post_url 2025-01-12-profiling-with-flamegraphs %}




