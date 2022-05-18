# Rust-Assertions

## What is it?
A library with assertion macros to control when some assertions are being compiled or run.
This is meant to allow filling the code with many debug_assertions without taking a huge performance hit during testing.
So instead of `debug_assert!(true)` you can use a trace assert `tassert!(true)` which will only compile to a check if 
you enable trace asserts.

## Motivation
As in any language, but especially in rust, debugging can be a bit of a pain. 
Writing special debug prints can dirty up your code and make it harder to maintain while having to redo your debugging
work each time is expensive. So we usually have logging prints that make sense, and maybe an assertion here or there.

I started to change how I work,and I am dubbing it "invariant debugging".
In addition to logging, and instead of debugging expressions (which afaik is not possible in rust), I use assertions.
You define invariants during development, and then you can use assertions to check them. 
This way it is both used for testing and documentation. 
For example in a hashset implementation you could add the following assertion:

```rust
pub fn add(key) {
    ...
    let res = self.data.insert(key, value);
    debug_assert!({
        let mut set = vec![];
        for e in v.iter() {
            if !set.contains(e) {
                set.push(0);
            }
        }
        set.len() == self.len()
            }, "HashSet has a non-unique key");
    res
}
```

This is not an efficient way to check for duplicate. 
And even worse some of our tests will be very slow.
Usually will comment this out or delete this after we fixed our bug, but it is still a good idea to have it.
Instead when using this macro you can use `tassert!`, and only  so you will get a similar piece of code:

```rust
pub fn add(key) {
    ...
    let res = self.data.insert(key, value);
    tassert!({...}, "HashSet has a non-unique key");
    res
}
```

But with this you can turn on trace asserts only in some paths or some tests.

## How to use it?

```rust
 iassert!(true, "This is always true");
 wassert!(true, "This is always true");
 tassert!(false, "Opps, this doesn't work when used in trace level");
```

Currently, we only support using the log level as the assertion level.
In the near future we will add support for other nice features such as custom assertion levels,
dynamic assertion levels, and procedural macros to define assertion functions.

## Installation

```toml
[dependencies]
assertions = "0.1"
```