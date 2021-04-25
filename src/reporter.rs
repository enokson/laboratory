use convert_case::{Case, Casing};
use console::{style};
use crate::suite::{
  Duration,
  DurationType,
  Speed,
  Suite
};
use serde::{Serialize};
use serde_json::{to_string, to_string_pretty};
use std::fmt::Display;

pub enum Reporter{
  Spec,
  Min,
  Dot,
  List,
  Rust,
  Tap,
  Json(bool) // true = pretty
}
enum SpeedDisplay {
  Fast(Duration),
  OnTime(Duration),
  Slow(Duration)
}
impl SpeedDisplay {
  pub fn to_string(&self) -> String {
    match self {
      SpeedDisplay::Fast(duration) => green(duration.to_string()),
      SpeedDisplay::OnTime(duration) => yellow(duration.to_string()),
      SpeedDisplay::Slow(duration) => red(duration.to_string())
    }
  }
}

#[derive(Debug, Serialize, Clone)]
struct JsonSpecReport {
  pub title: String,
  pub full_title: String,
  pub duration: u128,
  pub error: Option<String>,
  pub attempts: u32
}

impl JsonSpecReport {
  fn copy(&self) -> JsonSpecReport {
    JsonSpecReport {
      title: self.title.to_string(),
      full_title: self.full_title.to_string(),
      duration: self.duration,
      error: match &self.error {
        Some(msg) => Some(msg.to_string()),
        None => None
      },
      attempts: self.attempts
    }
  }
}

#[derive(Debug, Serialize)]
struct JsonStats {
  pub suites: u32,
  pub tests: u32,
  pub passing: u32,
  pub pending: u32,
  pub failing: u32,
  pub start: String,
  pub end: String,
  pub duration: u128
}

#[derive(Debug, Serialize)]
struct JsonReport {
  pub stats: JsonStats,
  pub tests: Vec<JsonSpecReport>,
  pub passing: Vec<JsonSpecReport>,
  pub pending: Vec<JsonSpecReport>,
  pub failing: Vec<JsonSpecReport>
}

struct MinReporterStats {
  pub passed: u32,
  pub failed: u32,
  pub pending: u32,
  pub error_lines: Vec<String>
}

struct DotReporterStats {
  pub passed: u32,
  pub failed: u32,
  pub pending: u32,
  pub dots: Vec<String>
}

fn red<T: Into<String> + Display>(text: T) -> String {
  style(text).red().for_stdout().to_string()
}
fn green<T: Into<String> + Display>(text: T) -> String {
  style(text).green().for_stdout().to_string()
}
fn cyan<T: Into<String> + Display>(text: T) -> String {
  style(text).cyan().for_stdout().to_string()
}
fn dim<T: Into<String> + Display>(text: T) -> String {
  style(text).dim().for_stdout().to_string()
}
fn yellow<T: Into<String> + Display>(text: T) -> String {
  style(text).yellow().for_stdout().to_string()
}

fn header() {
  print!("\n\n### Lab Results Start ###\n\n");
}

fn footer() {
  println!("\n### Lab Results End ###\n\n");
}

fn get_lines_for_spec<T>(suite: &Suite<T>, depth: u32, stats: &mut MinReporterStats) {

  println!("{}{}", suite_spacing(depth), suite.name.to_string());
  for spec in &suite.context.specs {
    if let Some(result) = &spec.result {
      if let Err(msg) = result {
        println!("{}{} {}", 
          line_spacing(depth),
          red(&format!("{})", stats.failed)),
          red(&spec.name.to_string()));
        stats.error_lines.push(red(&format!("{}) {}: {}", stats.failed, spec.name, msg)));
        stats.failed += 1;
      } else {
        let duration = match suite.duration_type {
          DurationType::Nano => Duration::Nano(spec.duration),
          DurationType::Micro => Duration::Micro(spec.duration),
          DurationType::Mil => Duration::Mil(spec.duration),
          DurationType::Sec => Duration::Sec(spec.duration)
        };
        let speed_display = match spec.context.speed_result {
          Speed::Fast => SpeedDisplay::Fast(duration),
          Speed::OnTime => SpeedDisplay::OnTime(duration),
          Speed::Slow => SpeedDisplay::Slow(duration)
        };
        println!("{}{}  {} {}", 
          line_spacing(depth),
          green("✓"),
          dim(&spec.name.to_string()), 
          speed_display.to_string());
          stats.passed += 1;
      }
    } else {
      println!("{}   {}", 
        line_spacing(depth), 
        dim(&spec.name.to_string()));
        stats.pending += 1;
    }
  }
  for child_suite in &suite.context.suites {
    get_lines_for_spec(child_suite, depth + 1, stats);

  }
}

fn suite_spacing(depth: u32) -> String {
  let mut tab = String::new();
  let tab_n = depth * 2;
  for _i in 1..=tab_n {
    tab.push(' ');
  }
  tab
}

fn line_spacing(depth: u32) -> String {
  let mut tab = String::new();
  let tab_n = (depth * 2) + 2;
  for _i in 1..=tab_n {
    tab.push(' ');
  }
  tab
}

fn space_per_byte(n: u32) -> String {
  let len = n.to_string().len();
  let mut return_str = String::new();
  for _i in 0..len {
    return_str.push(' ');
  }
  return_str
}

fn line_spacing_for_min(depth: u32) -> String {
  let mut tab = String::new();
  let tab_n = (depth * 2) + 2;
  for _i in 1..=tab_n {
    tab.push(' ');
  }
  tab
}  

fn get_lines_for_min<T>(suite: &Suite<T>, stats: &mut MinReporterStats, prefix: String, depth: u32) {
  for spec in &suite.context.specs {
    if let Some(result) = &spec.result {
      if let Err(msg) = result {
        stats.failed += 1;
        stats.error_lines.push(
          red(&format!("{}) {}\n   {}{}\n{}  Error: {}", 
            stats.failed, prefix, 
            line_spacing_for_min(depth), 
            spec.name, 
            space_per_byte(stats.failed), 
            msg)
          )
        );
      } else {
        stats.passed += 1;
      }
    } else {
      stats.pending += 1;
    }
  }
  for child_suite in &suite.context.suites {
    get_lines_for_min(child_suite, stats, format!("{}\n   {}{}", prefix, line_spacing_for_min(depth), child_suite.name), depth + 1);
  }
}

fn get_dots<T>(suite: &Suite<T>, stats: &mut DotReporterStats) {
  for spec in &suite.context.specs {
    if let Some(result) = &spec.result {
      match result {
        Ok(_) => {
          stats.passed += 1;
          match spec.context.speed_result {
            Speed::Fast => {
              stats.dots.push(green("."))
            },
            Speed::OnTime => {
              stats.dots.push(yellow("."))
            },
            Speed::Slow => {
              stats.dots.push(red("."))
            }
          }
        },
        Err(_) => {
          stats.failed += 1;
          stats.dots.push(red("!"))
        }
      }
    } else {
      stats.pending += 1;              
      stats.dots.push(cyan(","))
    }
  }
  for child_suite in &suite.context.suites {
    get_dots(child_suite, stats);
  }
}

fn get_list<T>(suite: &Suite<T>, stats: &mut MinReporterStats, prefix: String) {
  for spec in &suite.context.specs {
    if let Some(result) = &spec.result {
      if let Err(msg) = result {
        println!("✖ {}",
          red(format!("{} {}: {}", prefix, spec.name, msg))
        );
        stats.failed += 1;
      } else {
        let duration = match suite.duration_type {
          DurationType::Nano => Duration::Nano(spec.duration),
          DurationType::Micro => Duration::Micro(spec.duration),
          DurationType::Mil => Duration::Mil(spec.duration),
          DurationType::Sec => Duration::Sec(spec.duration)
        };
        println!("✓ {}{}",
          green(format!("{} {}", prefix, spec.name)),
          dim(format!(": {}", duration.to_string()))
        );
        stats.passed += 1;
      }
    } else {
      println!("  {}",
        dim(format!("{} {}", prefix, spec.name))
      );
      stats.pending += 1;
    }
  }
  for child_suite in &suite.context.suites {
    get_list(child_suite, stats, format!("{} {}", prefix, child_suite.name));
  }
}

fn get_tap_list<T>(suite: &Suite<T>, lines: &mut Vec<String>, count: &mut u32, prefix: String) {
  for spec in &suite.context.specs {
    *count += 1;
    if let Some(result) = &spec.result {
      if let Err(_msg) = result {
        lines.push(format!("{} {} - {}",
          red("not ok"),
          count,
          format!("{} {}", prefix, spec.name)
        ));
      } else {
        lines.push(format!("{} {} - {}",
          green("ok"),
          count,
          format!("{} {}", prefix, spec.name)
        ));
      }
    } else {
      lines.push(format!("{} {} - {}",
        green("ok"),
        count,
        format!("# skip {} {}", prefix, spec.name)
      ));
    }
  }
  for child_suite in &suite.context.suites {
    get_tap_list(child_suite, lines, count, format!("{} {}", prefix, child_suite.name));
  }
}

fn get_count<T>(suite: &Suite<T>) -> u32 {
  let mut count = suite.context.specs.len() as u32;
  for child_suite in &suite.context.suites {
    count += get_count(child_suite);
  }
  return count;
}

fn get_list_for_rust<T>(suite: &Suite<T>, stats: &mut MinReporterStats, prefix: String) {
  for spec in &suite.context.specs {
    if let Some(result) = &spec.result {
      if let Err(_msg) = result {
        println!("test {}::{} ... {}", prefix, spec.name.to_case(Case::Snake), red("FAILED"));
        stats.error_lines.push(format!("{}::{}", prefix, spec.name.to_case(Case::Snake)));
        stats.failed += 1;
      } else {
        println!("test {}::{} ... {}", prefix, spec.name.to_case(Case::Snake), green("ok"));
        stats.passed += 1;
      }
    } else {
      println!("test {}::{} ... {}", prefix, spec.name.to_case(Case::Snake), cyan("ignored"));
      stats.pending += 1;
    }
  }
  for child_suite in &suite.context.suites {
    get_list_for_rust(child_suite, stats, format!("{}::{}", prefix, child_suite.name.to_case(Case::Snake)));
  }
}

fn get_stats_for_json<T>(suite: &Suite<T>, stats: &mut JsonReport, prefix: String) {
  for spec in &suite.context.specs {
    let mut spec_stat = JsonSpecReport {
      title: spec.name.to_string(),
      full_title: format!("{} {}", prefix, spec.name),
      duration: spec.duration,
      error: None,
      attempts: spec.context.attempts
    };
    if let Some(result) = &spec.result {
      if let Err(msg) = result {
        spec_stat.error = Some(msg.to_string());
        stats.stats.failing += 1;
        stats.failing.push(spec_stat.copy());
      } else {
        stats.passing.push(spec_stat.copy());
        stats.stats.passing += 1;
      }
    } else {
      stats.pending.push(spec_stat.copy());
      stats.stats.pending += 1;
    }
    stats.stats.duration += spec.duration;
    stats.tests.push(spec_stat);
  }
  for child_suite in &suite.context.suites {
    stats.stats.suites += 1;
    get_stats_for_json(child_suite, stats, format!("{} {}", prefix, child_suite.name));
  }
}

fn get_suffix(n: u32) -> String {
  if n > 1 {
    "s".to_string()
  } else {
    "".to_string()
  }
}

pub fn report_to_stdout<T>(suite: &Suite<T>) {

  match suite.reporter {
    Reporter::Spec => {

      let mut stats = MinReporterStats {
        passed: 0,
        pending: 0,
        failed: 0,
        error_lines: vec![]
      };
 
      header();
 
      get_lines_for_spec(suite, 0, &mut stats);
 
      if stats.failed == 0 {
 
        let duration = match suite.duration_type {
          DurationType::Nano => Duration::Nano(suite.total_duration),
          DurationType::Micro => Duration::Micro(suite.total_duration),
          DurationType::Mil => Duration::Mil(suite.total_duration),
          DurationType::Sec => Duration::Sec(suite.total_duration)
        };
 
        println!("{}{} {}",
          green("✓"), 
          green(format!(" {} test{} completed", stats.passed, get_suffix(stats.passed))),
          dim(format!("{}", duration.to_string()))
        );
  
      } else {
 
        println!(" {}{}",
          red(format!("✖ {} of {} test{} failed", 
            stats.failed, 
            stats.passed + stats.failed, 
            get_suffix(stats.failed))
          ),
          dim(":")
        );
 
      }
 
      for spec_report_line in &stats.error_lines {
        println!("{}", spec_report_line);
      }

      footer();

    },
    Reporter::Min => {


      // 19 passing (698ms)
      // 1 failing
    
      // 1) Store
      //      #put()
      //        is not a real test:
      //    Error: the string "my error" was thrown, throw an Error :)
      //     at processImmediate (node:internal/timers:463:21)


      let mut stats = MinReporterStats {
        passed: 0,
        failed: 0,
        pending: 0,
        error_lines: vec![]
      };


      get_lines_for_min(suite, &mut stats, suite.name.to_string(), 0);
      
      let duration = match suite.duration_type {
        DurationType::Nano => Duration::Nano(suite.total_duration),
        DurationType::Micro => Duration::Micro(suite.total_duration),
        DurationType::Mil => Duration::Mil(suite.total_duration),
        DurationType::Sec => Duration::Sec(suite.total_duration)
      };

      header();
  
      if stats.passed > 0 {
        println!("{} {}",
          green(format!("{} test{} complete", stats.passed, get_suffix(stats.passed))),
          dim(format!("{}", duration.to_string()))
        );
      }
      if stats.pending > 0 {
        println!("{}", dim(format!("{} test{} pending", stats.pending, get_suffix(stats.pending))));
      }
      if stats.failed > 0 {
        println!("{}",
          red(format!("{} test{} failed", stats.failed, get_suffix(stats.failed)))
        );
        print!("\n\n");
        for line in &stats.error_lines {
          println!("{}", line);
          println!("");
        }
      }

      footer();

   },
    Reporter::Dot => {
      let mut stats = DotReporterStats {
        passed: 0,
        failed: 0,
        pending: 0,
        dots: vec![]
      };
      get_dots(suite, &mut stats);
      header();
      for line in &stats.dots {
        print!("{}", line);
      }
      print!("\n\n");
      println!("{}", green(format!("{} passing", stats.passed)));
      println!("{}", cyan(format!("{} pending", stats.pending)));
      println!("{}", red(format!("{} failed", stats.failed)));
      footer();
    },
    Reporter::List => {
      let mut stats = MinReporterStats {
        passed: 0,
        failed: 0,
        pending: 0,
        error_lines: vec![]
      };
      let duration = match suite.duration_type {
        DurationType::Nano => Duration::Nano(suite.total_duration),
        DurationType::Micro => Duration::Micro(suite.total_duration),
        DurationType::Mil => Duration::Mil(suite.total_duration),
        DurationType::Sec => Duration::Sec(suite.total_duration)
      };
      get_list(suite, &mut stats, suite.name.to_string());
      header();
      println!("{}", green(format!("{} passing {}", stats.passed, duration.to_string())));
      println!("{}", cyan(format!("{} pending", stats.pending)));
      println!("{}", red(format!("{} failed", stats.failed)));
      footer();
    },
    Reporter::Tap => {
      let mut lines = vec![];
      let mut count = 0;
      get_tap_list(suite, &mut lines, &mut count, suite.name.to_string());
      header();
      println!("{}", green(format!("1..{}", count)));
      for line in &lines {
        println!("{}", line);
      }
      footer();
    },
    Reporter::Rust => {

      // test suite::tests::describe_a_suite ... FAILED
      // failures:
      //     suite::tests::describe_a_suite
      // test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

      let mut stats = MinReporterStats {
        passed: 0,
        failed: 0,
        pending: 0,
        error_lines: vec![]
      };
      let count = get_count(suite);
      header();
      println!("Running {} test{}", count, get_suffix(count));
      print!("\n\n");
      get_list_for_rust(suite, &mut stats, suite.name.to_case(Case::Snake));
      print!("\n");
      let passed = green(&format!("{} passed", stats.passed));
      let ignored = cyan(&format!("{} ignored", stats.pending));      
      if stats.failed == 0 {
        println!("test result: {}. {}; 0 failed; {}; 0 measured; 0 filtered out", green("ok"), passed, ignored);
      } else {
        let failed = red(&format!("{} failed", stats.failed));
        println!("{}", red("failures:"));
        for line in &stats.error_lines {
          println!("    {}", red(line));            
        }
        print!("\n");
        println!("test result: {}. {}; {}; {}; 0 measured; 0 filtered out", red("FAILED"), passed, failed, ignored);
      }
      footer();
    },
    Reporter::Json(pretty) => {
      let mut json_report = JsonReport {
        stats: JsonStats {
          suites: 0,
          tests: 0,
          passing: 0,
          pending: 0,
          failing: 0,
          start: suite.start_time.to_string(),
          end: suite.end_time.to_string(),
          duration: 0
        },
        tests: vec![],
        passing: vec![],
        pending: vec![],
        failing: vec![]
      };
      get_stats_for_json(suite, &mut json_report, suite.name.to_string());
      header();
      if pretty {
        if let Ok(json) = to_string_pretty(&json_report) {
          println!("{}", json);
        } else {
          println!("Could not print out json result");
        }          
      } else {
        if let Ok(json) = to_string(&json_report) {
          println!("{}", json);
        } else {
          println!("Could not print out json result");
        }
      }
      footer();
    }
  }    
}
