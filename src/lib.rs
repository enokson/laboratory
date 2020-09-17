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

    const EXPECTED_FOLDER: &str = "./tests/expected";
    const OUTPUT_FOLDER: &str = "./tests/output";

    fn get_output_path(test_name: &str) -> String {
        let mut path = String::from(OUTPUT_FOLDER);
        path += &format!("/{}", test_name);
        path
    }

    fn get_expected_path(test_name: &str) -> String {
        let mut path = String::from(EXPECTED_FOLDER);
        path += &format!("/{}", test_name);
        path
    }

    fn get_approval_file(test_name: &str) -> String {
        read_to_string(get_expected_path(test_name))
            .expect(&format!("Could not find {}", get_expected_path(test_name)))
    }

    #[test]
    fn get_aprv_file() {
        let result = get_expected_path("my-test");
        assert_eq!("./tests/expected/my-test".to_string(), result);
    }

    #[test]
    fn simple_pass() {

        fn return_one() -> i32 { 1 }

        const TEST_NAME: &str = "simple";

        // simple spec pass
        let result_str = describe("add_one()")
            .specs(vec![

                it("should return 1", |_| { expect(return_one()).to_equal(1) })


            ])
            .export_to(&get_output_path(TEST_NAME))
            .run()
            .to_string();
        let control = get_approval_file(TEST_NAME);
        assert_eq!(result_str, control)
    }

    #[test]
    fn simple_fail() {

        fn add_one() -> i32 { 0 }

        const TEST_NAME: &str = "simple_fail";

        let result_str = describe("add_one")
            .specs(vec![

                it("should return 1", |_| {

                    expect(add_one()).to_equal(1)

                })

            ])
            .spec()
            .export_to(&get_output_path(TEST_NAME))
            .run()
            .to_string();

        let control = get_approval_file(TEST_NAME);
        assert_eq!(result_str, control)

    }

    #[test]
    fn min() {

        fn add_one() -> i32 { 1 }

        const TEST_NAME: &str = "min";
        let result_str = describe("add_one")
            .specs(vec![

                it("should return 1", |_| {

                    expect(add_one()).to_equal(1)

                })

            ])
            .min()
            .export_to(&get_output_path(TEST_NAME))
            .run()
            .to_string();

        let control = get_approval_file(TEST_NAME);
        assert_eq!(result_str, control)

    }

    #[test]
    fn min_fail() {

        fn return1() -> i32 { 0 }

        const TEST_NAME: &str = "min_fail";
        let result_str = describe("return1")
            .specs(vec![

                it("should return 1", |_| {

                    expect(return1()).to_equal(1)

                })

            ])
            .min()
            .export_to(&get_output_path(TEST_NAME))
            .run()
            .to_string();

        let control = get_approval_file(TEST_NAME);
        assert_eq!(result_str, control)

    }

    #[test]
    fn json() {

        fn add_one() -> i32 { 1 }

        const TEST_NAME: &str = "output_json.json";
        let result_str = describe("add_one")
            .specs(vec![

                it("should return 1", |_| {

                    expect(add_one()).to_equal(1)

                })

            ])
            .json()
            .export_to(&get_output_path(TEST_NAME))
            .run()
            .to_string();

        let control = get_approval_file(TEST_NAME);
        assert_eq!(result_str, control)

    }

    #[test]
    fn json_pretty() {

        fn add_one() -> i32 { 1 }

        const TEST_NAME: &str = "output_json_pretty.json";
        let result_str = describe("add_one")
            .specs(vec![

                it("should return 1", |_| {

                    expect(add_one()).to_equal(1)

                })

            ])
            .json_pretty()
            .export_to(&get_output_path(TEST_NAME))
            .run()
            .to_string();

        let control = get_approval_file(TEST_NAME);
        assert_eq!(result_str, control)

    }

    #[test]
    fn suite_skip() {

        fn add_one() -> i32 { 1 }

        fn return_two() -> i32 { 2 }

        const TEST_NAME: &str = "suite_skip";
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
            .export_to(&get_output_path(TEST_NAME))
            .run()
            .to_string();

        let control = get_approval_file(TEST_NAME);
        assert_eq!(result_str, control)

    }

    #[test]
    fn spec_skip() {

        fn add_one() -> i32 { 1 }

        fn return_two() -> i32 { 2 }

        const TEST_NAME: &str = "spec_skip";
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
            .export_to(&get_output_path(TEST_NAME))
            .run()
            .to_string();

        let control = get_approval_file(TEST_NAME);
        assert_eq!(result_str, control)

    }

    #[test]
    fn spec_only() {

        fn add_one() -> i32 { 1 }

        fn return_two() -> i32 { 2 }

        const TEST_NAME: &str = "spec_only";
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
            .export_to(&get_output_path(TEST_NAME))
            .run()
            .to_string();

        let control = get_approval_file(TEST_NAME);
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
