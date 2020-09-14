
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
