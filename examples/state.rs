
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
    use laboratory::{describe, it, expect, Deserialize, Serialize, State};
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
        fn hook_handle(state: &mut State) {

            // We need to call the get_state method in order to get the counter.
            // We also we to tell the Rust compiler what
            // type the result of get_state will be which
            // in this case is the counter.
            let mut counter= state.get::<Counter>().unwrap();

            // Now we will call the update method on Counter
            counter.update();

            // And if we want to update the state we need to call set_state
            state.set(counter).unwrap();
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
