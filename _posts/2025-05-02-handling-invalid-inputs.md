---
layout: post
title: Handling invalid inputs 
date: 2025-05-02
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

* invalid UTF-8 codes in un-encoded parts of the text; 
* invalid substring index: the entry is missing from the encoding table; 
* substring index is missing: the encoded string ends with the encoded "marker" byte (`0xF5..0xFF`), but there's no follow-up "index" byte `NN`. 

These are the situations that should be accounted for when decoding the input byte array. 

# Changes to the code 

#### *DecodeError* type 

The first step I took is to create an enum for errors that can happen during decoding: [`DecodeError`][decode-error-0.1.1]. I created a different variant for each invalid situation that we might encounter while processing the encoded byte array, as outlined in the [section above]. I tried to follow the [guidelines for error types][prev-post-guidelines], and added the following specifics to the `DecodeError` type: 

* implemented `std::Error` trait. Since I didn't need any specific behaviour, we use the default implementation. 
* implemented `Display` trait. There's nothing special about this implementation, either: we're simply dumping the error object to the output. This is a lazy implementation: if I had a more interactive UI, I'd have to think more carefully about more user-friendly output. However, this simple implementation is enough for debugging purposes. 
* finally, I also implemented `From<Utf8Error>` trait, to cover the cases when the creation of a UTF-8 character from bytes fail. `str::Utf8Error` is returned by [`str::from_utf8()`][from-utf-8-doc] function. Again, for a more user-friendly application, it would be nice to provide more information about where in the encoded text this error occurred, what was the incorrect byte sequence, etc., but for the purposes of this learning project I decided to take a shortcut and simply signal that the creation of _some_ UTF-8 character failed. 

One may notice that there's a lot of possible improvements here in terms of what information should be included into the error, and how it should be reported to the user. For example, I could have provided the data where in the encoded sequence the error occurred, what was the input that couldn't be converted into UTF-8 character, etc. I agree with these concerns. However, I think that developing a proper error handling is an iterative process: you can start with some basic implementation, and refine it later on. Proper manual testing plays an important role in this process: 

* from the user perspective, you should validate that the error was correctly shown in the UI; 
* as a developer, you should have enough information in the internal logs to understand where an error occurred and what conditions caused it. 

Current implementation of the `DecodeError` is a good starting point to build a more sophisticated solution. 

#### Changes to *decode()* function 





[encode-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/encoder.rs#L20
[decode-0.1.0]: https://github.com/tindandelion/rust-text-compression/blob/0.1.0/src/decoder.rs#L5
[rust-doc-string]: https://doc.rust-lang.org/rust-by-example/std/str.html
[decode-error-0.1.1]: https://github.com/tindandelion/rust-text-compression/blob/0.1.1/src/decoder/error.rs
[prev-post-guidelines]: {{site.baseurl}}/{% post_url 2025-05-01-tidbits-of-error-handling %}#guidelines
[from-utf-8-doc]: https://doc.rust-lang.org/std/str/fn.from_utf8.html

