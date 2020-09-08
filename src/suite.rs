use std::borrow::{BorrowMut};
use std::collections::HashMap;

use super::spec::Spec;

enum Report {
    Stdout
}

pub struct Suite<S> {
    name: String,
    test_list: Vec<Spec>,
    suite_list: Vec<Suite<S>>,
    hooks: HashMap<String, Box<dyn FnMut(S) -> S>>,
    reporter: Report,
    suite_state: Option<S>,
    state_hash: HashMap<u8, S>,
    pass_state: bool
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
            state_hash: HashMap::new(),
            pass_state: false
        }
    }
    pub fn run(mut self) -> Self {
        self.run_nested(0);
        self
    }
    fn run_nested(&mut self, nested: i32) {

        // execute_handle(self.suite_state, self.hooks.get_mut("before all"));
        self.execute_hook("before all");
        let len = self.test_list.len();
        for i in 0..len {
            // (self.before_each_handle)();
            self.execute_hook("before each");
            let test = &mut self.test_list[i];
            test.run();
            // (self.after_each_handle)();
            self.execute_hook("after each");
        }
        let len = self.suite_list.len();
        for i in 0..len {
            let suite = self.suite_list[i].borrow_mut();
            suite.run_nested(nested + 1);
            suite.print_nested(nested + 1);
        }
        self.execute_hook("after all");
        // (self.after_all_handle)();
    }
    pub fn tests(mut self, tests: Vec<Spec>) -> Self {
        self.test_list = tests;
        self
    }
    pub fn suites(mut self, suites: Vec<Suite<S>>) -> Self {
        self.suite_list = suites;
        self
    }
    pub fn state(mut self, state: S) -> Self {
        // self.state_hash.insert(1, Box::new(state));
        self.state_hash.insert(0, state);
        self
    }
    pub fn pass_state(mut self) -> Self {
        self.pass_state = true;
        self
    }
    fn execute_hook(&mut self, hook_name: &str) {
        match self.hooks.get_mut(hook_name) {
            Some(hook) => {
                match self.state_hash.remove(&0) {
                    Some(state) => {
                        self.state_hash.insert(0, (hook)(state));
                    },
                    None => {}
                }
            },
            None => {

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