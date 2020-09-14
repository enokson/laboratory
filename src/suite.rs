use console::style;

use std::borrow::{BorrowMut};
use std::collections::HashMap;
// use std::cell::RefCell;

use super::spec::{Spec, SpecResult};
use super::reporter::{ReporterType, Reporter};
use std::time::Instant;
use std::path::Path;

use serde::{Serialize};
// use serde_cbor::{to_vec, from_slice};
// use ron::{to_string, from_str};
use bincode::{serialize, deserialize};


use serde::de::Deserialize;

pub type BitState = Vec<u8>;

pub struct State {
    state: Vec<u8>
}
impl State {
    pub fn new() -> State {
        State { state: vec![] }
    }
    pub fn get_state<'a, T>(&'a self) -> T
        where
            T: Deserialize<'a>
    {
        deserialize(&self.state).expect("Could not deserialize state.")
        // from_str(&self.state).expect("Could not convert from string")
    }
    pub fn set_state<T: Serialize>(& mut self, state: T) {
        // self.state = to_string(&state).expect("Could not convert to String.");
        self.state = serialize(&state).expect("Could not serialize state.");
    }
}

#[derive(Serialize)]
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

pub struct Suite {
    duration: u128,
    hooks: HashMap<String, Box<dyn Fn(&mut State)>>,
    pub ignore: bool,
    name: String,
    reporter_: ReporterType,
    result: Option<SuiteResult>,
    state_: State,
    suites_: Vec<Suite>,
    specs_: Vec<Spec>,
    export_: Option<String>
}
impl Suite {

    pub fn new(name: String) -> Suite {
        Suite {
            duration: 0,
            hooks: HashMap::new(),
            ignore: false,
            name,
            reporter_: ReporterType::Spec,
            result: None,
            state_: State::new(),
            suites_: vec![],
            specs_: vec![],
            export_: None
        }
    }
    pub fn run(mut self) -> Self {
        let start_time = Instant::now();
        self.run_(0);
        self.duration = start_time.elapsed().as_millis();
        self
    }
    pub fn specs(mut self, tests: Vec<Spec>) -> Self {
        self.specs_ = tests;
        self
    }
    pub fn suites(mut self, suites: Vec<Suite>) -> Self {
        self.suites_ = suites;
        self
    }
    pub fn state<'a, S: Deserialize<'a> + Serialize>(mut self, state: S) -> Self {
        // self.state_hash.insert(1, Box::new(state));
        self.state_.set_state(state);
        // self.bit_state = to_vec(&state).expect("Could not Deserialize state");
        self
    }
    pub fn skip (mut self) -> Self {
        self.ignore = true;
        self
    }

    // change reporter
    pub fn spec(mut self) -> Self {
        self.reporter_ = ReporterType::Spec;
        self
    }
    pub fn min(mut self) -> Self {
        self.reporter_ = ReporterType::Minimal;
        self
    }
    pub fn json(mut self) -> Self {
        self.reporter_ = ReporterType::Json;
        self
    }
    pub fn json_pretty(mut self) -> Self {
        self.reporter_ = ReporterType::JsonPretty;
        self
    }
    pub fn export_to(mut self, path: &str) -> Self {
        self.export_ = Some(path.to_string());
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
        let len = self.specs_.len();
        let mut only_id = None;

        // check for specs marked as only
        for i in 0..len {
            let test = &self.specs_[i];
            if test.only_ == true {
                only_id = Some(i);
                break;
            }
        }

        match only_id {
            Some(id) => {

                // set all other specs to be ignored
                for i in 0..len {
                    if i != id {
                        let spec = &mut self.specs_[i];
                        spec.ignore = true;
                    }
                }

            },
            None => { }
        }

        // run specs not marked with ignore
        for i in 0..len {
            self.execute_hook("before each");
            let spec = &mut self.specs_[i];
            if self.ignore == true {
                spec.ignore = true;
            }
            spec.run(&mut self.state_);
            result.update_from_spec(spec.export_results(&self.name));
            // (self.after_each_handle)();
            self.execute_hook("after each");
        }

        let len = self.suites_.len();
        for i in 0..len {
            let suite = self.suites_[i].borrow_mut();
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
        match self.hooks.get(hook_name) {
            Some(hook) => {
                (hook)(&mut self.state_);
            },
            None => {

            }
        }
    }
    fn report(&self) {
        let result = match &self.result {
            Some(result) => {
                match &self.reporter_ {
                    ReporterType::Spec => Reporter::spec(result.clone()),
                    ReporterType::Minimal => Reporter::min(result.clone()),
                    ReporterType::Json => Reporter::json(result.clone()),
                    ReporterType::JsonPretty => Reporter::json_pretty(result.clone())
                }
            },
            None => {
                // no result found
                String::from("result not found")
            }
        };
        match &self.export_ {
            Some(path) => {
                Reporter::export_to_file(&path, result);
            },
            None => {
                println!("\n{}\n\n", result);
            }
        }
    }
    fn get_completed_count(&self) -> u128 {
        let mut count: u128 = 0;
        for spec in self.specs_.iter() {
            if spec.ignore == false {
                count += 1;
            }
        }
        for suite in self.suites_.iter() {
            count += suite.get_completed_count();
        }
        count
    }

    pub fn before_all<H>(mut self, handle: H) -> Self
        where
            H: Fn(&mut State) + 'static
    {
        self.hooks.insert("before all".to_string(), Box::new(handle));
        self
    }
    pub fn before_each<H>(mut self, handle: H) -> Self
        where
            H: Fn(&mut State) + 'static
    {
        self.hooks.insert("before each".to_string(), Box::new(handle));
        self
    }
    pub fn after_all<H>(mut self, handle: H) -> Self
        where
            H: Fn(&mut State) + 'static
    {
        self.hooks.insert("after all".to_string(), Box::new(handle));
        self
    }
    pub fn after_each<H>(mut self, handle: H) -> Self
        where
            H: Fn(&mut State) + 'static
    {
        self.hooks.insert("after each".to_string(), Box::new(handle));
        self
    }

}