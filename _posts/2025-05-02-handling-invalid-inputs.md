---
layout: post
title: Handling invalid inputs 
date: 2025-05-02
---

Up to this point, I was mostly focusing on getting things to work, and I've been blissfully ignoring various error conditions that can occur. Now, I'd like to switch gears and pay more attention to the aspect of error handling. Two main functions of the compression algorithm are [`encode()`][encode-0.1.0] and [`decode()`][decode-0.1.0], so let's focus on them. In particular, let's examine if there are possibilities for these functions to accept invalid inputs, and how they should respond to them. 

# What could possibly go wrong? 

#### _encode()_ function 

Looking at the [`encode()`][encode-0.1.0] function signature, it looks like there's not much of a possibility for erroneous input here: that function accepts the source text as a string slice (`&str`). Luckily for us, Rust does all heavy lifting: strings in Rust are guaranteed to [always be valid UTF-8 sequences][rust-doc-string], so I can't imagine a scenario where the input to this function could be invalid: the `String` implementation won't allow it. 

#### Are string slices UTF-8? 

There's one question that arises, though. As I mentioned, strings in Rust are valid UTF-8 strings. But what about string slices (`&str`)? `encode()` function accepts a string slice: is it possible to slice up a string such that the slice is no longer UTF-8? 

Consider the following example: 

```rust 
#[test]
fn slice_multibyte_string_in_the_middle_of_a_charater() {
    let source = "こんにちはこんにちは世界世界";

    let slice = &source[0..1];
    assert_eq!(slice, "");
}
```

Here, we have a multi-byte UTF-8 string `source`. One might think that what we get in `slice` is the first character of the source string, as indicated by the range of `0..1`. However, that's not true: in Rust, slice bounds indicate *byte indexes*, not *character indexes:* they differ in case of multi-byte Unicode characters. Effectively, I'm trying to create a string slice from the first 2 bytes of a 4-byte character 'こ'. So at the first glance, it looks like it's possible to create an invalid string slice from a valid UTF-8 string.

However, Rust prevents us from doing so. The compiler will accept this code, but it will panic at runtime with an error: 

```
byte index 1 is not a char boundary; it is inside 'こ' (bytes 0..3) of `こんにちはこんにちは世界世界`
```

The main takeaway is that *string slices are also valid UTF-8 strings in Rust.* If you try to slice the string across character boundaries, you'll end up with the runtime error. **It's worth remembering that bounds in string slices are _byte indexes_, not _character indexes_**. If you need to slice a string by character indexes, you should use the `chars()` iterator and collect the characters you want.

#### _decode()_ function
{: #error-types }

With [`decode()`][decode-0.1.0] the situation is different. It accepts an encoded string in a form of a byte vector, and there's a few possibilities of errors in the input data: 

* invalid UTF-8 codes in un-encoded parts of the text; 
* invalid substring index: the entry is missing from the encoding table; 
* substring index is missing from the input: the encoded string ends with the encoded "marker" byte (`0xF5..0xFF`), but there's no follow-up "index" byte `NN`. 

These are the situations that should be accounted for when decoding the input byte vector. 

# Changes to the code 

Before moving on to the implementation details, I'd like to make a disclaimer: I'm not striving for a production-ready solution here. Implementing proper error handling for a production-ready feature _is hard_. I think the best approach is to run several iterations on it. The iteration loop should include the feedback from the testers, UI designers, real users, as well as developers themselves: 

* from a user perspective, you should ensure that the error was correctly shown in the UI. This process may include making changes to the application UI, thinking carefully about when and how the users get the error notifications, what actions they can take to correct the error, distinguish between the business logic and technical errors, etc.;
* from a developer perspective, you should have enough information in the internal logs to understand where an error occurred and what conditions led to it.

In this training project, I'm not striving for a production-ready solution. This allows me to simplify error handling code significantly: I don't have to think about user interface parts, nor about collecting accurate detailed information about the error conditions. For me, the goal is to be able to detect different types of errors and report them back, so I can have some understanding about what went wrong, without having to thoroughly analyze the stack trace. 

With all these considerations out of the way, let's dive into the implementation details. 

#### *DecodeError* type 

The first step I took is to create an enum for errors that can happen during decoding: [`DecodeError`][decode-error-0.1.1]. I created a different variant for each invalid situation that we might encounter while processing the encoded byte array, as outlined in the [section above](#error-types). I tried to follow the [guidelines for error types][prev-post-guidelines], and added the following specifics to the `DecodeError` type: 

* Implemented `std::Error` trait. Since I didn't need any specific behaviour, I used the default implementation. 
* implemented `Display` trait. There's nothing special about this implementation, either: we're simply dumping the error object to the output. This is a lazy implementation: if I had a more interactive UI, I'd have to think more carefully about a user-friendly output. However, that is enough for debugging purposes. 
* Finally, I also implemented `From<Utf8Error>` trait, to cover the cases when the creation of a UTF-8 character from bytes fail. `str::Utf8Error` is returned by [`str::from_utf8()`][from-utf-8-doc] function. Again, for a more user-friendly application, it would be nice to provide more information about where in the encoded text this error occurred, what was the incorrect byte sequence, etc., but for sake of simplicity I decided to take a shortcut and just signal that the creation of _some_ UTF-8 character failed. 

All in all, the current implementation of the `DecodeError` isn't perfect, but it's a good starting point to build a more detailed solution. 

#### Changes to *decode()* function 

With `DecodeError` enum at hand, I had to make modifications to the implementation of the [`decode()`][decode-0.1.1] function. 

First, I changed its return type to `Result<String, DecodeError>`. Next, I went through the code and found all places where an error could happen. This was mostly a mechanical process, thanks to the Rust's error representation. Since Rust makes error situations explicit via the `Result` return type, one can't simply overlook the need to handle errors. Rust forces you to make a decision about what to do with the error. You can use [`unwrap()`][rust-doc-unwrap] or [`expect()`][rust-doc-expect] to force the code to panic at runtime, or you can choose to deal with the error in a more elegant way. 

`unwrap()` had been my tool of choice while I was still developing the algorithm, and now I just needed to go through the code and replace calls to `unwrap()` to bubble up the appropriate instance of `DecodeError`. The biggest effort in this process was to write a proper set of [unit tests][decode-unit-tests] to cover error conditions. 

Finally, I also made the [`main()`][main-0.1.1] function return `Result<(), Box<dyn Error>>`. When an error occurs in `main()`, it returns the error exit code, and prints the debug representation of the error, using the `Debug` trait. 

Notice the use of `Box<dyn Error>` as an error type. This is a kind of a _catch-all_ statement: the code inside `main()` is allowed to return any type of error, as long as it implements `std::Error` trait (that's why implementing `std::Error` for your own error types is a [good practice][prev-post-guidelines]). This is a technique called _opaque error:_ instead of dealing with each error type differently, you treat them all equally and have a unified error handling mechanism, if it's appropriate. In case of `main()`, the behaviour is to simply print the error details to the user using the `Debug` trait, which all implementors of `std::Error` are obliged to implement. Neat! 

The end result is available on GitHub under the [tag 0.1.1][tag-0.1.1].



[encode-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder.rs#L20
[decode-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/decoder.rs#L5
[rust-doc-string]: https://doc.rust-lang.org/rust-by-example/std/str.html
[decode-error-0.1.1]: https://github.com/tindandelion/rust-text-compression/blob/0.1.1/src/decoder/error.rs
[prev-post-guidelines]: {{site.baseurl}}/{% post_url 2025-05-01-tidbits-of-error-handling %}#guidelines
[from-utf-8-doc]: https://doc.rust-lang.org/std/str/fn.from_utf8.html
[rust-doc-unwrap]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap
[rust-doc-expect]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
[decode-0.1.1]: https://github.com/tindandelion/rust-text-compression/blob/0.1.1/src/decoder/decode.rs#L7
[main-0.1.1]: https://github.com/tindandelion/rust-text-compression/blob/0.1.1/src/bin/main.rs#L22
[tag-0.1.1]: https://github.com/tindandelion/rust-text-compression/tree/0.1.1
