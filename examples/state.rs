
fn no_op() -> bool { true  }

fn main() { no_op(); }

#[cfg(test)]
mod tests {

    use super::no_op;
    use laboratory::{describe, it, expect, Deserialize, Serialize};
    use std::fmt::{Debug};

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

        let state: Counter = describe("no_op").state(Counter::new()).before_all(|state| {
            let mut counter: Counter = state.get_state();
            counter.before_all_hit_count += 1;
            state.set_state(counter);
        }).before_each(|state| {
            let mut counter: Counter = state.get_state();
            counter.before_each_hit_count += 1;
            state.set_state(counter);
        }).after_each(|state| {
            let mut counter: Counter = state.get_state();
            counter.after_each_hit_count += 1;
            state.set_state(counter);
        }).after_all(|state| {
            let mut counter: Counter = state.get_state();
            counter.after_all_hit_count += 1;
            // println!("\n\n{:#?}", &counter);
            state.set_state(counter);
        }).specs(vec![

            it("should do nothing", |state| {
                let mut counter: Counter = state.get_state();
                counter.test_hit_count += 1;
                state.set_state(counter);
                expect(no_op()).to_be(true)
            }),

            it("should do nothing again", |state| {
                let mut counter: Counter = state.get_state();
                counter.test_hit_count += 1;
                state.set_state(counter);
                expect(no_op()).to_be(true)
            })

        ]).run().to_state();

        println!("{:#?}\n\n", state);

    }

}
