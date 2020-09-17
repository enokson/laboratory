# laboratory
A simple, expressive unit test framework for Rust


![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/enokson/laboratory/build/master?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/v/laboratory?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/l/laboratory?style=for-the-badge)


## Features
* before, before_each, after, after_each hooks  
* Different reporter options  
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
                // comes with laboratory.
                // We expect the result of add_one(0) to equal 1
                expect(add_one(0)).to_equal(1)

            }),

            // just as a sanity check, let's add a second test
            it("should return 2 when passed 1", |_| {

                expect(add_one(1)).to_equal(2)

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
```
running 1 test



  add_one()
     ✓  should return 1 (0ms)
     ✓  should return 2 (0ms)


  ✓ 2 tests completed (0ms)




test tests::suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Nested suites
```rust

struct Foo {
    bar: String,
    baz: i32
}

impl Foo {
    pub fn new() -> Foo {
        Foo { bar: String::new(), baz: 0 }
    }
    pub fn append(&mut self, str: &str) {
        self.bar += str;
    }
    pub fn increase(&mut self) {
        self.baz += 1;
    }
}

fn main () {
    let mut foo = Foo::new();
    foo.append("fizzbuzz");
    foo.increase();
}

#[cfg(test)]
mod tests {

    // get the test suite
    use laboratory::{describe, it, expect};

    // get the Foo struct
    use super::*;

    // define single test
    #[test]
    fn test() {

        describe("Foo").suites(vec![

            describe("#new()").specs(vec![

                it("should return foo with two members", |_| {

                    let result = Foo::new();
                    expect(result.bar).to_be(String::new())?;
                    expect(result.baz).to_equal(0)?;
                    Ok(())

                })

            ]),

            describe("#append()").specs(vec![

                it("should append fizzbuzz to Foo#bar", |_| {
                    let mut foo = Foo::new();
                    foo.append("fizzbuzz");
                    expect(foo.bar).to_be("fizzbuzz".to_string())
                })

            ]),

            describe("#increase()").specs(vec![

                it("should increase Foo#baz by 1", |_| {
                    let mut foo = Foo::new();
                    foo.increase();
                    expect(foo.baz).to_equal(1)
                })

            ])

        ])
        .run();

    }

}

```
Result:
```textmate
running 1 test



  Foo
    #new()
       ✓  should return foo with two members (0ms)
    #append()
       ✓  should append fizzbuzz to Foo#bar (0ms)
    #increase()
       ✓  should increase Foo#baz by 1 (0ms)


  ✓ 3 tests completed (0ms)




test tests::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Failed Tests
```rust

// add_one should add one to any number, but a typo results in 6 being added
fn add_one (n: i32) -> i32 { n + 6 }

fn add_two (n: i32) -> i32 { n + 2 }

fn main () {
    add_one(0);
    add_two(0);
}

#[cfg(test)]
mod tests {

    use laboratory::{describe,it,expect};
    use super::*;

    #[test]
    fn test() {

        describe("Package").suites(vec![

            describe("add_one()").specs(vec![

                it("should return 1 to when given 0", |_| {
                    expect(add_one(0)).to_equal(1)
                })

            ]),

            describe("add_two()").specs(vec![

                it("should return 2 to when given 0", |_| {
                    expect(add_two(0)).to_equal(2)
                })

            ])

        ]).run();

    }

}

```

Result:
```textmate

running 1 test



  Package
    add_one()
       0) should return 1 to when given 0 (0ms)
    add_two()
       ✓  should return 2 to when given 0 (0ms)


  ✖ 1 of 2 tests failed:

  0) add_one() should return 1 to when given 0: Expected 6 to equal 1





test tests::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

## Hooks
```rust

fn no_op() -> bool { true  }

fn main() { no_op(); }

#[cfg(test)]
mod tests {

    use super::no_op;
    use laboratory::{describe, it, expect};

    #[test]
    fn test() {

        describe("no_op").before_all(|_| {
            println!("\n\n  before hook called");
        }).before_each(|_| {
            println!("  before_each hook called");
        }).after_each(|_| {
            println!("  after_each hook called");
        }).after_all(|_| {
            println!("  after_all hook called");
        }).specs(vec![

            it("should do nothing", |_| {
                expect(no_op()).to_be(true)
            }),

            it("should do nothing again", |_| {
                expect(no_op()).to_be(true)
            })

        ]).run();

    }

}

```
Result:
```
running 1 test


  before hook called
  before_each hook called
  after_each hook called
  before_each hook called
  after_each hook called
  after_all hook called



  no_op
     ✓  should do nothing (0ms)
     ✓  should do nothing again (0ms)


  ✓ 2 tests completed (0ms)




test tests::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Using State
```rust

fn no_op() -> bool { true  }
fn add_one(n: i32) -> i32 { n + 1 }
fn add_two(n: i32) -> i32 { n + 2 }

fn main() {
    no_op();
    add_one(0);
}

#[cfg(test)]
mod tests {

    use super::*;
    use laboratory::{describe, it, expect, Deserialize, Serialize, State};
    use std::fmt::{Debug};
    use crate::add_one;

    // We want a counter to count each time a hook or test is called
    // Any state we want to use in the suite must be able to be serialized and deserialized by serde
    #[derive(Deserialize, Serialize, Debug)]
    struct Counter {
        // the counter will hold a member for each category
        pub before_all_hit_count: u8,
        pub before_each_hit_count: u8,
        pub after_each_hit_count: u8,
        pub after_all_hit_count: u8,
        pub test_hit_count: u8
    }
    impl Counter {
        fn new() -> Counter {
            Counter {
                before_all_hit_count: 0,
                before_each_hit_count: 0,
                after_each_hit_count: 0,
                after_all_hit_count: 0,
                test_hit_count: 0
            }
        }
    }

    #[test]
    fn test() {

        fn update_before_all(state: &mut State) {
            let mut counter: Counter = state.get_state();
            counter.before_all_hit_count += 1;
            state.set_state(counter);
        }
        fn update_before_each(state: &mut State) {
            let mut counter: Counter = state.get_state();
            counter.before_each_hit_count += 1;
            state.set_state(counter);
        }
        fn update_after_each(state: &mut State) {
            let mut counter: Counter = state.get_state();
            counter.after_each_hit_count += 1;
            state.set_state(counter);
        }
        fn update_after_all(state: &mut State) {
            let mut counter: Counter = state.get_state();
            counter.after_all_hit_count += 1;
            // println!("\n\n{:#?}", &counter);
            state.set_state(counter);
        }
        fn update_test(state: &mut State) {
            let mut counter: Counter = state.get_state();
            counter.test_hit_count += 1;
            state.set_state(counter);
        }

        let state: Counter = describe("no_op")
            .state(Counter::new())
            .before_all(update_before_all)
            .before_each(update_before_each)
            .after_each(update_after_each)
            .after_all(update_after_all)
            .suites(vec![

                // this suite will inherit the parent suite's state
                describe("add_one()")
                    .before_all(update_before_all)
                    .before_each(update_before_each)
                    .after_each(update_after_each)
                    .after_all(update_after_all)
                    .specs(vec![

                        it("should return 1", |state| {
                            update_test(state);
                            expect(add_one(0)).to_be(1)
                        }),

                        it("should return 2", |state| {
                            update_test(state);
                            expect(add_one(1)).to_be(2)
                        })

                    ])
                    .inherit_state(),

                // it will use its own state
                describe("add_two()")
                    .state(Counter::new())
                    .before_all(update_before_all)
                    .before_each(update_before_each)
                    .after_each(update_after_each)
                    .after_all(update_after_all)
                    .specs(vec![

                        it("should return 2", |state| {
                            update_test(state);
                            expect(add_two(0)).to_be(2)
                        }),

                        it("should return 4", |state| {
                            update_test(state);
                            expect(add_two(2)).to_be(4)
                        })

                    ])


            ])
            .specs(vec![

                it("should do nothing", |state| {
                    update_test(state);
                    expect(no_op()).to_be(true)
                }),

                it("should do nothing again", |state| {
                    update_test(state);
                    expect(no_op()).to_be(true)
                })

            ]).run().to_state();

        println!("{:#?}\n\n", state);

    }

}

```
Result:
```
running 1 test



  no_op
     ✓  should do nothing (0ms)
     ✓  should do nothing again (0ms)
    add_one()
       ✓  should return 1 (0ms)
       ✓  should return 2 (0ms)
    add_two()
       ✓  should return 2 (0ms)
       ✓  should return 4 (0ms)


  ✓ 6 tests completed (0ms)




Counter {
    before_all_hit_count: 2,
    before_each_hit_count: 4,
    after_each_hit_count: 4,
    after_all_hit_count: 2,
    test_hit_count: 4,
}


test tests::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Testing Large Packages Divided by Modules
```rust

fn main() {
    add_one::add_one(0);
    multiply_by_two::multiply_by_two(1);
}

pub mod add_one {

    pub fn add_one (x: u64) -> u64 { x + 1 }

    #[cfg(test)]
    pub mod tests {

        use super::*;
        use laboratory::{describe, it, expect, Suite};
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
pub mod multiply_by_two {

    pub fn multiply_by_two (x: u64) -> u64 { x * 2 }

    #[cfg(test)]
    pub mod tests {

        use super::*;
        use laboratory::{describe, it, expect, Suite};
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

#[cfg(test)]
mod tests {
    use super::*;
    use laboratory::{describe};

    #[test]
    fn test() {

        describe("Package").suites(vec![

            add_one::tests::suite(),
            multiply_by_two::tests::suite()

        ]).run();

    }

}

```
Result:
```
running 1 test



  Package
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
fn main() {
    add_one(0);
    add_two(0);
}

fn add_one (x: u64) -> u64 { x + 1 }
fn add_two (x: u64) -> u64 { x + 5 }

#[cfg(test)]
mod tests {

    use super::*;
    use laboratory::{describe, it, expect};

    #[test]
    fn suite() {

        describe("Package").suites(vec![

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
```
running 1 test

{
  "name": "Package",
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
          "duration": 0
        },
        {
          "name": "should return 2",
          "full_name": "add_one() should return 2",
          "pass": true,
          "error_msg": null,
          "duration": 0
        }
      ],
      "duration": 0
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
          "error_msg": "Expected 5 to equal 2",
          "duration": 0
        }
      ],
      "duration": 0
    }
  ],
  "child_tests": [],
  "duration": 0
}


test tests::suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```