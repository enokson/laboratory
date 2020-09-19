use serde::Serialize;
use super::spec_result::SpecResult;

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
    // pub fn add_spec_result (&mut self, spec: SpecResult) {
    //     self.child_tests.push(spec);
    // }
    pub fn updated_from_suite(&mut self, child_result_option: Option<SuiteResult>) {
        if let Some(child_result) = child_result_option {
            self.passing += child_result.get_passing();
            self.failing += child_result.get_failing();
            self.ignored += child_result.get_ignored();
            self.child_suites.push(child_result);
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
            passing: self.passing,
            failing: self.failing,
            ignored: self.ignored,
            child_suites: self.child_suites.clone(),
            child_tests: self.child_tests.clone(),
            duration: self.duration
        }
    }
}