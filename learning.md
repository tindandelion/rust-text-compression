# Questions to ask

- Clippy - what is it?
- How does `cargo build` work? Where is the linker?
- Crates: binary vs library
- Add dependencies to a crate: manual vs cargo.
- crates.io - explore
- Types: enums
- Result type
- Memory management: stack vs heap
  - where do variables live?
  - Is there memory copying when passing variables to functions?
  - Is there memory copying when returning results from functions?
- Ownership and borrowing
- I installed `cargo-watch`. Where did it go?
- Cargo: custom commands
- Tests: custom formatting

# Unrelated

- Syntax Highlighter plugin - why is it better than the default one?

# Topics to mention

- BTreeMap
- PartialOrd, PartialEq, Ord, Eq
- Strings: different kinds of strings in Rust
- Difference between `&str` and `&String`

# First iteration

- Encoding: use 2 bytes 0xF5xx to encode a substring
- That gives us 256 possible substrings
- We only gain compression if the substring is longer than 2 bytes
- Debug: Why the compression ratio is different for the same input?
