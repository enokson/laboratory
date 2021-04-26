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
fn main() {
    add_one(0);
}

// Here we have one function that does
// one thing: Adds one to whatever number
// we pass to it.
fn add_one (n: u64) -> u64 { n + 1 }

#[cfg(test)]
mod tests {

    // lets pull our add_one function into scope
    use super::*;

    // now let's pull in our lab tools into scope
    // to test our function
    use laboratory::{describe, expect, LabResult, NullState};

    // From Rust's perspective we will only define
    // one test, but inside this test we can define
    // however many tests we need.
    #[test]
    fn suite() -> LabResult {

        // let's describe what our add_one() function will do.
        // The describe function takes a closure as its second
        // argument. And that closure also takes an argument which
        // we will call "suite". The argument is the suite's context
        // and it allows for extensive customizations. The context struct
        // comes with a method called it() and using this method we can
        // define a test.
        describe("add_one()", |suite| {

            // when describing what it should do, feel free to be
            // as expressive as you would like.
            suite.it("should return 1 when passed 0", |_| {

                // here we will use the default expect function
                // that comes with laboratory.
                // We expect the result of add_one(0) to equal 1
                expect(add_one(0)).to_equal(1)

            })

            // just as a sanity check, let's add a second test
            .it("should return 2 when passed 1", |_| {

                expect(add_one(1)).to_equal(2)

            });

        }).state(NullState).milis().run()

    }
}
```

Then run: 
```shell script
$ cargo test -- --nocapture
```

Result:  
```

running 1 test


### Lab Results Start ###

add_one()
  ✓  should return 1 when passed 0 (0ms)
  ✓  should return 2 when passed 1 (0ms)
✓ 2 tests completed (0ms)

### Lab Results End ###


test tests::suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```
