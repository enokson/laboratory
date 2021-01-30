use std::time::{Instant, Duration};
use std::borrow::BorrowMut;
use serde::Serialize;
use super::error::Error;
use super::spec_result::SpecResult;
use super::state::State;


pub struct Spec {
    pub name: String,
    pub test: Box<dyn FnMut(&mut State) -> Result<(), Error>>,
    pub pass: Option<bool>,
    pub error_msg: Option<Error>,
    pub ignore: bool,
    pub only_: bool,
    pub time_started: Option<Instant>,
    pub duration: Option<Duration>
}
impl Spec {
    pub fn new <T>(name: String, handle: T) -> Spec
        where
            T: FnMut(&mut State) -> Result<(), Error> + 'static
    {
        Spec {
            name,
            test: Box::new(handle),
            pass: None,
            error_msg: None,
            ignore: false,
            only_: false,
            time_started: None,
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
        let test: &mut dyn FnMut(&mut State) -> Result<(), Error> = self.test.borrow_mut();
        if !self.ignore {
            let start_time = Instant::now();
            match (test)(state) {
                Ok(_) => {
                    self.pass = Some(true);
                }
                Err(message) => {
                    self.pass = Some(false);
                    self.error_msg = Some(message);
                }
            }
            self.time_started = Some(start_time);
            self.duration = Some(start_time.elapsed())
        }
    }
    pub fn export_results(&self, suite_name: &str) -> SpecResult {
        SpecResult::new(suite_name, &self.name, self.pass, &self.error_msg, self.time_started)
    }

}
