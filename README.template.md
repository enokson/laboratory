# laboratory
A simple, expressive unit test framework for Rust


![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/enokson/laboratory/build/master?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/v/laboratory?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/l/laboratory?style=for-the-badge)


## Features
* before, before_each, after, after_each hooks  
* Different reporter options  
* Different outputs such as to_string and to_result (for continuous integration tests)
* Reports test durations  
* The use of custom assertion libraries  
* Exclude tests  
* Nested test suites  
* The use of state  
* "should panic" testing  
* Console highlighting

## Installation
In Cargo.toml:
```toml
[dev-dependencies]
laboratory = "*"
```
Then in your test files
```rust
#[cfg(test)]
mod tests {
    use laboratory::{describe, describe_skip, it, it_skip, it_only, expect};
}
```

## Getting Started
### Testing a simple function "add_one()"
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

## Nested suites
```rust
// from examples/nested-suites.rs
//###NESTED###//
```
Result:
```text
//###NESTED-RESULT###//
```

## Failed Tests
```rust
// from examples/failure.rs
//###FAILURE###//
```

Result:
```text
//###FAILURE-RESULT###//
```

## Hooks
```rust
// from examples/hooks.rs
//###HOOKS###//
```
Result:
```text
//###HOOKS-RESULT###//
```

## Using State
```rust
// from examples/state.rs
//###STATE###//
```
Result:
```text
//###STATE-RESULT###//
```

## Testing Large Packages Divided by Modules
```rust
// from examples/importing-tests.rs
//###IMPORT###//
```
Result:
```text
//###IMPORT-RESULT###//
```

## Optional Reporting Styles: Spec, Minimum, JSON, and JSON-Pretty
```rust
// from examples/reporting-json-pretty.rs
//###REPORT###//
```
Result:
```text
//###REPORT-RESULT###//
```