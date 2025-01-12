Got some bugs to fix 

# Inclusion of single-occurrence strings 

That one I spotted while analyzing the subsrings found by the algorithm. I went went through the list of top 5 most impactful substrings, and found that the following substring occurs only once in the original text:

```
"follow.\n                                                         Exeunt.\n\n\n\n\n"
```

Indeed, that's an error. It doesn't make sense to extract a substring that occurs only once in the original text. A [simple test][single-occurrence-test] revealed the problem:

```rust
#[test]
fn most_impactfult_strings_skip_single_occurence() {
    let mut dict = SubstringLedger::new();
    dict.insert_new(substring("aaaaaa"));

    dict.insert_new(substring("bb"));
    dict.increment_count(&substring("bb"));

    let most_impactful = dict.get_most_impactful_strings(&EncoderSpec {
        num_strings: 10,
        encoded_size: 1,
    });
    assert_eq!(vec!["bb"], most_impactful.to_vec());
}
```

# Encoding strings with multi-byte characters

That bug revealed itself when I decided to switch to another test data source. "Hamlet" turned out to be too short, so I decided to switch to the longest piece of prose I knew: _War and Peace_ by Leo Tolstoy. I grabbed the text from [Project Gutenberg][gutenberg-war-and-peace], and started preparing the test data from it. 

Error text:

```
File name: wap-100.txt
thread 'main' panicked at src/encoder/encode_string.rs:23:29:
byte index 1 is not a char boundary; it is inside '\u{feff}' (bytes 0..3) of `The Project Gutenberg eBook of War and Peace
    
This ebook is for the use of anyone anywhere in the United States and
most other parts of the world at no cost and with almost no restrictions
whatsoever. You may copy it, give it away or re-use it u`[...]
```

TODO: Need some explanation here about strings & bytes.

The test code to reveal the bug: 

```rust
#[test]
fn encode_multibyte_string() {
    let source = "こんにちはこんにちは世界世界";

    let encoded = encode_string(&source, &make_dictionary(vec![]));
    assert_eq!(source.as_bytes(), encoded);
}
```

TODO: Link to the fix 

# Skipping the substrings 

The last bug I want to taclke is actually in the heart of the algorithm, the [`build_ledger` function][build-ledger]. I acutally spotted it while preparing one of the [previous posts].

The bug manifests itself in such a way that the algorithm is skipping some of the substrings from merging. In particualr, it goes to find the matching the substring at the start of the text, and then tries to find a follow-up match. If the follow-up is found, we merge these to substrings, and then jump to the rest of the text after the second match. But, we're effectively skipping the second match to be merged with what may come after it! 

To give you an example, let's consider the folling test case. Suppose at some step we have a `DictionaryLedger` containing the following substrings: `["ca", "me", "lot"]`, and the text we're processing next is `"camelot"`. We find the first match, `"ca"`, and then try to find a follow-up match. We find `"me"`, and merge these two substrings into `"came"`. We then jump to the rest of the text after the second match, and go to process `"lot"`. But, we don't try to merge `"me"` with `"lot"`! 

Putting it all together into the test code that goes through the process step by step:

```rust
#[test]
fn merge_three_consecutive_substrings() {
    let mut ledger = SubstringLedger::new();
    ledger.insert_new(substring("ca"));
    ledger.insert_new(substring("me"));
    ledger.insert_new(substring("lot"));

    let source = "camelot";

    // Processing "ca" + "me" = "came"
    let next_head = build_ledger_step(source, &mut ledger);
    assert_eq!(
        vec![("came", 1), ("lot", 1), ("ca", 2), ("me", 1)],
        ledger.entries()
    );

    // Processing "me" + "lot" = "melot"
    let next_head = build_ledger_step(next_head, &mut ledger);
    assert_eq!(
        vec![("melot", 1), ("came", 1), ("lot", 1), ("ca", 2), ("me", 2)],
        ledger.entries()
    );

    // Processing "lot"
    build_ledger_step(next_head, &mut ledger);
    assert_eq!(
        vec![("melot", 1), ("came", 1), ("lot", 2), ("ca", 2), ("me", 2)],
        ledger.entries()
    );
}
```

I had to extract the `build_ledger_step` function from the `build_ledger` function, so that I can easily create the test case preconditions, and apply the fix to make the test pass.  

```rust
pub fn build_ledger(source: &str) -> SubstringLedger {
    let mut ledger = SubstringLedger::new();
    let mut head: &str = source;

    while head.len() > 0 {
        head = build_ledger_step(head, &mut ledger);
    }
    ledger
}

fn build_ledger_step<'a>(head: &'a str, ledger: &mut SubstringLedger) -> &'a str {
    if let Some(next_char) = head.chars().next() {
        let next_head: &'a str;

        if let Some(substr_match) = ledger.find_longest_match(head) {
            let rest = &head[substr_match.len()..];

            if let Some(follow_up_match) = ledger.find_longest_match(rest) {
                let new_substring = substr_match.concat(&follow_up_match);
                next_head = &head[substr_match.len()..];
                ledger.insert_new(new_substring);
            } else {
                next_head = rest;
            }

            ledger.increment_count(&substr_match);
        } else {
            let new_substring = Substring::from_char(next_char);
            next_head = &head[new_substring.len()..];
            ledger.insert_new(new_substring);
        }
        next_head
    } else {
        ""
    }
}
```

Now with the fix, the test passes, but I'm not very happy with the state of the code. For one, the nesting level in `build_ledger_step` is too much. The second thing I noticed is the obvious inefficiency in the algorithm: we're calling `find_longest_match` twice for the same substring. It's done the first time to find the follow-up match, and then in the next step we call it again for the same substring. As the next step, I'm going to refactor the code to make it more readable and efficient. 

[gutenberg-war-and-peace]: https://www.gutenberg.org/cache/epub/2600/pg2600.txt



