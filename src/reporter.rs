use super::spec::Spec;

pub enum Reporter {
    Spec,
    // DotMatrix,
    // Minimal,
    // Json,
    // Html
}

pub struct SpecReporter {
    passing: u64,
    failing: u64,
    ignored: u64,
    error_messages: String,
    report: String,
    duration: u128
}
impl SpecReporter {
    pub fn new() -> SpecReporter {
        SpecReporter {
            passing: 0,
            failing: 0,
            ignored: 0,
            error_messages: "".to_string(),
            report: "".to_string(),
            duration: 0
        }
    }
    pub fn handle_spec(&mut self, spec: &Spec) {

    }
}
