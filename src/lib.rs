#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]

mod assertion;
mod spec;
mod suite;
mod reporter;

use suite::{Suite, State};
use assertion::Expect;
use spec::Spec;
use std::fmt::{Debug};
pub use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! should_panic {
    ($name:expr, $handle: expr) => {

        {
            use std::panic::{ catch_unwind, set_hook, take_hook };

            set_hook(Box::new(|_| {
                // println!("");
            }));
            let tmp_result = catch_unwind(|| {
                ($handle)();
            }).is_ok();
            let _ = take_hook();
            if tmp_result == false {
                Ok(())
            } else {
                Err(format!("Expected {} to panic but it didn't", stringify!($name)))
            }

        }

    };
}

#[macro_export]
macro_rules! should_not_panic {
    ($name:expr, $handle: expr) => {

        {
            use std::panic::{ catch_unwind, set_hook, take_hook };

            set_hook(Box::new(|_| {
                // println!("");
            }));
            let tmp_result = catch_unwind(|| {
                ($handle)();
            }).is_ok();
            let _ = take_hook();
            if tmp_result == true {
                Ok(())
            } else {
                Err(format!("Expected {} to panic but it didn't", stringify!($name)))
            }

        }

    };
}


pub fn expect<T>(result: T) -> Expect<T>
    where T: PartialEq + Debug
{
    Expect::new(result)
}

pub fn describe(name: &'static str) -> Suite {
    Suite::new(name.to_string())
}

pub fn describe_skip(name: &'static str) -> Suite {
    Suite::new(name.to_string()).skip()
}

pub fn it <H>(name: &'static str, handle: H) -> Spec
where
    H: FnMut(&mut State) -> Result<(), String> + 'static
{
    Spec::new(name.to_string(), handle)
}

pub fn it_skip<H>(name: &'static str, handle: H) -> Spec
    where
        H: FnMut(&mut State) -> Result<(), String> + 'static
{
    Spec::new(name.to_string(), handle).skip()
}

pub fn it_only<H>(name: &'static str, handle: H) -> Spec
    where
        H: FnMut(&mut State) -> Result<(), String> + 'static
{
    Spec::new(name.to_string(), handle).only()
}

/*pub fn serialize_state<'a, T: Deserialize<'a> + Serialize>(state: &BitState) -> &'a T {
    from_slice(&state).expect("Could not serialize state.")
}
pub fn deserialize_state<'a, T: Deserialize<'a> + Serialize>(state: T) -> BitState {
    to_vec(&state).expect("Could not deserialize state.")
}*/

#[cfg(test)]
mod test {
    use super::{Spec, Suite, Expect, expect};
    use super::{describe, describe_skip, it, it_skip, it_only};
    use std::borrow::{BorrowMut, Borrow};
    use std::thread;
    use std::time::Duration;

    use serde::{Deserialize, Serialize};

    #[derive(PartialEq)]
    struct Foo {
        pub bar: String
    }

    impl Foo {
        pub fn new (bar: &str) -> Foo {
            Foo {
                bar: bar.to_string()
            }
        }
    }

    #[test]
    fn suite() {

        #[derive(Deserialize, Serialize, Debug)]
        struct Counter {
            count: i32
        };
        impl Counter {
            pub fn new() -> Counter { Counter { count: 0 } }
            pub fn update(&mut self) {
                self.count += 1;
            }
        }

        #[derive(Deserialize, Serialize, Debug)]
        struct Line {
            ln: String
        }
        impl Line {
            pub fn new() -> Line { Line { ln: String::new() } }
            pub fn append(&mut self, ln: &str) {
                self.ln += ln;
            }
        }

        struct Person {
            name: String
        }
        impl Person {
            pub fn new(name: &str) -> Person {
                Person {
                    name: name.to_string()
                }
            }
            pub fn change_name(&mut self, name: &str) {
                self.name = name.to_string();
            }
        }

        fn add_one (x: u64) -> u64 { x + 1 };

        fn intensive_add_one (x: u64) -> u64 {
            thread::sleep(Duration::from_millis(10));
            x + 1
        }

        describe("Library")
            .suites(vec![

                describe("add_one()")
                    .specs(vec![

                        it("should return 1", || {
                            let result = &add_one(0);
                            expect(result).equals(&1)?;
                            Ok(())
                        }),

                        it("should return 2", || {
                            let result = &add_one(1);
                            expect(result).equals(&2)?;
                            Ok(())
                        })

                    ])
                    .before_all(|state| {
                        state.set_state(Counter::new())
                    })
                    .before_each(|state| {
                        let mut counter: Counter = state.get_state();
                        counter.update();
                        state.set_state(counter);
                    })
                    .after_each(|state| {

                    })
                    .after_all(|state| {
                        let counter: Counter = state.get_state();
                        // println!("Count: {}", counter.count);
                    }),


                describe("Person")
                    .specs(vec![

                        it("should return baxtiyor", || {
                            let baxtiyor = Person::new("baxtiyor");
                            expect(baxtiyor.name).to_be("baxtiyor".to_string())?;
                            Ok(())
                        }),

                        it("should return joshua after changing the person's name", || {
                            let mut joshua = Person::new("baxtyior");
                            joshua.change_name("joshua");
                            expect(joshua.name).to_be("joshua".to_string())?;
                            Ok(())
                        })

                    ]),

                describe("intensive_add_one()")
                    .specs(vec![

                        it("should return 5", || {
                            let result = intensive_add_one(4);
                            expect(result).equals(5)?;
                            Ok(())
                        })

                    ])


            ])
            .run();

    }

}
