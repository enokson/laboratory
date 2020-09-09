use super::suite::SuiteResult;
use super::spec::SpecResult;
use std::collections::HashMap;
use console::style;

pub enum ReporterType {
    Spec,
    // DotMatrix,
    // Minimal,
    // Json,
    // Html
}

pub struct Reporter;
impl Reporter {
    pub fn spec(mut suite_results: SuiteResult) {

        fn display_spec_line(spec: &SpecResult, indention: u32) -> String {
            let mut ln = String::new();
            ln += &indent(indention + 5);
            ln += &style('✓').green().to_string();
            ln += &indent(2);
            let sub_ln = format!("{} ({}ms)", spec.get_name(), spec.get_duration());
            ln += &style(sub_ln).dim().to_string();
            ln
        }
        fn display_spec_err_ln(spec: &SpecResult, indention: u32, fail_id: &u64) -> String {
            let mut ln = String::new();
            ln += &indent(indention + 5);
            ln += &format!("{}) {} ({}ms)", &fail_id, spec.get_name(), spec.get_duration());
            ln = style(ln).red().to_string();
            ln
        }
        fn display_spec_ignored_ln(spec: &SpecResult, indention: u32,) -> String {
            let mut ln = String::new();
            ln += &indent(indention + 8);
            ln += &style(spec.get_name()).dim().to_string();
            // ln += &format!("{} ({}ms)", spec.get_name(), spec.get_duration());
            ln
        }
        fn display_suite_line(suite: &SuiteResult, indention: u32) -> String {
            let mut ln = String::new();
            ln += &indent(indention + 2);
            ln += suite.get_name();
            ln += "\n";
            ln
        }
        fn display_spec_err_msg(spec: &SpecResult, fail_id: &u64) -> String {
            let mut ln = String::new();
            ln += &indent(2);
            ln += &format!("{}) {}: ", style(fail_id).red(), spec.get_full_name());
            ln += &style(spec.get_err_msg()).red().to_string();
            ln += "\n";
            ln
        }
        fn get_spec_lines(spec: &SpecResult, ln: &mut String, fail_ln: &mut String, failed_id: &mut u64, indention: u32) {
            match spec.get_pass() {
                Some(pass) => {
                    if pass {
                        *ln += &display_spec_line(&spec, indention);
                    } else {
                        *ln += &display_spec_err_ln(&spec, indention, &failed_id);
                        *fail_ln += &display_spec_err_msg(&spec, &failed_id);
                        *failed_id += 1;
                    }
                },
                None => {
                    *ln += &display_spec_ignored_ln(&spec, indention);
                }
            }
            *ln += "\n"
        }
        fn get_all_spec_lines_from_result(suite: &mut SuiteResult, ln: &mut String, fail_ln: &mut String, failed_id: &mut u64, indention: u32) {
            // *ln += "\n";
            *ln += &display_suite_line(&suite, indention);
            // *ln += "\n";
            for spec in suite.get_child_specs() {
                get_spec_lines(&spec, ln, fail_ln, failed_id, indention);
            }
            for mut child_suite in suite.get_child_suites() {
                get_all_spec_lines_from_result(&mut child_suite, ln, fail_ln, failed_id, indention + 2);
            }
        }
        fn get_count(suite: &SuiteResult, count: &mut u64) -> u64 {
            *count += suite.get_child_specs().len() as u64;
            for child in suite.get_child_suites() {
                get_count(&child, count);
            }
            count.clone()
        }
        fn indent(indention: u32) -> String {
            let mut ln = String::new();
            for _i in 0..indention {
                ln += " ";
            }
            ln
        }

        let mut ln = String::new();
        let mut fail_ln = String::new();
        let mut failed_id = 0;
        ln += "\n\n";
        get_all_spec_lines_from_result(&mut suite_results, &mut ln, &mut fail_ln, &mut failed_id, 0);
        ln +=  "\n\n";
        ln += &indent(2);
        if failed_id == 0 {
            ln += &style('✓').green().to_string();
            let sub_line = format!(" {} tests completed", get_count(&suite_results, &mut 0));
            ln += &style(sub_line).green().to_string();
            ln += &style(format!(" ({}ms)", suite_results.get_duration())).dim().to_string();
        } else {
            ln += &style(format!("✖ {} of {} tests failed", failed_id, get_count(&suite_results, &mut 0))).red().to_string();
            ln += &style(":").dim().to_string();
            ln += "\n\n";
            ln += &fail_ln;
        }
        ln += "\n\n";
        println!("{}", ln);
    }
}
