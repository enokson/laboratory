use std::time::{Instant, Duration};
use serde::{Deserialize, Serialize};
use crate::error;

use super::error::Error;

#[derive(Deserialize, Serialize)]
pub struct SpecResult {
    name: String,
    full_name: String,
    pass: Option<bool>,
    error_msg: Option<Error>,
    duration: Duration
}
impl SpecResult {
    pub fn new(
        suite_name: &str,
        name: &str,
        pass: Option<bool>,
        err_msg: &Option<Error>,
        time_started: Option<Instant>) -> SpecResult {
        let pass = match pass {
            Some(result) => Some(result),
            None => None
        };
        let error_msg = match err_msg {
            Some(result) => Some(result.clone()),
            None => None
        };
        let duration = match time_started {
            Some(instant) => instant.elapsed(),
            None => Duration::new(0,0)
        };
        SpecResult {
            name: name.to_string(),
            full_name: format!("{} {}", suite_name, name),
            pass,
            error_msg: match error_msg {
                Some(error) => match error {
                    Error::Assertion(msg) => Some(Error::Assertion(msg.to_string())),
                    Error::Deserialize => Some(Error::Deserialize),
                    Error::Serialize => Some(Error::Serialize),
                    Error::ResultsNotFound => Some(Error::ResultsNotFound)
                }
                None => None
            },
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
    pub fn get_duration(&self) -> &Duration {
        &self.duration
    }
    pub fn get_full_name(&self) -> &str { &self.full_name }
    pub fn get_err_msg(&self) -> String {
        match &self.error_msg {
            Some(err_msg) => err_msg.to_string(),
            None => "".to_string()
        }
    }
}

impl Clone for SpecResult {
    fn clone (&self) -> SpecResult {
        let pass = match &self.pass {
            Some(result) => Some(*result),
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
            duration: self.duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn err_msg_spec() {
        let r = SpecResult::new("name", "name", None, &None, None);
        assert_eq!(r.get_err_msg(), "");
    }

}