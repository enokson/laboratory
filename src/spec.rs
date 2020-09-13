
use std::time::{Instant, Duration};
use super::suite::State;
use std::borrow::BorrowMut;
use serde::Serialize;

#[derive(Serialize)]
pub struct SpecResult {
    name: String,
    full_name: String,
    pass: Option<bool>,
    error_msg: Option<String>,
    // pub time_started: String,
    // pub time_ended: String,
    duration: u128
}
impl SpecResult {
    pub fn new(
        suite_name: &str,
        name: &str,
        pass: Option<bool>,
        err_msg: &Option<String>,
        time_started: Option<Instant>) -> SpecResult {
        let pass = match pass {
            Some(result) => Some(result.clone()),
            None => None
        };
        let error_msg = match err_msg {
            Some(result) => Some(result.clone()),
            None => None
        };
        let duration = match time_started {
            Some(instant) => instant.elapsed().as_millis(),
            None => 0
        };
        SpecResult {
            name: name.to_string(),
            full_name: format!("{} {}", suite_name.clone(), name.clone()),
            pass,
            error_msg,
            duration,
        }
    }
    pub fn get_pass(&self) -> Option<bool> {
        self.pass
    }
    pub fn update_passing(&self) -> u64 {
        match self.pass {
            Some(is_passing) => {
                if is_passing {
                    1
                } else {
                    0
                }
            },
            None => 0
        }
    }
    pub fn update_failing(&self) -> u64 {
        match self.pass {
            Some(is_passing) => {
                if is_passing {
                    0
                } else {
                    1
                }
            },
            None => 0
        }
    }
    pub fn update_ignored(&self) -> u64 {
        match self.pass {
            Some(_) => 0,
            None => 1
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_duration(&self) -> &u128 {
        &self.duration
    }
    pub fn get_full_name(&self) -> &str { &self.full_name }
    pub fn get_err_msg(&self) -> &str {
        match &self.error_msg {
            Some(err_msg) => err_msg,
            None => ""
        }
    }
}

impl Clone for SpecResult {
    fn clone (&self) -> SpecResult {
        let pass = match &self.pass {
            Some(result) => Some(result.clone()),
            None => None
        };
        let error_msg = match &self.error_msg {
            Some(result) => Some(result.clone()),
            None => None
        };
        SpecResult {
            name: self.name.clone(),
            full_name: self.full_name.clone(),
            pass,
            error_msg,
            duration: self.duration.clone(),
        }
    }
}

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