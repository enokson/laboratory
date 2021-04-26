# laboratory
A simple, expressive unit test framework for Rust



![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/enokson/laboratory/build/master?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/v/laboratory?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/l/laboratory?style=for-the-badge)

Checkout the [documentation](https://enokson.github.io/laboratory/) and the extensive [examples](https://github.com/enokson/laboratory/tree/master/examples) over on github.

Laboratory is layer 2 test runner solution that sits on top of the Rust test runner to provide unparalleled features and ease of use.

## Features
* before_all, before_each, after_all, after_each hooks  
* Different reporter options: spec, minimal, json, json-pretty, rust, dot, tap, list
* Reports test durations in: nanoseconds, microseconds, milliseconds and seconds  
* The use of custom assertion libraries  
* Exclude tests  
* Nested test suites  
* Test retry support
* The use of state  
* "should panic" testing
* Console highlighting
* Dynamic testing
* Highlights slow tests
* No weird macros to try to figure out or debug!
* Human readable code and test results

## Installation
In Cargo.toml:
```toml
[dev-dependencies]
laboratory = "2.0.0"
```
Then in your test files
```rust
#[cfg(test)]
mod tests {
    use laboratory::{describe, describe_skip, it, it_skip, it_only, expect};
}
```

## Getting Started
### Testing a simple function
```rust
// from examples/simple.rs
//###SIMPLE###//
```

Then run: 
```shell script
$ cargo test -- --nocapture
```

Result:  
```
//###SIMPLE-RESULT###//
```
