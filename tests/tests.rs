use std::fs::{remove_file, read_to_string};
use laboratory::*;
use laboratory::Error;

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

    use laboratory::SuiteResult;
    use serde_json::from_str;

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

    let _result: SuiteResult = from_str(&result_str).expect("could not serialize the result");

}

#[test]
fn json_pretty() {

    use laboratory::SuiteResult;
    use serde_json::from_str;

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

    let _result: SuiteResult = from_str(&result_str).expect("could not serialize the result");

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
        .state(Counter::new()).unwrap()
        .suites(vec![

            describe("return_one()")
                .inherit_state()
                .specs(vec![

                    it("should return 1", |state| {
                        let mut counter: Counter = state.get()?;
                        counter.count += 1;
                        state.set(counter)?;
                        expect(return_one()).to_equal(1)

                    }),
                    it("should return 1 again", |state| {
                        let mut counter: Counter = state.get()?;
                        counter.count += 1;
                        state.set(counter)?;
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
        .to_state().unwrap();

    assert_eq!(counter.count, 2)

}

#[test]
fn return_result() {

    fn add_one(n: i32) -> i32 { n + 1 }
    let test_result = describe("add_one()").specs(vec![

        it("should return 1", |_| {

            expect(add_one(0)).to_equal(1)

        }),

        it("should return 2", |_| {

            expect(add_one(0)).to_equal(2)

        })

    ]).run().to_result();

    let mut does_match = false;
    if let Err(error) = test_result {
        if let Error::Assertion(msg) = error {
            if "1 of 2 tests failed".to_string() == msg {
                does_match = true
            }
        }
    }
    assert_eq!(does_match, true)

}

#[test]
fn micro() {

    fn return_one() -> i32 { 1 }

    const TEST_NAME: &str = "micro";

    // simple spec pass
    let result_str = describe("add_one()")
        .specs(vec![

            it("should return 1", |_| { expect(return_one()).to_equal(1) })


        ])
        .export_to(&get_output_path(TEST_NAME))
        .in_microseconds()
        .run()
        .to_string();
    // let control = get_approval_file(TEST_NAME);
    assert!(result_str.contains("μs)"))
}

#[test]
fn nano() {

    fn return_one() -> i32 { 1 }

    const TEST_NAME: &str = "nano";

    // simple spec pass
    let result_str = describe("add_one()")
        .specs(vec![

            it("should return 1", |_| { expect(return_one()).to_equal(1) })


        ])
        .export_to(&get_output_path(TEST_NAME))
        .in_nanoseconds()
        .run()
        .to_string();
    // let control = get_approval_file(TEST_NAME);
    assert!(result_str.contains("ns)"))
}

#[test]
fn seconds() {

    fn return_one() -> i32 { 1 }

    const TEST_NAME: &str = "seconds";

    // simple spec pass
    let result_str = describe("add_one()")
        .specs(vec![

            it("should return 1", |_| { expect(return_one()).to_equal(1) })


        ])
        .export_to(&get_output_path(TEST_NAME))
        .in_seconds()
        .run()
        .to_string();
    // let control = get_approval_file(TEST_NAME);
    assert!(result_str.contains("sec)"))
}

#[test]
fn should_catch_state_deserialize_error() {

    #[derive(Serialize, Deserialize)]
    struct MyStruct {
        pub foo: String
    }

    #[derive(Serialize, Deserialize)]
    struct MyOtherStruct {
        pub foo: String,
        pub bar: String
    }

    describe("add_one()")
        .state(MyStruct { foo: "fizzbuzz".to_string() }).unwrap()
        .specs(vec![
            it("should return 1", |state| {
                let my_other_struct: MyOtherStruct = state.get::<MyOtherStruct>()?;
                expect(my_other_struct.bar).to_equal("hello, worlds".to_string())
            })
        ])
        .run();
}