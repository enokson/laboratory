use std::time::{Instant, Duration};
use std::borrow::BorrowMut;
use serde::Serialize;
use super::spec_result::SpecResult;
use super::state::State;


pub struct Spec {
    pub name: String,
    pub test: Box<dyn FnMut(&mut State) -> Result<(), String>>,
    pub pass: Option<bool>,
    pub error_msg: Option<String>,
    pub ignore: bool,
    pub only_: bool,
    pub time_started: Option<Instant>,
    // pub time_ended: Option<Instant>,
    pub duration: Option<u128>
}
impl Spec {
    pub fn new <T>(name: String, handle: T) -> Spec
        where
            T: FnMut(&mut State) -> Result<(), String> + 'static
    {
        Spec {
            name,
            test: Box::new(handle),
            pass: None,
            error_msg: None,
            ignore: false,
            only_: false,
            time_started: None,
            // time_ended: None,
            duration: None
        }
    }
    pub fn skip (mut self) -> Self {
        self.ignore = true;
        self
    }
    pub fn only(mut self) -> Self {
        self.only_ = true;
        self
    }
    pub fn run(&mut self, state: &mut State) {
        let test: &mut dyn FnMut(&mut State) -> Result<(), String> = self.test.borrow_mut();
        if self.ignore == false {
            let start_time = Instant::now();
            match (test)(state) {
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
            self.time_started = Some(start_time);
            // self.time_ended = Some(Instant::now());
            self.duration = Some(start_time.elapsed().as_millis())
        }
    }
    pub fn export_results(&self, suite_name: &str) -> SpecResult {
        SpecResult::new(suite_name, &self.name, self.pass, &self.error_msg, self.time_started)
    }
}