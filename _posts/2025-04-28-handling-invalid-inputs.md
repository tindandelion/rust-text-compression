---
layout: post
title: Handling invalid input
date: 2025-04-28
---

Up to this point, I was mostly focusing on getting things to work, and I've been blissfully ignoring various error conditions that can occur. Now, I'd like to switch gears and pay more attention to the aspect of error handling. Two main functions of the compression algorithm are [`encode()`][encode-0.1.0] and [`decode()`][decode-0.1.0], so let's focus on them. In particular, let's examine if there are possibilities for these functions to accept invalid inputs, and how they should respond to such inputs. 

# What could possibly go wrong? 

#### `encode()` function 

Looking at [`encode()`][encode-0.1.0] function signature, it turns out that there's not much of a possibility for erroneous input here: that function accepts the source text as a string slice (`&str`). Luckily for us, Rust does all heavy lifting. Strings in Rust are guaranteed to [always be valid UTF-8 sequences][rust-doc-string], so I can't imagine a scenario where the input to this function could be invalid: the `String` implementation and Rust type checker won't allow it. 

#### Are string slices UTF-8? 

There's one question that arises here. As I mentioned, strings in Rust are valid UTF-8 strings. But what about string slices (`&str`)? `encode()` function accepts a string slice: is it possible to slice up a UTF-8 string such that the slice is no longer UTF-8? 

Consider the following example: 

```rust 
#[test]
fn slice_multibyte_string_in_the_middle_of_a_charater() {
    let source = "こんにちはこんにちは世界世界";

    let slice = &source[0..1];
    assert_eq!(slice, "");
}
```

Here, we have a multi-byte UTF-8 string `source`. One might think that what we get in `slice` is a first character of the source string, as indicated by the range of `0..1`. However, that's not true: in Rust, slice bounds indicate *byte indexes*, not *character indexes*: they are different in case of multi-byte Unicode characters. Effectively, Im trying to create a string slice from the first 2 bytes of a multi-byte character 'こ'. So at a first glance, it looks like it's possible to create an invalid string slice from a valid UTF-8 string.

However, Rust prevents us from creating such slices. The compiler will accept this code, but it will fail at runtime with an error: 

```
byte index 1 is not a char boundary; it is inside 'こ' (bytes 0..3) of `こんにちはこんにちは世界世界`
```

The main takeaway is that *string slices are also valid UTF-8 strings in Rust.* If you try to slice the string across character boundaries, you'll end up with the runtime error. It's worth remembering that bounds in string slices are **byte indexes**, not **character indexes**.  

If you need to slice a string by character indexes, you need to use the `chars()` iterator and collect the characters you want.

#### `decode()` function 

With [`decode()`][decode-0.1.0], the situation is different. It accepts an encoded string in a form of a byte vector, and there's a few possibilities of errors in the input data: 

* Invalid UTF-8 codes in un-encoded parts of the text; 
* Invalid substring index: the entry is missing from the encoding table; 
* Substring index is missing: the encoded string ends with the encoded "marker" byte (`0xF5..0xFF`), but there's no follow-up "index" byte `NN`. 

These are the situations that should be accounted for when decoding the input byte array. 

# Tidbits of error handling in Rust

Error handling in Rust is well documented in [multiple sources][rust-book-errors], so let's not waste space on obvious things. In this section, I'd like to mention a few things I've learned on top of the basics. 

#### `std::Error` trait 

When you define custom error types, it's a good practice to make your error type implement a `std::Error` trait. This makes your error type fit nicely into the rest of Rust ecosystem. Implementing `Error` trait also requires you to implement `Debug` and `Display` traits on your error types. 

Luckily, that doesn't require a lot of coding in common cases. The default implementation of `Error` and `#[derive(Debug)]` are sufficient to comply. The only piece of code you need to write is the implementation of `Display::fmt()` method. This method is supposed to provide a *user-friendly* information about what went wrong: it's up to you as an application developer to decide what will be shown to the user.   

There are no specific guidelines about how `Display::fmt()` should be implemented. Jon Gjengset gives the following advice in his book ["Rust for Rustaceans"][rust-for-rustaceans], in the chapter about error handling: 

> In general, your implementation of Display should give a one-line description of what went wrong that can easily be folded into other error messages. The display format should be lowercase and without trailing punctuation so that it fits nicely into other, larger error reports.

Another interesting method in `std::Error` is `source()`. The default implementation simply returns `None`. It is useful if your error type is a wrapper around another error. In this case, you can override this method to give access to the inner error. 

#### Syntax sugar: `?` operator

The `?` operator is commonly used in Rust to avoid repetitive boilerplate code related to error propagation. It's a shorthand for *unwrap the successful result or return the error early*, in cases when you simply need to abort the function and return the error to the caller. But it's not limited only to errors: it works with `Option` type also. 

Interestingly, the `?` operator uses the [`Try`][try-trait] trait behind the scenes. Currently, this trait is still experimental, but once it stabilizes, it will be possible to make your own types `?`-compliant by implementing this trait. 

#### `try` blocks 

Yet another **experimental feature** I've learned about is [`try`][try-blocks] blocks. 

Sometimes an early return with `?` operator can backfire. Consider the following example: 

```rust
fn query_database() -> Result<i32, Error> {
    let conn = Database::connect()?;

    let x = query_value(conn)?; // Early return on error
    let y = x + 10;

    conn.close();
    Ok(y)
}
```

If `query_value(conn)` returns an error, the `?` operator will cause the entire `query_database()` to return early and skip the important `conn.close()` call at the end! This is the problem `try` blocks are intended to solve: 

```rust
fn try_query_database() -> Result<i32, Error> {
    let conn = Database::connect()?;

    let y: Result<i32, Error> = try {
        let x = query_value(conn)?;
        x + 10
    };

    conn.close();
    y
}
```

In this snippet, `query_value()?` will return early only from the `try` block, and the execution will proceed to close the connection and return the result. 

# Changes to the code 


[encode-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder.rs#L20
[decode-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/decoder.rs#L5
[rust-doc-string]: https://doc.rust-lang.org/rust-by-example/std/str.html
[rust-book-errors]: https://doc.rust-lang.org/book/ch09-00-error-handling.html
[rust-for-rustaceans]: https://rust-for-rustaceans.com/
[try-trait]: https://doc.rust-lang.org/std/ops/trait.Try.html
[try-blocks]: https://doc.rust-lang.org/beta/unstable-book/language-features/try-blocks.html


