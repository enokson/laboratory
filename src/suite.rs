use console::style;

use std::borrow::{BorrowMut};
use std::collections::HashMap;

use super::spec::Spec;
use super::reporter::{Reporter, SpecReporter};
use std::time::Instant;

pub struct Result {
    passing: u64,
    failing: u64,
    ignored: u64,
    duration: u128
}

pub struct Suite<S> {
    name: String,
    test_list: Vec<Spec>,
    suite_list: Vec<Suite<S>>,
    hooks: HashMap<String, Box<dyn FnMut(S) -> S>>,
    reporter: Reporter,
    suite_state: Option<S>,
    state_hash: HashMap<u8, S>,
    duration: u128,
    pub ignore: bool
}
impl<S> Suite<S> {

    pub fn new(name: String) -> Suite<S> {
        Suite {
            name,
            test_list: vec![],
            suite_list: vec![],
            suite_state: None,
            hooks: HashMap::new(),
            reporter: Reporter::Spec,
            state_hash: HashMap::new(),
            ignore: false,
            duration: 0,
        }
    }
    pub fn run(mut self) -> Self {
        let start_time = Instant::now();
        self.run_nested(0);
        self.duration = start_time.elapsed().as_millis();
        self.export_report(0);
        self
    }
    pub fn specs(mut self, tests: Vec<Spec>) -> Self {
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
    pub fn skip (mut self) -> Self {
        self.ignore = true;
        self
    }

    fn run_nested(&mut self, nested: i32) {

        // execute_handle(self.suite_state, self.hooks.get_mut("before all"));
        self.execute_hook("before all");
        let len = self.test_list.len();
        let mut only_id = None;

        // check for specs marked as only
        for i in 0..len {
            let test = &self.test_list[i];
            if test.only_ == true {
                only_id = Some(i);
                break;
            }
        }
        match only_id {
            Some(id) => {

                // run the first test marked as only

                self.execute_hook("before each");
                let test = &mut self.test_list[id];
                test.run();
                self.execute_hook("after each");

                // self.handle_result(test);
            },
            None => {

                // run all tests not marked as ignore

                for i in 0..len {
                    // (self.before_each_handle)();
                    self.execute_hook("before each");
                    let test = &mut self.test_list[i];
                    if self.ignore == true {
                        test.ignore = true;
                    }
                    test.run();
                    // (self.after_each_handle)();
                    self.execute_hook("after each");
                }
            }
        }

        let len = self.suite_list.len();
        for i in 0..len {
            let suite = self.suite_list[i].borrow_mut();
            if self.ignore == true {
                suite.ignore = true;
            }
            suite.run_nested(nested + 1);
            suite.export_report(nested + 1);
        }
        self.execute_hook("after all");
        // (self.after_all_handle)();
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

    fn export_report(&self, nested: i32) -> String {
        match self.reporter {
            Reporter::Spec => {
                let mut report = String::new();
                let get_spacing = |count| {
                    let mut spaces = String::new();
                    for i in 1..=count {
                        spaces += " ";
                    }
                    spaces
                };
                if nested == 0 {
                    report += "\n\n";
                } else {
                    report += "\n";
                }
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
                    let duration = match &test.duration {
                        Some(duration) => duration,
                        None => &0
                    };
                    report += &format!(" ({}ms)", duration);
                    report += "\n";
                });
                self.suite_list.iter().for_each(|suite| {
                    report += &suite.export_report(nested + 1);
                });
                let result = self.get_results();
                if nested == 0 {
                    report += "\n\n";

                    if result.failing != 0 {

                    } else {
                        report += &format!("  ✓ {} tests completed ({}ms)", result.passing, result.duration);
                    }
                    report += "\n\n";
                    println!("{}", report);
                }

                // report += "\n";

                report
            }
        }


    }
    fn get_results (&self) -> Result {
        let mut passing = 0;
        let mut failing = 0;
        let mut ignored = 0;
        let mut duration = self.duration;
        self.test_list.iter().for_each(|test| {
            match test.pass {
                Some(result) => {
                    if result == true {
                        passing += 1;
                    } else {
                        failing += 1;
                    }
                },
                None => {
                    ignored += 1;
                }
            }
        });
        self.suite_list.iter().for_each(|suite| {
            let results = suite.get_results();
            passing += results.passing;
            failing += results.failing;
            ignored += results.ignored;
            duration += results.duration
        });
        Result {
            passing,
            failing,
            ignored,
            duration
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

}