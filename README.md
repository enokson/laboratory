# laboratory
A simple, expressive unit test framework for Rust


![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/enokson/laboratory/build/master?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/v/laboratory?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/l/laboratory?style=for-the-badge)


## Features
* before, before_each, after, after_each hooks  
* Different reporter options: spec, minimal, json, json-pretty  
* Different outputs such as to_string and to_result (for continuous integration tests)
* Reports test durations in: nanoseconds, microseconds, milliseconds and seconds  
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
    use laboratory::{describe, it, expect};

    // From Rust's perspective we will only define
    // one test, but inside this test we can define
    // however many test we need.
    #[test]
    fn suite() {

        // let's describe what our add_one function will do.
        // Notice the method "specs" which takes a Vec as it's
        // argument. Inside this vec is where we will define
        // the tests related to add_one.
        describe("add_one()").specs(vec![

            // when describing what it should do, feel free to be
            // as expressive as you would like.
            it("should return 1 when passed 0", |_| {

                // here we will use the default expect function
                // that comes with laboratory.
                // We expect the result of add_one(0) to equal 1
                expect(add_one(0)).to_equal(1)

            }),

            // just as a sanity check, let's add a second test
            it("should return 2 when passed 1", |_| {

                expect(add_one(1)).to_equal(2)

            })

        ]).in_nanoseconds().run();

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



  add_one()
     ✓  should return 1 when passed 0 (910ns)
     ✓  should return 2 when passed 1 (262ns)


  ✓ 2 tests completed (1172ns)




test tests::suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


```

## Nested suites
```rust
// from examples/nested-suites.rs

// Here we will define a struct with
// two members, an associated function,
// and two methods.
struct Foo {
    line: String,
    count: i32
}
impl Foo {
    pub fn new() -> Foo {
        Foo { line: String::new(), count: 0 }
    }
    pub fn append(&mut self, str: &str) {
        self.line += str;
    }
    pub fn increase(&mut self) {
        self.count += 1;
    }
}

fn main () {
    let mut foo = Foo::new();
    foo.append("fizzbuzz");
    foo.increase();
}

#[cfg(test)]
mod tests {

    // Pull the Foo struct into scope
    use super::*;

    // Now pull in the lab tools
    use laboratory::{describe, it, expect};

    // define single test
    #[test]
    fn test() {

        // Now we can describe Foo.
        // Notice the "suites" method that takes a Vec
        // as its argument. This is where we can describe
        // Foo's members and methods.
        describe("Foo").suites(vec![

            // Here we describe the "new" associated function
            describe("#new()").specs(vec![

                it("should return an instance of Foo with two members", |_| {

                    let foo = Foo::new();
                    expect(foo.line).to_be(String::new())?;
                    expect(foo.count).to_equal(0)

                })

            ]),

            // Now we will describe the "append" method
            describe("#append()").specs(vec![

                it("should append \"fizzbuzz\" to Foo#line", |_| {

                    let mut foo = Foo::new();
                    foo.append("fizzbuzz");
                    expect(foo.line).to_be("fizzbuzz".to_string())

                })

            ]),

            // Finally, we will describe the "increase" method
            describe("#increase()").specs(vec![

                it("should increase Foo#count by 1", |_| {

                    let mut foo = Foo::new();
                    foo.increase();
                    expect(foo.count).to_equal(1)

                })

            ])

        ]).in_microseconds().run();

    }

}



```
Result:
```text

running 1 test



  Foo
    #new()
       ✓  should return an instance of Foo with two members (1μs)
    #append()
       ✓  should append "fizzbuzz" to Foo#line (2μs)
    #increase()
       ✓  should increase Foo#count by 1 (0μs)


  ✓ 3 tests completed (4μs)




test tests::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


```

## Failed Tests
```rust
// from examples/failure.rs

// add_one should add one to any number,
// but a typo results in 6 being added
fn add_one (n: i32) -> i32 { n + 6 }

fn add_two (n: i32) -> i32 { n + 2 }

fn main() {
    add_one(0);
    add_two(0);
}

#[cfg(test)]
mod tests {

    // let call in our functions
    use super::*;

    // and now let's bring in our lab tools
    use laboratory::{describe,it,expect};

    // define our single rust test
    #[test]
    fn test() {

        // We have two different functions that we
        // want to test in our crate. So, let's
        // describe our crate and nest our functions
        // under that umbrella.
        describe("Crate").suites(vec![

            describe("add_one()").specs(vec![

                it("should return 1 to when passed 0", |_| {

                    expect(add_one(0)).to_equal(1)

                })

            ]),

            describe("add_two()").specs(vec![

                it("should return 2 to when passed 0", |_| {

                    expect(add_two(0)).to_equal(2)

                })

            ])

        ]).run();

    }

}

```

Result:
```text

running 1 test



  Crate
    add_one()
       0) should return 1 to when passed 0 (0ms)
    add_two()
       ✓  should return 2 to when passed 0 (0ms)


  ✖ 1 of 2 tests failed:

  0) add_one() should return 1 to when passed 0: Expected 6 to equal 1




test tests::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


```

## Hooks
```rust
// from examples/hooks.rs

fn always_return_true() -> bool { true  }

fn main() {
    always_return_true();
}

#[cfg(test)]
mod tests {

    use super::always_return_true;
    use laboratory::{describe, it, expect};

    #[test]
    fn test() {

        // In this suite we want to use hooks to
        // perform actions before and after our tests.
        // The actions we to run in this scenario is simply
        // outputting to to stdout.
        describe("always_return_true")

            // We want to run this action before all
            // all tests in this suite is ran. This action
            // will only be ran once.
            .before_all(|_| {

                println!("\n\n  before_all hook called");
                Ok(())

            })

            // We want to run this action just before every test
            // in this suite. Since we have two tests this action
            // will be ran twice.
            .before_each(|_| {

                println!("  before_each hook called");
                Ok(())

            })

            // likewise, we also have actions we want to run
            // after our tests.
            .after_each(|_| {

                println!("  after_each hook called");
                Ok(())

            }).after_all(|_| {

                println!("  after_all hook called");
                Ok(())

            }).specs(vec![

                it("should return true", |_| {
                    expect(always_return_true()).to_be(true)
                }),

                it("should return true again", |_| {
                    expect(always_return_true()).to_be(true)
                })

            ]).run();

    }

}

```
Result:
```text

running 1 test


  before_all hook called
  before_each hook called
  after_each hook called
  before_each hook called
  after_each hook called
  after_all hook called



  always_return_true
     ✓  should return true (0ms)
     ✓  should return true again (0ms)


  ✓ 2 tests completed (0ms)




test tests::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


```

## Using State
```rust
// from examples/state.rs

fn always_return_true() -> bool { true  }
fn add_one(n: i32) -> i32 { n + 1 }
fn add_two(n: i32) -> i32 { n + 2 }

fn main() {
    let _true = always_return_true();
    let _one = add_one(0);
    let _two = add_two(0);
}

#[cfg(test)]
mod tests {

    use super::*;
    use laboratory::{describe, it, expect, Deserialize, Serialize, State, Error};
    use std::fmt::{Debug};

    // We want a counter to count each time a hook or test is called
    // Note that any state we want to use in the suite
    // must be able to be serialized and deserialized by serde.

    #[derive(Deserialize, Serialize, Debug)]
    struct Counter {
        suite: String, // the name of the suite
        call_count: u8 // the number of times a hook or test was called
    }

    impl Counter {
        fn new(suite: &str) -> Counter {
            Counter {
                suite: String::from(suite),
                call_count: 0
            }
        }
        fn update(&mut self) {
            self.call_count += 1;
            println!("  {} hit count: {}", self.suite, self.call_count);
        }
    }

    #[test]
    fn test() {

        // Here we will define a function to handle all the hook calls
        fn hook_handle(state: &mut State) -> Result<(), Error> {

            // We need to call the get_state method in order to get the counter.
            // We also we to tell the Rust compiler what
            // type the result of get_state will be which
            // in this case is the counter.
            let mut counter = state.get::<Counter>()?;

            // Now we will call the update method on Counter
            counter.update();

            // And if we want to update the state we need to call set_state
            state.set(counter)?;
            Ok(())
        }

        // In this example we want to return the state
        // after all the tests are ran so that we can echo the
        // the final result to stdout.
        let state: Counter = describe("My Crate")

            // We can give the suite the initial state by
            // using the state method, but we could very well
            // skip using the state method and define the state
            // in the before_all or even in the before_each hook.
            .state(Counter::new("Parent Level")).unwrap()

            // Now we will define our hooks
            .before_all(hook_handle)
            .before_each(hook_handle)
            .after_each(hook_handle)
            .after_all(hook_handle)

            .suites(vec![

                // this suite will inherit the parent's state
                describe("add_one()")

                    // Here is the set of hooks for the child suite
                    .before_all(hook_handle)
                    .before_each(hook_handle)
                    .after_each(hook_handle)
                    .after_all(hook_handle)


                    .specs(vec![

                        it("should return 1", |state| {
                            hook_handle(state);
                            expect(add_one(0)).to_be(1)
                        }),

                        it("should return 2", |state| {
                            hook_handle(state);
                            expect(add_one(1)).to_be(2)
                        })

                    ])
                    .inherit_state(),

                // This suite will use its own state
                describe("add_two()")

                    // since this suite will not inherit state
                    // from the parent we will give it a new one.
                    .state(Counter::new("Child Level")).unwrap()

                    // Here is the set of hooks for the second child suite
                    .before_all(hook_handle)
                    .before_each(hook_handle)
                    .after_each(hook_handle)
                    .after_all(hook_handle)
                    .specs(vec![

                        it("should return 2", |state| {
                            hook_handle(state);
                            expect(add_two(0)).to_be(2)
                        }),

                        it("should return 4", |state| {
                            hook_handle(state);
                            expect(add_two(2)).to_be(4)
                        })

                    ]),

                // this suite will also inherit the parent's state
                describe("always_return_true()")

                    // Here is the set of hooks for the child suite
                    .before_all(hook_handle)
                    .before_each(hook_handle)
                    .after_each(hook_handle)
                    .after_all(hook_handle)


                    .specs(vec![

                        it("should always return true", |state| {
                            hook_handle(state);
                            expect(add_one(0)).to_be(1)
                        })

                    ])
                    .inherit_state()


            ]).run().to_state().unwrap();

        println!("{:#?}\n\n", state);

    }

}

```
Result:
```text

running 1 test
  Parent Level hit count: 1
  Parent Level hit count: 2
  Parent Level hit count: 3
  Parent Level hit count: 4
  Parent Level hit count: 5
  Parent Level hit count: 6
  Parent Level hit count: 7
  Parent Level hit count: 8
  Parent Level hit count: 9
  Child Level hit count: 1
  Child Level hit count: 2
  Child Level hit count: 3
  Child Level hit count: 4
  Child Level hit count: 5
  Child Level hit count: 6
  Child Level hit count: 7
  Child Level hit count: 8
  Parent Level hit count: 10
  Parent Level hit count: 11
  Parent Level hit count: 12
  Parent Level hit count: 13
  Parent Level hit count: 14
  Parent Level hit count: 15



  My Crate
    add_one()
       ✓  should return 1 (0ms)
       ✓  should return 2 (0ms)
    add_two()
       ✓  should return 2 (0ms)
       ✓  should return 4 (0ms)
    always_return_true()
       ✓  should always return true (0ms)


  ✓ 5 tests completed (0ms)




Counter {
    suite: "Parent Level",
    call_count: 15,
}


test tests::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


```

## Testing Large Packages Divided by Modules
```rust
// from examples/importing-tests.rs

fn main() {
    add_one::add_one(0);
    multiply_by_two::multiply_by_two(1);
}

// In this crate we have two
// public modules: add_one, and multiply_by_two

pub mod add_one {

    // here is a function that we want to test
    pub fn add_one (x: u64) -> u64 { x + 1 }


    #[cfg(test)]
    pub mod tests {

        use super::*;
        use laboratory::{describe, it, expect, Suite};

        // here is where we will define our suite.
        // Notice that this function returns a Suite struct.
        // Also notice that no other methods are called on this suite.
        pub fn suite() -> Suite {

            describe("add_one()").specs(vec![

                it("should return 1", |_| {
                    expect(add_one(0)).to_equal(1)
                }),

                it("should return 2", |_| {
                    expect(add_one(1)).to_equal(2)
                })

            ])

        }

    }

}

// here is our second module
pub mod multiply_by_two {

    // ...and the function we want to test
    pub fn multiply_by_two (x: u64) -> u64 { x * 2 }

    #[cfg(test)]
    pub mod tests {

        use super::*;
        use laboratory::{describe, it, expect, Suite};

        // Again, we will define a function that returns a Suite struct
        pub fn suite() -> Suite {

            describe("multiply_by_two()").specs(vec![

                it("should return 2", |_| {
                    expect(multiply_by_two(1)).to_equal(2)
                }),

                it("should return 4", |_| {
                    expect(multiply_by_two(2)).to_equal(4)
                })

            ])

        }

    }
}

// Now here is where we will import and run our
// tests under one umbrella of the crate.
#[cfg(test)]
mod tests {

    // pull our modules into scope
    use super::*;

    // pull in our lab tools
    use laboratory::{describe};

    #[test]
    fn test() {

        // Describe the crate.
        describe("My Crate")
            .suites(vec![

                // now we will call our functions that simply
                // returns a Suite struct.
                add_one::tests::suite(),
                multiply_by_two::tests::suite()

            ])

            // Now we can run our tests with any other options
            .run();

    }

}

```
Result:
```text

running 1 test



  My Crate
    add_one()
       ✓  should return 1 (0ms)
       ✓  should return 2 (0ms)
    multiply_by_two()
       ✓  should return 2 (0ms)
       ✓  should return 4 (0ms)


  ✓ 4 tests completed (0ms)




test tests::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


```

## Optional Reporting Styles: Spec, Minimum, JSON, and JSON-Pretty
```rust
// from examples/reporting-json-pretty.rs

fn main() {
    let _one = add_one(0);
    let _two = add_two(0);
}

fn add_one (x: u64) -> u64 { x + 1 }
fn add_two (x: u64) -> u64 { x + 5 }

#[cfg(test)]
mod tests {

    use super::*;
    use laboratory::{describe, it, expect};

    #[test]
    fn suite() {

        // To export to json-pretty we will simply call
        // the json_pretty method on the suite.
        describe("My Crate").suites(vec![

            describe("add_one()").specs(vec![

                it("should return 1", |_| {
                    expect(add_one(0)).to_equal(1)
                }),

                it("should return 2", |_| {
                    expect(add_one(1)).to_equal(2)
                })

            ]),

            describe("add_two()").specs(vec![

                it("should return 2", |_| {
                    expect(add_two(0)).to_equal(2)
                })

            ])

        ]).json_pretty().run();

    }
}
```
Result:
```text

running 1 test

{
  "name": "My Crate",
  "passing": 2,
  "failing": 1,
  "ignored": 0,
  "child_suites": [
    {
      "name": "add_one()",
      "passing": 2,
      "failing": 0,
      "ignored": 0,
      "child_suites": [],
      "child_tests": [
        {
          "name": "should return 1",
          "full_name": "add_one() should return 1",
          "pass": true,
          "error_msg": null,
          "duration": {
            "secs": 0,
            "nanos": 559
          }
        },
        {
          "name": "should return 2",
          "full_name": "add_one() should return 2",
          "pass": true,
          "error_msg": null,
          "duration": {
            "secs": 0,
            "nanos": 246
          }
        }
      ],
      "duration": {
        "secs": 0,
        "nanos": 805
      }
    },
    {
      "name": "add_two()",
      "passing": 0,
      "failing": 1,
      "ignored": 0,
      "child_suites": [],
      "child_tests": [
        {
          "name": "should return 2",
          "full_name": "add_two() should return 2",
          "pass": false,
          "error_msg": {
            "Assertion": "Expected 5 to equal 2"
          },
          "duration": {
            "secs": 0,
            "nanos": 1428
          }
        }
      ],
      "duration": {
        "secs": 0,
        "nanos": 1428
      }
    }
  ],
  "child_tests": [],
  "duration": {
    "secs": 0,
    "nanos": 2233
  }
}


test tests::suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


```