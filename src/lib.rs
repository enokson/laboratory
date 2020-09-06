use std::cmp::PartialEq;
use std::borrow::{BorrowMut, Borrow};
use std::char::decode_utf16;
use hex;
use std::io::Read;

pub type Tests = Vec<Test>;
pub type Suites = Vec<Suite>;
pub type ExpectResult = Result<(), String>;
pub type Handle = dyn Fn() -> Result<(), String>;
pub type HandleRef = Box<Handle>;

pub enum Report {
    Stdout
}

pub struct Expect<T>
where
    T: PartialEq,
{
    pub result: T
}
impl<T> Expect<T>
where
    T: PartialEq,
{
    pub fn expect(result: T) -> Expect<T> {
        Expect { result }
    }
    pub fn equals(&self, right: T) -> ExpectResult {
        if self.result == right {
            Ok(())
        } else {
            Err("Comparisons do not match".to_string())
        }
    }
    pub fn to_equal (&self, right: T) -> ExpectResult {
        self.equals(right)
    }
    pub fn to_be (&self, right: T) -> ExpectResult {
        self.equals(right)
    }
}

pub fn expect<T>(result: T) -> Expect<T>
    where T: PartialEq {
    Expect { result }
}

pub fn describe <H>(name: &'static str) -> Suite {
    Suite::describe(name.to_string())
}

pub fn it <H>(name: &'static str, handle: H) -> Test
where
    H: Fn() -> Result<(), String> + 'static
{
    Test::new(name.to_string(), handle)
}

pub struct Test {
    pub name: String,
    pub test: Box<dyn Fn() -> Result<(), String>>,
    pub pass: Option<bool>,
    pub error_msg: Option<String>,
}
impl Test {
    pub fn new <T>(name: String, handle: T) -> Test
    where
        T: Fn() -> Result<(), String> + 'static
    {
        Test {
            name,
            test: Box::new(handle),
            pass: None,
            error_msg: None,
        }
    }
    pub fn run(&mut self) {
        let test = self.test.as_ref();
        match (test)() {
            Ok(_) => {
                self.pass = Some(true);
            }
            Err(message) => {
                self.pass = Some(false);
                self.error_msg = Some(message);
            },
            _ => {
                self.pass = Some(false);
                self.error_msg = Some("something happened".to_string());
            }
        }
    }
}

pub struct Suite {
    pub name: String,
    pub tests: Tests,
    pub suites: Suites,
    pub handle: Box<dyn Fn()>,
    pub before_all_handle: Option<Box<dyn Fn()>>,
    pub before_each_handle: Option<Box<dyn Fn()>>,
    pub after_all_handle: Option<Box<dyn Fn()>>,
    pub after_each_handle: Option<Box<dyn Fn()>>,
    pub reporter: Report
}
impl Suite {

    pub fn describe(name: String) -> Suite {
        Suite {
            name,
            tests: vec![],
            suites: vec![],
            handle: Box::new(handle),
            before_all_handle: None,
            before_each_handle: None,
            after_all_handle: None,
            after_each_handle: None,
            reporter: Report::Stdout
        }
    }
    pub fn run(mut self) -> Self {
        self.run_nested(0);
        self
    }
    fn run_nested(&mut self, nested: i32) {
        let execute_handle = |handle_option: &Option<Box<dyn Fn()>>| {
            match &handle_option {
                Some(handle) => { (handle.as_ref())(); },
                None => {}
            }
        };
        let len = self.tests.len();
        execute_handle(&self.before_all_handle);
        for i in 0..len {
            execute_handle(&self.before_each_handle);
            let test = &mut self.tests[i];
            test.run();
            execute_handle(&self.after_each_handle);
        }
        for i in 0..self.suites.len() {
            let suite = &mut self.suites[i];
            suite.run_nested(nested + 1);
            suite.print_nested(nested + 1);
        }
        execute_handle(&self.after_all_handle);

    }
    pub fn tests(mut self, tests: Tests) -> Self {
        self.tests = tests;
        self
    }
    pub fn suites(mut self, suites: Suites) -> Self {
        self.suites = suites;
        self
    }

    pub fn before_all<H>(mut self, handle: H) -> Self
    where
        H: Fn() + 'static
    {
        self.before_all_handle = Some(Box::new(handle));
        self
    }
    pub fn before_each<H>(mut self, handle: H) -> Self
        where H: Fn() + 'static
    {
        self.before_each_handle = Some(Box::new(handle));
        self
    }
    pub fn after_all<H>(mut self, handle: H) -> Self
        where H: Fn() + 'static
    {
        self.after_all_handle = Some(Box::new(handle));
        self
    }
    pub fn after_each<H>(mut self, handle: H) -> Self
        where H: Fn() + 'static
    {
        self.after_each_handle = Some(Box::new(handle));
        self
    }
    fn report (mut self, reporter: Report) -> Self {
        self.reporter = reporter;
        self
    }
    pub fn print (&self) {
        let report = self.print_nested(0);
        let results = self.get_results();
        println!("{}", &report);
        println!("  passing: {}\n", results.0);
        println!("  failing: {}\n", results.1);
        println!("  skipped: {}\n", results.2);
        println!("\n\n");
    }
    fn print_nested (&self, nested: i32) -> String {
        let mut report = String::new();

        let get_spacing = |count| {
            let mut spaces = String::new();
            for i in 1..=count {
                spaces += " ";
            }
            spaces
        };
        report += "\n\n";
        report += &get_spacing(nested + 2);
        report += &self.name;
        report += "\n";
        self.tests.iter().for_each(|test| {
            match test.pass {
                Some(result) => {
                    if result == true {
                        report += &format!("{}{} {}", get_spacing(nested + 4), '✓', &test.name);
                        // println!("{}{} {}", get_spacing(nested + 4), '✓', test.name);
                    } else {
                        report += &format!("{}{} {}", get_spacing(nested + 4), '✗', &test.name);
                        // println!("{}{} {}", get_spacing(nested + 4), '✗', test.name);
                    }
                },
                None => {
                    report += &format!("{}{} {}", get_spacing(nested + 4), ' ', &test.name);
                    // println!("{}  {}", get_spacing(nested + 4), test.name);
                }
            }
            report += "\n";
        });
        self.suites.iter().for_each(|suite| {
            report += &suite.print_nested(nested + 1);
        });
        report += "\n";
        report
    }
    fn get_results (&self) -> (i32, i32, i32) {
        let mut passed = 0;
        let mut failed = 0;
        let mut ignored = 0;
        self.tests.iter().for_each(|test| {
            match test.pass {
                Some(result) => {
                    if result == true {
                        passed += 1;
                    } else {
                        failed += 1;
                    }
                },
                None => {
                    ignored += 1;
                }
            }
        });
        self.suites.iter().for_each(|suite| {
            let results = suite.get_results();
            passed += results.0;
            failed += results.1;
            ignored += results.2;
        });
        (passed, failed, ignored)
    }
}

#[cfg(test)]
mod test {
    use super::{Test, Suite, Expect, expect, describe, it};

    #[derive(PartialEq)]
    struct Foo {
        bar: String
    }

    impl Foo {
        pub fn new (bar: String) -> Foo {
            Foo {
                bar
            }
        }
    }

    #[test]
    fn suite() {
        fn add_one (x: u64) -> u64 { x + 1 };
        let test_1 = Test::new("should_return_1".to_string(), || {
            let result = &add_one(0);
            expect(result).equals(&1)?;
            Ok(())
        });
        let test_2 = Test::new("should_return_2".to_string(), || {
            let result = &add_one(1);
            expect(result).equals(&4)?;
            Ok(())
        });
        describe("Library")
            .tests(vec![

                it("should_return_1", || {
                    let result = &add_one(0);
                    expect(result).equals(&1)?;
                    Ok(())
                }),

                it("should_return_2", || {
                    let result = &add_one(1);
                    expect(result).equals(&4)?;
                    Ok(())
                })

            ])
            .suites(vec![

                describe("add_one")

                    .tests(vec![

                        it("should_return_1", || {
                            let result = &add_one(0);
                            expect(result).equals(&1)?;
                            Ok(())
                        }),

                        it("should_return_2", || {
                            let result = &add_one(1);
                            expect(result).equals(&4)?;
                            Ok(())
                        }),

                        it("should_return_3", || {
                            let result = &add_one(2);
                            expect(result).equals(&3)?;
                            Ok(())
                        })

                    ]),

                describe("Foo")

                    .tests(vec![

                        it("should have member \"bar\"", || {
                            expect(Foo::new("baz".to_string()).bar).to_be("baz".to_string())?;
                            Ok(())
                        })

                    ])

            ])
            .run()
            .print();

    }
}
