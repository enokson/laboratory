# laboratory
A unit test runner for Rust

## Features
The use of hooks such as before, before_each, after, after_each   
Different reporter options  
Reports test durations  
The use of custom assertion libraries  
Exclude tests  
Nested test suites  
The use of state

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
### Testing a simple function "add_one"
```rust

// the functio to test
fn add_one(n: i32) -> i32 {
    n + 1
}

#[cfg(test)]
mod tests {
    
    // import testing functions
    use laboratory::{describe, it, expect};

    // define a test
    #[test]
    fn suite() {

        describe("add_one()")
            .specs(vec![
                
                it("should return 2 when given 1", || {
                    expect(add_one(1)).to_equal(2)?;
                    Ok(())
                })

            ]).run();

    }
}
```

Then run: 
```shell script
$ cargo test -- --nocapture
```

Result:
```rust
//  add_one()
//     ✓  should return 2 when given 1 (0ms)
//
//
//  ✓ 1 tests completed (0ms)
```