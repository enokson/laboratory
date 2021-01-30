use std::collections::HashMap;
use std::path::{Path};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::time::Duration;

use console::style;
use serde_json::{to_string, to_string_pretty};

use super::suite::DurationPrecision;
use super::suite_result::SuiteResult;
use super::spec_result::SpecResult;

fn get_count(suite: &SuiteResult, count: &mut u64) -> u64 {
    *count += suite.get_passing() + suite.get_failing() as u64;
    // for child in suite.get_child_suites() {
    //     get_count(&child, count);
    // }
    *count
}
fn indent(indention: u32, mut ln: String) -> String {
    for _i in 0..indention {
        ln += " ";
    }
    ln
}
fn display_spec_err_msg(spec: &SpecResult, fail_id: &u64, stdout: bool, mut ln: String) -> String {
    ln = indent(2, ln);
    if stdout {
        ln += &format!("{}) {}: ", style(fail_id).red(), spec.get_full_name());
        ln += &style(spec.get_err_msg()).red().to_string();

    } else {
        ln += &format!("{}) {}: ", fail_id, spec.get_full_name());
        ln += &spec.get_err_msg();
    }
    ln += "\n";
    ln
}
fn get_duration(duration: &Duration, precision: &DurationPrecision) -> String {
    match precision {
        DurationPrecision::Mil => format!("{}ms", duration.as_millis()),
        DurationPrecision::Micro => format!("{}μs", duration.as_micros()),
        DurationPrecision::Nano => format!("{}ns", duration.as_nanos()),
        DurationPrecision::Sec => format!("{}sec", duration.as_secs())
    }
}
fn display_spec_line(spec: &SpecResult, indention: u32, stdout: bool, precision: &DurationPrecision, mut ln: String) -> String {
    ln = indent(indention + 5, ln);
    if stdout {
        ln += &style('✓').green().to_string();
    } else {
        ln += "✓";
    }
    ln = indent(2, ln);
    let sub_ln = format!("{} ({})", spec.get_name(), get_duration(spec.get_duration(), precision));
    if stdout {
        ln += &style(sub_ln).dim().to_string();
    } else {
        ln += &sub_ln
    }
    ln
}
fn display_spec_err_ln(spec: &SpecResult, indention: u32, fail_id: &u64, stdout: bool, precision: &DurationPrecision, mut ln: String) -> String {
    ln = indent(indention + 5, ln);
    ln += &format!("{}) {} ({})", &fail_id, spec.get_name(), get_duration(spec.get_duration(), precision));
    if stdout {
        ln = style(ln).red().to_string();
    }
    ln
}
fn display_spec_ignored_ln(spec: &SpecResult, indention: u32, stdout: bool, mut ln: String) -> String {
    ln = indent(indention + 8, ln);
    if stdout {
        ln += &style(spec.get_name()).dim().to_string();
    } else {
        ln += spec.get_name()
    }

    // ln += &format!("{} ({}ms)", spec.get_name(), spec.get_duration());
    ln
}
fn display_suite_line(suite: &SuiteResult, indention: u32, mut ln: String) -> String {
    ln = indent(indention + 2, ln);
    ln += suite.get_name();
    ln += "\n";
    ln
}

fn display_success_line(suite_results: &SuiteResult, stdout: bool, precision: &DurationPrecision, mut ln: String) -> String {
    if stdout {
        ln += &style('✓').green().to_string();
        let sub_line = format!(" {} tests completed", get_count(&suite_results, &mut 0));
        ln += &style(sub_line).green().to_string();
        ln += &style(format!(" ({})", get_duration(&suite_results.get_duration(), precision))).dim().to_string();
    } else {
        ln += "✓";
        ln += &format!(" {} tests completed", get_count(&suite_results, &mut 0));
        ln += &format!(" ({})", get_duration(&suite_results.get_duration(), precision));
    }
    ln
}
fn display_error_lines(suite_results: &SuiteResult, failed_id: u64, fail_ln: String, stdout: bool, mut ln: String) -> String {
    if stdout {
        ln += &style(format!("✖ {} of {} tests failed", failed_id, get_count(&suite_results, &mut 0))).red().to_string();
        ln += &style(":").dim().to_string();
    } else {
        ln += &format!("✖ {} of {} tests failed", failed_id, get_count(&suite_results, &mut 0));
        ln += ":";
    }
    ln += "\n\n";
    ln += &fail_ln;
    ln.pop();
    ln
}
fn display_test_result(suite_results: &SuiteResult, stdout: bool, mut ln: String, fail_ln: String, failed_id: u64, precision: &DurationPrecision) -> String {
    if failed_id == 0 {
        ln = display_success_line(&suite_results, stdout, precision, ln);
    } else {
        ln = display_error_lines(&suite_results, failed_id, fail_ln, stdout, ln);
    }
    ln
}

fn add_padding(stdout: bool, ln: String) -> String {
    if stdout {
        format!("\n\n{}\n\n", ln)
    } else {
        format!("{}\n", ln)
    }
}

pub enum ReporterType {
    Spec,
    // Dot,
    Minimal,
    Json,
    JsonPretty
    // Html
}

pub struct Reporter;
impl Reporter {
    pub fn spec(mut suite_results: SuiteResult, stdout: bool, precision: &DurationPrecision) -> String {

        fn get_spec_lines(spec: &SpecResult, indention: u32, stdout: bool, mut ln: String,
                          mut fail_ln: String, mut failed_id: u64, precision: &DurationPrecision) -> (String, String, u64) {
            match spec.get_pass() {
                Some(pass) => {
                    if pass {
                        ln = display_spec_line(&spec, indention, stdout, precision, ln);
                    } else {
                        ln = display_spec_err_ln(&spec, indention, &failed_id, stdout, precision, ln);
                        fail_ln = display_spec_err_msg(&spec, &failed_id, stdout, fail_ln);
                        failed_id += 1;
                    }
                },
                None => {
                    ln = display_spec_ignored_ln(&spec, indention, stdout, ln);
                }
            }
            ln += "\n";
            (ln, fail_ln, failed_id)
        }
        fn get_all_spec_lines_from_result(suite: &mut SuiteResult, indention: u32, stdout: bool,
                                          mut ln: String, mut fail_ln: String, mut failed_id: u64, precision: &DurationPrecision) -> (String, String, u64) {
            // *ln += "\n";
            ln = display_suite_line(&suite, indention, ln);
            // *ln += "\n";
            for spec in suite.get_child_specs() {
                let r = get_spec_lines(&spec, indention, stdout, ln, fail_ln, failed_id, &precision);
                ln = r.0;
                fail_ln = r.1;
                failed_id = r.2;
            }
            for mut child_suite in suite.get_child_suites() {
                let r = get_all_spec_lines_from_result(&mut child_suite, indention + 2, stdout, ln, fail_ln, failed_id, &precision);
                ln = r.0;
                fail_ln = r.1;
                failed_id = r.2;
            }
            (ln, fail_ln, failed_id)
        }

        let mut ln = String::new();
        let fail_ln = String::new();
        let failed_id = 0;
        let r = get_all_spec_lines_from_result(&mut suite_results, 0, stdout, ln, fail_ln, failed_id, precision);
        ln = r.0;
        ln +=  "\n\n";
        ln = indent(2, ln);
        ln = display_test_result(&suite_results, stdout, ln, r.1, r.2, precision);
        add_padding(stdout, ln)
        
    }
    pub fn min(mut suite_results: SuiteResult, stdout: bool, precision: &DurationPrecision) -> String {

        fn get_spec_lines(spec: &SpecResult, stdout: bool, fail_ln: String, failed_id: u64) -> (String, u64) {
            match spec.get_pass() {
                Some(pass) => {
                    if !pass {
                        ( display_spec_err_msg(&spec, &failed_id, stdout, fail_ln), failed_id + 1 )
                    } else {
                        ( fail_ln, failed_id )
                    }
                },
                None => ( fail_ln, failed_id )
            }
        }
        fn get_all_spec_lines_from_result(suite: &mut SuiteResult, indention: u32, stdout: bool, mut ln: String, mut fail_ln: String, mut failed_id: u64) -> (String, String, u64) {
            for spec in suite.get_child_specs() {
                let r = get_spec_lines(&spec, stdout, fail_ln, failed_id);
                fail_ln = r.0;
                failed_id = r.1;
            }
            for mut child_suite in suite.get_child_suites() {
                let r = get_all_spec_lines_from_result(&mut child_suite, indention + 2, stdout, ln, fail_ln, failed_id);
                ln = r.0;
                fail_ln = r.1;
                failed_id = r.2;
            }
            (ln, fail_ln, failed_id)
        }

        let mut ln = String::new();
        let fail_ln = String::new();
        let failed_id = 0;
        if stdout {
            ln += "\n\n";
        }
        let r = get_all_spec_lines_from_result(&mut suite_results, 0, stdout, ln, fail_ln, failed_id);
        // ln +=  "\n\n";
        ln = indent(2, r.0);
        ln = display_test_result(&suite_results, stdout, ln, r.1, r.2, precision);
        add_padding(stdout, ln)

    }
    pub fn json(suite_results: SuiteResult) -> String {
        to_string(&suite_results).expect("Could not send to JSON")
    }
    pub fn json_pretty(suite_results: SuiteResult) -> String {
        to_string_pretty(&suite_results).expect("Could not send to JSON")
    }
    pub fn export_to_file(output: &str, report: &str) {
        if let Some(parent) = Path::new(output).parent() {
            if !parent.exists() {
                create_dir_all(parent).unwrap_or_else(|_| panic!("Could not create {:#?}", output));
            }
        }
        let mut file = File::create(output).expect("Could not create output file");
        file.write_all(report.as_bytes()).expect("Could not output to file");
    }
}
