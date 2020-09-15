use std::borrow::{BorrowMut};
use std::collections::HashMap;
use std::time::Instant;
use std::path::Path;
use std::thread::Thread;
use console::style;

// use serde::de::Deserialize;
use serde::{Deserialize, Serialize};


use super::spec::Spec;
use super::reporter::{ReporterType, Reporter};
use super::state::State;
use super::suite_result::SuiteResult;

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
    export_: Option<String>,
    inherit_state_: bool
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
            export_: None,
            inherit_state_: false
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
    pub fn inherit_state(mut self) -> Self {
        self.inherit_state_ = true;
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
    pub fn to_state<'a, S: Deserialize<'a>>(&'a self) -> S {
        self.state_.get_state()
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
        let mut raw_state = self.get_state_raw();
        for i in 0..len {
            let suite = self.suites_[i].borrow_mut();
            if self.ignore == true {
                suite.ignore = true;
            }
            if suite.should_inherit() {
                // let raw_state = self.get_state_raw();
                suite.set_state_raw(&raw_state);
            }
            suite.run_(nest_count + 1);
            result.updated_from_suite(suite.clone_result());
            if suite.should_inherit() {
                raw_state = suite.get_state_raw();
               // self.set_state_raw(&suite.get_state_raw());
            }
        }
        self.set_state_raw(&raw_state);
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
/*    fn get_completed_count(&self) -> u128 {
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
    }*/

    // GETTERS
    pub fn should_inherit(&self) -> bool { self.inherit_state_ }
    // pub fn should_ignore(&self) -> bool { self.ignore }
    pub fn get_state_raw(&self) -> Vec<u8> {
        self.state_.get_raw_state().to_vec()
    }

    // SETTERS
    pub fn set_state_raw(&mut self, state: &Vec<u8>) {
        self.state_.set_raw_state(state.to_vec());
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