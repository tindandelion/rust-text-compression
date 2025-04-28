---
layout: post
title: Handling invalid input
date: 2025-04-28
---

Up to this point, I was mostly focusing on getting things to work, and I've been blissfully ignoring various error conditions that can occur. Now, I'd like to switch gears and pay more attention to the aspect of error handling. Two main functions of the compression algorithm are [`encode()`][encode-0.1.0] and [`decode()`][decode-0.1.0], so let's focus on them. In particular, let's examine if there are possibilities for these functions to accept invalid inputs, and how they should respond to such inputs. 

# What could possibly go wrong? 

Looking at [`encode()`][encode-0.1.0] function signature, it turns out that there's not much of a possibility for erroneous input here: that function accepts the source text as a string slice (`&str`). Luckily for us, Rust does all heavy lifting. Strings in Rust are guaranteed to [always be valid UTF-8 sequences][rust-doc-string], so I can't imagine a scenario where the input to this function could be invalid: the `String` implementation and Rust type checker won't allow it. 

With [`decode()`][decode-0.1.0], the situation is different. It accepts an encoded string in a form of a byte vector, and there's a few possibilities why the input may be invalid: 

* Todo: Enumerate error conditions. 


# Error handling in Rust: `Result` type 

#### `std::Error` trait 

#### `?` operator as a syntax sugar 

What types can be `?`-ed? 

#### `try` blocks 


[encode-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder.rs#L20
[decode-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/decoder.rs#L5
[rust-doc-string]: https://doc.rust-lang.org/rust-by-example/std/str.html

