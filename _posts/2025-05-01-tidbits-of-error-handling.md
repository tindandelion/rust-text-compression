---
layout: post
title: Tidbits of error handling in Rust
date: 2025-05-01
---

Error handling in Rust is well documented in [multiple sources][rust-book-errors], so let's not waste space on obvious things. In this post, I'd like to share a few things I've learned on top of the basics. 

# Error trait **std::Error**
{: #guidelines }

When you define custom error types, it's a good practice to make them implement the `std::Error` trait. This makes your error type fit nicely into the rest of Rust ecosystem: `std::Error` is a kind of a *catch-all* case for all error types.

Implementing `Error` trait requires you to implement `Debug` and `Display` traits as well. Luckily, that doesn't require a lot of coding in common cases. The default implementation of `Error` and `#[derive(Debug)]` are sufficient to comply. The only piece of code you'll need to write yourself is the implementation of `Display::fmt()` method. This method is supposed to provide a *user-friendly* information about what went wrong: it's up to you as an application developer to decide what will be shown to the user.   

There are no specific guidelines about how `Display::fmt()` should be implemented. Jon Gjengset gives the following advice in his book ["Rust for Rustaceans"][rust-for-rustaceans], in the chapter about error handling: 

> In general, your implementation of Display should give a one-line description of what went wrong that can easily be folded into other error messages. The display format should be lowercase and without trailing punctuation so that it fits nicely into other, larger error reports.

Another interesting method in `std::Error` is `source()`. The default implementation simply returns `None`. It is useful if your error type is a wrapper around another error: in this case, you can override this method to give access to the inner error. 

# Syntax sugar: **?** operator

The `?` operator is commonly used in Rust to avoid repetitive boilerplate code related to error propagation. It's a shorthand for *unwrap the successful result or return the error early*, in cases when you simply need to abort the function and return the error to the caller. 

One neat trick that `?` does under the hood is converting from one error type to another. Let's say, I'm writing a function `outer_func() -> Result<(), OuterError>`, and it calls another function, `inner_func() -> Result<(), InnerError>`: 

```rust 
fn outer_func() -> Result<(), OuterError> {
    // ... do some work 
    let x = inner_func()? 
    // ... do some other work 
}

fn inner_func() -> Result<(), InnerError> {
    // ...
}
```

The `?` operator will convert `InnerError` into `OuterError` for me. But it's not magic, of course: in order to make the conversion work, I need to implement a `From` trait on my `OuterError` to specify _exactly how_ I can create the `OuterError` from `InnerError`: 

```rust 
struct OuterError {
    // ...
}; 

impl From<InnerError> for OuterErorr { 
    fn from(error: InnerError) -> Self {
        // Construct OuterError from InnerError
    }
}
```

Another tidbit is that `?` is not limited only to errors: it works with `Option` type also. In case of `Option`, it's behaviour can be described as *unwrap the Option or return None early*. 

Once we see it working for `Result` and `Option`, a question arises: can it handle my own types as well? Interestingly, the `?` operator uses the [`Try`][try-trait] trait behind the scenes. Currently, this feature is still experimental, but once it stabilizes, it will be possible to make your own types `?`-compliant by implementing this trait. 

# Try blocks 

Yet another **experimental feature** I've learned about is [`try blocks`][try-blocks]. 

Sometimes an early return with `?` operator can backfire. Consider the following example: 

```rust
fn query_database() -> Result<i32, Error> {
    let conn = Database::connect()?; // Acquire connection 

    let x = query_value(conn)?; // Early return on error
    let y = x + 10;

    conn.close(); // Release connection
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

# Moving on 

Now that we're equipped with a bit of knowledge about error handling in Rust, I'm going to move on to [implementing proper error handling][next-post] in my project.

[rust-book-errors]: https://doc.rust-lang.org/book/ch09-00-error-handling.html
[rust-for-rustaceans]: https://rust-for-rustaceans.com/
[try-trait]: https://doc.rust-lang.org/std/ops/trait.Try.html
[try-blocks]: https://doc.rust-lang.org/beta/unstable-book/language-features/try-blocks.html
[next-post]: {{site.baseurl}}/{% post_url 2025-05-02-handling-invalid-inputs %}
