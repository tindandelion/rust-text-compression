---
layout: post
title: Refactoring a messy function 
date: 2025-01-19
---

As I mentioned in the [previous post][previous-post], I'm not very happy with the current implementation of the [`build_ledger_step`][build-ledger-step-current] function. Today I discuss what my concerns were, and describe the refactoring process that I went through to address them. 

# The problems with _build_ledger_step()_

Let's have a look at the [current implementation][build-ledger-step-current] of this function:

```rust
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
It's not particularly scary (yet), but it's not very readable either. I don't like that it's overloaded with too much low-level details, and it's not very easy to see the big picture behind it: the outline of the algorithm itself. As a side-effect, there are also some code redundancies, and possibly bugs as well, that are obscured by these low-level details. 

What I would like to have instead, is that this function should lay out the high-level structure of the algorithm, and the details should be hidden away from the sight into smaller functions. 

Let's do it in a series of refactoring steps. Since I have a set of unit tests, I'm not scared of breaking the code in the process, and I can do the refactoring in small increments, making sure that tests pass at each step. 

# Step 1: Split logic into smaller functions 

The first thing I want to do is tinker with the inner `if` statement. Personally, I prefer to keep `if` branches short. You should be able to grasp the whole `if` statement at a glance, and see right away what the condition is, and what is done in each branch. Ideally, each branch should be a one-liner, but it's ok if they contain a few lines, as long as you don't have to spend too much time parsing out what's going on. 

So I went on and extracted the code from each branch into separate functions, and named them after what they do: 

```rust 
fn build_ledger_step<'a>(head: &'a str, ledger: &mut SubstringLedger) -> &'a str {
    if let Some(next_char) = head.chars().next() {
        if let Some(substr_match) = ledger.find_longest_match(head) {
            ledger.increment_count(&substr_match);
            merge_with_follow_up_match(head, ledger, &substr_match)
        } else {
            create_single_char_substring(head, ledger, next_char)
        }
    } else {
        ""
    }
}

fn merge_with_follow_up_match<'a>(
    head: &'a str,
    ledger: &mut SubstringLedger,
    substr_match: &Substring,
) -> &'a str {
    let rest = &head[substr_match.len()..];
    if let Some(follow_up_match) = ledger.find_longest_match(rest) {
        let new_substring = substr_match.concat(&follow_up_match);
        ledger.insert_new(new_substring);
    }
    rest
}

fn create_single_char_substring<'a>(
    head: &'a str,
    ledger: &mut SubstringLedger,
    next_char: char,
) -> &'a str {
    let new_substring = Substring::from_char(next_char);
    let rest = &head[new_substring.len()..];
    ledger.insert_new(new_substring);
    rest
}
```

Now the high-level outline of the algorithm is much more obvious. The additional benefit was that, once I extracted the code into smaller functions, I immediately noticed a few redundancies in the code, that otherwise were buried in the details. 

# Step 2: Introduce _BuildState_ struct

The second thing that popped out to me at once was that these functions had similarities in their parameter lists. It looked like `head` and `ledger` were always passed around together. That's a hint that there's a hidden abstraction waiting to be revealed.

I've created a new struct `BuildState` to couple `head` and `ledger`, and then I've changed the functions to take `BuildState` as a parameter. It also looked natural to me to make those functions return a new `BuildState` with the updated `head` and `ledger`. It's more of a matter of preference; the `state` could be modified in-place, but it was easier for me to reason about the code this way.

```rust
struct BuildState<'a> {
    head: &'a str,
    ledger: SubstringLedger,
}

fn build_ledger_step<'a>(mut state: BuildState<'a>) -> BuildState<'a> {
    if let Some(next_char) = state.head.chars().next() {
        if let Some(substr_match) = state.ledger.find_longest_match(state.head) {
            state.ledger.increment_count(&substr_match);
            merge_with_follow_up_match(state, &substr_match)
        } else {
            create_single_char_substring(state, next_char)
        }
    } else {
        BuildState {
            head: "",
            ledger: state.ledger,
        }
    }
}

fn merge_with_follow_up_match<'a>(
    mut state: BuildState<'a>,
    substr_match: &Substring,
) -> BuildState<'a> {
    let rest = &state.head[substr_match.len()..];
    if let Some(follow_up_match) = state.ledger.find_longest_match(rest) {
        let new_substring = substr_match.concat(&follow_up_match);
        state.ledger.insert_new(new_substring);
    }

    BuildState {
        head: rest,
        ledger: state.ledger,
    }
}

fn create_single_char_substring<'a>(
    mut state: BuildState<'a>,
    next_char: char,
) -> BuildState<'a> {
    let new_substring = Substring::from_char(next_char);
    let rest = &state.head[new_substring.len()..];
    state.ledger.insert_new(new_substring);
    BuildState {
        head: rest,
        ledger: state.ledger,
    }
}
```

# Step 3: Make functions to be methods of _BuildState_

Now that I have a `BuildState` struct, it becomes a convenient place to put methods that operate on its parts, so I convert all previous functions to be methods of `BuildState`. 

```rust
struct BuildState<'a> {
    head: &'a str,
    ledger: SubstringLedger,
}

impl<'a> BuildState<'a> {
    fn new(head: &'a str) -> Self {
        Self {
            head,
            ledger: SubstringLedger::new(),
        }
    }

    fn at_end(&self) -> bool {
        self.head.len() == 0
    }

    fn step(mut self) -> BuildState<'a> {
        if let Some(next_char) = self.head.chars().next() {
            if let Some(substr_match) = self.ledger.find_longest_match(self.head) {
                self.ledger.increment_count(&substr_match);
                self.merge_with_follow_up_match(&substr_match)
            } else {
                self.create_single_char_substring(next_char)
            }
        } else {
            self.make_end_state()
        }
    }

    fn merge_with_follow_up_match(mut self, substr_match: &Substring) -> BuildState<'a> {
        let rest = &self.head[substr_match.len()..];
        if let Some(follow_up_match) = self.ledger.find_longest_match(rest) {
            let new_substring = substr_match.concat(&follow_up_match);
            self.ledger.insert_new(new_substring);
        }

        BuildState {
            head: rest,
            ledger: self.ledger,
        }
    }

    fn create_single_char_substring(mut self, next_char: char) -> BuildState<'a> {
        let new_substring = Substring::from_char(next_char);
        let rest = &self.head[new_substring.len()..];
        self.ledger.insert_new(new_substring);
        BuildState {
            head: rest,
            ledger: self.ledger,
        }
    }

    fn make_end_state(self) -> BuildState<'a> {
        BuildState {
            head: "",
            ledger: self.ledger,
        }
    }
}
```

By now, I'm much more satisfied with the code, and I'm ready to work on the last thing that bothered me: the redundant `find_longest_match` calls. 

# Tackle the inefficient lookup 

As I described in the [previous post][previous-post], there is an inefficiency in the implementation: in many cases we search for the same substring twice. First we do it when we find a follow-up match to merge with. Then at the next step, we search for the same substring again, but now from the start of the text. It seems we could get rid of this redundancy if we simply reused that result in the next step. Luckily, now we have a place to keep it and pass between the steps: in the `BuildState` struct. 

So I've added a `last_match: Option<Substring>` field to the `BuildState` struct. In case when the follow-up match is found, we store it in this field and pass it to the next step. If the follow-up match is not found, we set it to `None`. Then at the next step, we can reuse this value immediately if it's present. 

I'm not posting the result code for `BuildState` here to keep the post short. You can find it in [GitHub][version-src-link], as always. 

[previous-post]: {{site.baseurl}}/{% post_url 2025-01-18-bug-fixes %}
[build-ledger-step-current]: https://github.com/tindandelion/rust-text-compression/blob/0.0.4/src/encoder/build_ledger.rs#L13
[version-src-link]: https://github.com/tindandelion/rust-text-compression/tree/0.0.5