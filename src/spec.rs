use std::time::{Instant, Duration};

pub struct SpecResult {
    pub name: String,
    pub pass: Option<bool>,
    pub error_msg: Option<String>,
    // pub time_started: String,
    // pub time_ended: String,
    pub duration: u128,
}

pub struct Spec {
    pub name: String,
    pub test: Box<dyn Fn() -> Result<(), String>>,
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
            T: Fn() -> Result<(), String> + 'static
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
    pub fn run(&mut self) {
        let test = self.test.as_ref();
        if self.ignore == false {
            let start_time = Instant::now();
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
            self.time_started = Some(start_time);
            // self.time_ended = Some(Instant::now());
            self.duration = Some(start_time.elapsed().as_millis())
        }
    }
    fn export_results(&self) -> SpecResult {
        let pass = match &self.pass {
            Some(result) => Some(result.clone()),
            None => None
        };
        let error_msg = match &self.error_msg {
            Some(result) => Some(result.clone()),
            None => None
        };
        let duration = match &self.time_started {
            Some(instant) => instant.elapsed().as_millis(),
            None => 0
        };
        SpecResult {
            name: self.name.clone(),
            pass,
            error_msg,
            duration,
        }
    }
}