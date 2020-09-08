use std::cmp::PartialEq;
use std::borrow::{BorrowMut, Borrow};
use std::char::decode_utf16;
use hex;
use std::io::Read;
use std::any::{Any};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::ops::Deref;

pub type Tests = Vec<Test>;
pub type Suites<S> = Vec<Suite<S>>;
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

pub fn describe<S>(name: &'static str) -> Suite<S> {
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

pub struct Suite<S> {
    name: String,
    test_list: Tests,
    suite_list: Suites<S>,
    hooks: HashMap<String, Box<dyn FnMut(S) -> S>>,
    reporter: Report,
    suite_state: Option<S>
}
impl<S> Suite<S> {

    pub fn describe(name: String) -> Suite<S> {
        Suite {
            name,
            test_list: vec![],
            suite_list: vec![],
            suite_state: None,
            hooks: HashMap::new(),
            reporter: Report::Stdout,
            // state_hash: Box::new(None)
        }
    }
    pub fn run(mut self) -> Self {
        self.run_nested(0);
        self
    }
    fn run_nested(&mut self, nested: i32) {

        let len = self.test_list.len();
        // execute_handle(self.suite_state, self.hooks.get_mut("before all"));
        self.suite_state = match self.hooks.get_mut("before all") {
            Some(hook) => match self.suite_state {
                Some(state) => Some((hook)(state)),
                None => None
            },
            None => match self.suite_state {
                Some(state) => Some(state),
                None => None
            }
        };
        for i in 0..len {
            // (self.before_each_handle)();
            let test = &mut self.test_list[i];
            test.run();
            // (self.after_each_handle)();
        }
        for i in 0..self.suite_list.len() {
            let suite = &mut self.suite_list[i];
            suite.run_nested(nested + 1);
            suite.print_nested(nested + 1);
        }
        // (self.after_all_handle)();

    }
    pub fn tests(mut self, tests: Tests) -> Self {
        self.test_list = tests;
        self
    }
    pub fn suites(mut self, suites: Suites<S>) -> Self {
        self.suite_list = suites;
        self
    }
    pub fn state(mut self, state: S) -> Self {
        // self.state_hash.insert(1, Box::new(state));
        self.suite_state = Some(state);
        self
    }
    fn execute_hook(&mut self, hook_name: &str) -> Option<S> {
        match self.hooks.get_mut(hook_name) {
            Some(hook) => {
                match self.suite_state {
                    Some(state) => {
                        Some((hook)(state))
                    },
                    None => None
                }
            },
            None => {
                match self.suite_state {
                    Some(state) => Some(state),
                    None => None
                }
            }
        }
    }

    pub fn before_all<H>(mut self, handle: H) -> Self
    where
        H: FnMut(S) -> S + 'static
    {
        self.hooks.insert("before all".to_string(), Box::new(handle));
        self
    }
    pub fn before_each<H>(mut self, handle: H) -> Self
        where
            H: FnMut(S) -> S + 'static
    {
        self.hooks.insert("before each".to_string(), Box::new(handle));
        self
    }
    pub fn after_all<H>(mut self, handle: H) -> Self
        where
            H: FnMut(S) -> S + 'static
    {
        self.hooks.insert("after all".to_string(), Box::new(handle));
        self
    }
    pub fn after_each<H>(mut self, handle: H) -> Self
        where
            H: FnMut(S) -> S + 'static
    {
        self.hooks.insert("after each".to_string(), Box::new(handle));
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
        self.test_list.iter().for_each(|test| {
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
        self.suite_list.iter().for_each(|suite| {
            report += &suite.print_nested(nested + 1);
        });
        report += "\n";
        report
    }
    fn get_results (&self) -> (i32, i32, i32) {
        let mut passed = 0;
        let mut failed = 0;
        let mut ignored = 0;
        self.test_list.iter().for_each(|test| {
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
        self.suite_list.iter().for_each(|suite| {
            let results = suite.get_results();
            passed += results.0;
            failed += results.1;
            ignored += results.2;
        });
        (passed, failed, ignored)
    }
}

pub struct State<T>(Rc<RefCell<T>>);
impl <T>State<T> {
    pub fn new (state: T) -> State<T> {
        State(Rc::new(RefCell::new(state)))
    }
    // pub fn as_ref(&self) -> &T {
    //     self.0.as_ref()
    // }
    // pub fn into_inner(self) -> & RefCell<T> {
    //     self.0.borrow_mut()
    // }
}

impl<T> Clone for State<T> {
    fn clone (&self) -> State<T> {
        State(self.0.clone())
    }
}

// impl<T> Deref for State<T> {
//     type Target = Rc<T>;
//     fn deref(&self) -> &Rc<T> {
//         &self.0.
//     }
// }


#[cfg(test)]
mod test {
    use std::cell::{RefCell, RefMut};
    use std::rc::Rc;
    // use super::{Test, Suite, Expect, State, expect, describe, it};
    use std::borrow::BorrowMut;

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

        // #[macro_use]
        // use super::it::{describe};

        // let mut counter_rc = Rc::new(RefCell::new(0));
        //
        // fn add_one (x: u64) -> u64 { x + 1 };

        // describe("Library")
        //     .state(0)
        //     .before_all(|state: i32| {
        //
        //     })
        //     .before_each(|| {
        //         // let mut counter_ref = counter.borrow_mut();
        //         // counter.clone() += 1;
        //         // println!("before_each_hook: {}", counter.clone());
        //     })
        //     .after_each(|| {
        //         // let counter_ref = counter.clone();
        //         // counter.clone() += 1;
        //         // println!("after_each_hook: {}", counter.clone());
        //     })
        //     .after_all(|| {
        //         // let counter_ref = counter.clone();
        //         // counter.clone() += 0;
        //         // println!("after_all_hook: {}", counter.clone());
        //     })
        //     .tests(vec![
        //
        //         it("should_return_1", || {
        //             let result = &add_one(0);
        //             expect(result).equals(&1)?;
        //             Ok(())
        //         }),
        //
        //         it("should_return_2", || {
        //             let result = &add_one(1);
        //             expect(result).equals(&4)?;
        //             Ok(())
        //         })
        //
        //     ])
        //     .suites(vec![
        //
        //         describe("add_one")
        //
        //             .tests(vec![
        //
        //                 it("should_return_1", || {
        //                     let result = &add_one(0);
        //                     expect(result).equals(&1)?;
        //                     Ok(())
        //                 }),
        //
        //                 it("should_return_2", || {
        //                     let result = &add_one(1);
        //                     expect(result).equals(&4)?;
        //                     Ok(())
        //                 }),
        //
        //                 it("should_return_3", || {
        //                     let result = &add_one(2);
        //                     expect(result).equals(&3)?;
        //                     Ok(())
        //                 })
        //
        //             ]),
        //
        //         describe("Foo")
        //
        //             .tests(vec![
        //
        //                 it("should have member \"bar\"", || {
        //                     expect(Foo::new("baz").bar).to_be("baz".to_string())?;
        //                     Ok(())
        //                 })
        //
        //             ])
        //
        //     ])
        //     .run()
        //     .print();

            // println!("counter: {}", counter.borrow());
    }
}
