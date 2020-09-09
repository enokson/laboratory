use console::style;

use std::borrow::{BorrowMut};
use std::collections::HashMap;

use super::spec::{Spec, SpecResult};
use super::reporter::{ReporterType, Reporter};
use std::time::Instant;

pub struct SuiteResult {
    name: String,
    passing: u64,
    failing: u64,
    ignored: u64,
    child_suites: Vec<SuiteResult>,
    child_tests: Vec<SpecResult>,
    duration: u128
}
impl SuiteResult {
    pub fn new(name: &str) -> SuiteResult {
        SuiteResult {
            name: name.to_string(),
            passing: 0,
            failing: 0,
            ignored: 0,
            child_suites: vec![],
            child_tests: vec![],
            duration: 0
        }
    }
    pub fn add_spec_result (&mut self, spec: SpecResult) {
        self.child_tests.push(spec);
    }
    pub fn updated_from_suite(&mut self, child_result_option: Option<SuiteResult>) {
        match child_result_option {
            Some(child_result) => {
                self.passing += child_result.get_passing();
                self.failing += child_result.get_failing();
                self.ignored += child_result.get_ignored();
                self.child_suites.push(child_result);
            },
            _ => {}
        }

    }
    pub fn update_from_spec(&mut self, spec: SpecResult) {
        self.passing += spec.update_passing();
        self.failing += spec.update_failing();
        self.ignored += spec.update_ignored();
        self.child_tests.push(spec);
    }
    pub fn get_passing(&self) -> u64 { self.passing }
    pub fn get_failing(&self) -> u64 { self.failing }
    pub fn get_ignored(&self) -> u64 { self.ignored }
    pub fn get_child_specs(&self) -> Vec<SpecResult> {
        self.child_tests.clone()
    }
    pub fn get_child_suites(&self) -> Vec<SuiteResult> {
        self.child_suites.clone()
    }
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_duration(&self) -> &u128 { &self.duration }
    pub fn set_duration(&mut self, duration: u128) { self.duration = duration }
}
impl Clone for SuiteResult {
    fn clone(&self) -> SuiteResult {
        SuiteResult {
            name: self.name.clone(),
            passing: self.passing.clone(),
            failing: self.failing.clone(),
            ignored: self.ignored.clone(),
            child_suites: self.child_suites.clone(),
            child_tests: self.child_tests.clone(),
            duration: self.duration.clone()
        }
    }
}

pub struct Suite<S> {
    duration: u128,
    hooks: HashMap<String, Box<dyn FnMut(S) -> S>>,
    pub ignore: bool,
    name: String,
    reporter: ReporterType,
    result: Option<SuiteResult>,
    state_hash: HashMap<u8, S>,
    suite_list: Vec<Suite<S>>,
    suite_state: Option<S>,
    test_list: Vec<Spec>,
}
impl<S> Suite<S> {

    pub fn new(name: String) -> Suite<S> {
        Suite {
            duration: 0,
            hooks: HashMap::new(),
            ignore: false,
            name,
            reporter: ReporterType::Spec,
            result: None,
            state_hash: HashMap::new(),
            suite_list: vec![],
            suite_state: None,
            test_list: vec![],
        }
    }
    pub fn run(mut self) -> Self {
        let start_time = Instant::now();
        self.run_(0);
        self.duration = start_time.elapsed().as_millis();
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

    fn clone_result(&self) -> Option<SuiteResult> {
        match &self.result {
            Some(result) => Some(result.clone()),
            None => None
        }
    }
    fn run_(&mut self, nest_count: u32) {

        let mut result = SuiteResult::new(&self.name);
        let start_time = Instant::now();
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
                result.update_from_spec(test.export_results(&self.name));
                self.execute_hook("after each");

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
                    result.update_from_spec(test.export_results(&self.name));
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
            suite.run_(nest_count + 1);
            result.updated_from_suite(suite.clone_result());
        }
        self.execute_hook("after all");
        result.set_duration(start_time.elapsed().as_millis());
        self.result = Some(result);
        if nest_count == 0 {
            self.report();
        }

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
    fn report(&self) {
        match &self.result {
            Some(result) => {
                match &self.reporter {
                    ReporterType::Spec => Reporter::spec(result.clone())
                }
            },
            None => {
                // no result found
                println!("result not found");
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

}