#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]

mod assertion;
mod spec;
mod suite;
mod reporter;
mod state;
mod suite_result;
mod spec_result;

pub use suite::{Suite};
pub use state::State;
pub use assertion::Expect;
pub use spec::Spec;
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

#[cfg(test)]
mod tests {

    use std::fs::{remove_file, read_to_string};
    use std::path::Path;
    use super::*;

    #[test]
    fn simple_pass() {

        fn return_one() -> i32 { 1 }

        // simple spec pass
        let result_str = describe("add_one()")
            .specs(vec![

                it("should return 1", |_| { expect(return_one()).to_equal(1) })


            ])
            // .export_to("./tests/results/simple")
            .run()
            .to_string();
        let control = read_to_string("./tests/results/simple").expect("Could not find ./tests/results/simple");
        assert_eq!(result_str, control)
    }

    #[test]
    fn simple_fail() {

        fn add_one() -> i32 { 0 }

        let result_str = describe("add_one")
            .specs(vec![

                it("should return 1", |_| {

                    expect(add_one()).to_equal(1)

                })

            ])
            .spec()
            // .export_to("./tests/results/simple-fail")
            .run()
            .to_string();

        let control = read_to_string("./tests/results/simple-fail").expect("Could not find ./tests/results/simple-fail");
        assert_eq!(result_str, control)

    }

    #[test]
    fn min() {

        fn add_one() -> i32 { 1 }

        const OUTPUT: &str = "./tests/results/min";
        let result_str = describe("add_one")
            .specs(vec![

                it("should return 1", |_| {

                    expect(add_one()).to_equal(1)

                })

            ])
            .min()
            // .export_to(OUTPUT)
            .run()
            .to_string();

        let control = read_to_string(OUTPUT).expect(&format!("Could not find {}", OUTPUT));
        assert_eq!(result_str, control)

    }

    #[test]
    fn min_fail() {

        fn return1() -> i32 { 0 }

        const OUTPUT: &str = "./tests/results/min-fail";
        let result_str = describe("return1")
            .specs(vec![

                it("should return 1", |_| {

                    expect(return1()).to_equal(1)

                })

            ])
            .min()
            // .export_to(OUTPUT)
            .run()
            .to_string();

        let control = read_to_string(OUTPUT).expect(&format!("Could not find {}", OUTPUT));
        assert_eq!(result_str, control)

    }

    #[test]
    fn json() {

        fn add_one() -> i32 { 1 }

        const OUTPUT: &str = "./tests/results/output-json.json";
        let result_str = describe("add_one")
            .specs(vec![

                it("should return 1", |_| {

                    expect(add_one()).to_equal(1)

                })

            ])
            .json()
            // .export_to(OUTPUT)
            .run()
            .to_string();

        let control = read_to_string(OUTPUT).expect(&format!("Could not find {}", OUTPUT));
        assert_eq!(result_str, control)

    }

    #[test]
    fn json_pretty() {

        fn add_one() -> i32 { 1 }

        const OUTPUT: &str = "./tests/results/output-json-pretty.json";
        let result_str = describe("add_one")
            .specs(vec![

                it("should return 1", |_| {

                    expect(add_one()).to_equal(1)

                })

            ])
            .json_pretty()
            // .export_to(OUTPUT)
            .run()
            .to_string();

        let control = read_to_string(OUTPUT).expect(&format!("Could not find {}", OUTPUT));
        assert_eq!(result_str, control)

    }

    #[test]
    #[ignore]
    fn suite_skip() {

        fn add_one() -> i32 { 1 }

        fn return_two() -> i32 { 2 }

        const OUTPUT: &str = "./tests/results/suite-skip";
        let result_str = describe("Library")
            .suites(vec![

                describe_skip("add_one()")
                    .specs(vec![

                        it("should return 1", |_| {

                            expect(add_one()).to_equal(1)

                        })

                    ]),

                describe("return_two()")
                    .specs(vec![

                        it("should return 2", |_| {

                            expect(return_two()).to_equal(2)

                        })

                    ])


            ])
            // .export_to(OUTPUT)
            .run()
            .to_string();

        let control = read_to_string(OUTPUT).expect(&format!("Could not find {}", OUTPUT));
        assert_eq!(result_str, control)

    }

    #[test]
    fn spec_skip() {

        fn add_one() -> i32 { 1 }

        fn return_two() -> i32 { 2 }

        const OUTPUT: &str = "./tests/results/spec-skip";
        let result_str = describe("Library")
            .suites(vec![

                describe("add_one()")
                    .specs(vec![

                        it_skip("should return 1", |_| {

                            expect(add_one()).to_equal(1)

                        }),
                        it("should return 1", |_| {

                            expect(add_one()).to_equal(1)

                        })

                    ]),

                describe("return_two()")
                    .specs(vec![

                        it("should return 2", |_| {

                            expect(return_two()).to_equal(2)

                        })

                    ])


            ])
            .export_to(OUTPUT)
            .run()
            .to_string();

        // let control = read_to_string(OUTPUT).expect(&format!("Could not find {}", OUTPUT));
        // assert_eq!(result_str, control)

    }

    #[test]
    fn spec_only() {

        fn add_one() -> i32 { 1 }

        fn return_two() -> i32 { 2 }

        const OUTPUT: &str = "./tests/results/spec-only";
        let result_str = describe("Library")
            .suites(vec![

                describe("add_one()")
                    .specs(vec![

                        it_only("should return 1", |_| {

                            expect(add_one()).to_equal(1)

                        }),
                        it("should return 1", |_| {

                            expect(add_one()).to_equal(1)

                        })

                    ]),

                describe("return_two()")
                    .specs(vec![

                        it("should return 2", |_| {

                            expect(return_two()).to_equal(2)

                        })

                    ])


            ])
            // .export_to(OUTPUT)
            .run()
            .to_string();

        let control = read_to_string(OUTPUT).expect(&format!("Could not find {}", OUTPUT));
        assert_eq!(result_str, control)

    }

    #[test]
    fn state_passing() {

        #[derive(Deserialize, Serialize, Debug)]
        struct Counter {
            count: i32
        }

        impl Counter {
            pub fn new() -> Counter { Counter { count: 0 } }
        }

        fn return_one() -> i32 { 1 }
        fn return_two() -> i32 { 2 }

        let counter: Counter = describe("Library")
            .state(Counter::new())
            .suites(vec![

                describe("return_one()")
                    .inherit_state()
                    .specs(vec![

                        it("should return 1", |suite| {
                            let mut counter: Counter = suite.get_state();
                            counter.count += 1;
                            suite.set_state(counter);
                            expect(return_one()).to_equal(1)

                        }),
                        it("should return 1 again", |suite| {

                            let mut counter: Counter = suite.get_state();
                            counter.count += 1;
                            suite.set_state(counter);
                            expect(return_one()).to_equal(1)

                        })

                    ]),

                describe("return_two()")
                    .specs(vec![

                        it("should return 2", |_| {

                            expect(return_two()).to_equal(2)

                        })

                    ])


            ])
            .run()
            .to_state();

        assert_eq!(counter.count, 2)

    }

}