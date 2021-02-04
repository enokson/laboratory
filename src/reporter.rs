use convert_case::{Case, Casing};
use console::{style, Style};
use crate::suite::{
  Duration,
  DurationType,
  Speed,
  Suite
};
use serde::{Serialize};
use serde_json::{to_string, to_string_pretty};
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
      SpeedDisplay::Fast(duration) => style(duration.to_string()).green().to_string(),
      SpeedDisplay::OnTime(duration) => style(duration.to_string()).yellow().to_string(),
      SpeedDisplay::Slow(duration) => style(duration.to_string()).red().to_string()
    }
  }
}

#[derive(Debug, Serialize, Clone)]
struct JsonSpecReport {
  pub title: String,
  pub full_title: String,
  pub duration: u128,
  pub error: Option<String>
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
      }
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

fn red(text: &str) -> String {
  style(text).red().to_string()
}
fn green(text: &str) -> String {
  style(text).green().to_string()
}
fn cyan(text: &str) -> String {
  style(text).cyan().to_string()
}
fn dim(text: &str) -> String {
  style(text).dim().to_string()
}

fn get_lines_for_spec(suite: &Suite, depth: u32, stats: &mut MinReporterStats) {

  let red = Style::new().for_stdout().red();

  println!("{}{}", suite_spacing(depth), suite.name.to_string());
  for spec in &suite.context.specs {
    if let Some(result) = &spec.result {
      if let Err(msg) = result {
        println!("{}{} {}", 
          line_spacing(depth),
          red.apply_to(format!("{})", stats.failed)),
          red.apply_to(spec.name.to_string()));
        stats.error_lines.push(style(format!("{}) {}: {}", stats.failed, spec.name, msg)).red().to_string());
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
          style("✓").green().to_string(),
          style(spec.name.to_string()).dim().to_string(), 
          speed_display.to_string());
          stats.passed += 1;
      }
    } else {
      println!("{}   {}", 
        line_spacing(depth), 
        style(spec.name.to_string()).dim().to_string());
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

fn get_lines_for_min(suite: &Suite, stats: &mut MinReporterStats, prefix: String, depth: u32) {
  for spec in &suite.context.specs {
    if let Some(result) = &spec.result {
      if let Err(msg) = result {
        stats.failed += 1;
        stats.error_lines.push(
          style(format!("{}) {}\n   {}{}\n{}  Error: {}", 
            stats.failed, prefix, 
            line_spacing_for_min(depth), 
            spec.name, 
            space_per_byte(stats.failed), 
            msg)).red().to_string()
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

fn get_dots(suite: &Suite, stats: &mut DotReporterStats) {
  for spec in &suite.context.specs {
    if let Some(result) = &spec.result {
      match result {
        Ok(_) => {
          stats.passed += 1;
          match spec.context.speed_result {
            Speed::Fast => {
              stats.dots.push(style(".").green().to_string())
            },
            Speed::OnTime => {
              stats.dots.push(style(".").yellow().to_string())
            },
            Speed::Slow => {
              stats.dots.push(style(".").red().to_string())
            }
          }
        },
        Err(_) => {
          stats.failed += 1;
          stats.dots.push(style("!").red().to_string())
        }
      }
    } else {
      stats.pending += 1;              
      stats.dots.push(style(",").cyan().to_string())
    }
  }
  for child_suite in &suite.context.suites {
    get_dots(child_suite, stats);
  }
}

fn get_list(suite: &Suite, stats: &mut MinReporterStats, prefix: String) {
  for spec in &suite.context.specs {
    if let Some(result) = &spec.result {
      if let Err(msg) = result {
        println!("✖ {}",
          style(format!("{} {}: {}", prefix, spec.name, msg)).red().to_string()
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
          style(format!("{} {}", prefix, spec.name)).green().to_string(),
          style(format!(": {}", duration.to_string())).dim().to_string()
        );
        stats.passed += 1;
      }
    } else {
      println!("  {}",
        style(format!("{} {}", prefix, spec.name)).dim().to_string()
      );
      stats.pending += 1;
    }
  }
  for child_suite in &suite.context.suites {
    get_list(child_suite, stats, format!("{} {}", prefix, child_suite.name));
  }
}

fn get_tap_list(suite: &Suite, lines: &mut Vec<String>, count: &mut u32, prefix: String) {
  for spec in &suite.context.specs {
    *count += 1;
    if let Some(result) = &spec.result {
      if let Err(_msg) = result {
        lines.push(format!("{} {} - {}",
          style("not ok").red().to_string(),
          count,
          format!("{} {}", prefix, spec.name)
        ));
      } else {
        lines.push(format!("{} {} - {}",
          style("ok").green().to_string(),
          count,
          format!("{} {}", prefix, spec.name)
        ));
      }
    } else {
      lines.push(format!("{} {} - {}",
        style("ok").green().to_string(),
        count,
        format!("# skip {} {}", prefix, spec.name)
      ));
    }
  }
  for child_suite in &suite.context.suites {
    get_tap_list(child_suite, lines, count, format!("{} {}", prefix, child_suite.name));
  }
}

fn get_count(suite: &Suite) -> u32 {
  let mut count = suite.context.specs.len() as u32;
  for child_suite in &suite.context.suites {
    count += get_count(child_suite);
  }
  return count;
}

fn get_list_for_rust(suite: &Suite, stats: &mut MinReporterStats, prefix: String) {
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

fn get_stats_for_json(suite: &Suite, stats: &mut JsonReport, prefix: String) {
  for spec in &suite.context.specs {
    let mut spec_stat = JsonSpecReport {
      title: spec.name.to_string(),
      full_title: format!("{} {}", prefix, spec.name),
      duration: spec.duration,
      error: None
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

pub fn report_to_stdout(suite: &Suite) {

  match suite.reporter {
    Reporter::Spec => {
            
      let mut stats = MinReporterStats {
        passed: 0,
        pending: 0,
        failed: 0,
        error_lines: vec![]
      };
      print!("\n\n");
      get_lines_for_spec(suite, 0, &mut stats);
      
      println!("");
      if stats.failed == 0 {
        let duration = match suite.duration_type {
          DurationType::Nano => Duration::Nano(suite.total_duration),
          DurationType::Micro => Duration::Micro(suite.total_duration),
          DurationType::Mil => Duration::Mil(suite.total_duration),
          DurationType::Sec => Duration::Sec(suite.total_duration)
        };
        println!("{}{} {}",
          style("✓").green().to_string(), 
          style(format!(" {} test{} completed", stats.passed, get_suffix(stats.passed))).green().to_string(),
          style(format!("{}", duration.to_string())).dim().to_string()
        );
      } else {
        println!(" {}{}",
          style(format!("✖ {} of {} test{} failed", stats.failed, stats.passed + stats.failed, get_suffix(stats.failed))).red().to_string(),
          style(":").dim().to_string()
        );
      }
      println!("");
        
      for spec_report_line in &stats.error_lines {
        println!("{}", spec_report_line);
      }
      println!("");
      println!("");
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

      print!("\n\n");
      if stats.passed > 0 {
        println!("{} {}",
          style(format!("{} test{} complete", stats.passed, get_suffix(stats.passed))).green().to_string(),
          style(format!("{}", duration.to_string())).dim().to_string()
        );
      }
      if stats.pending > 0 {
        println!("{}", style(format!("{} test{} pending", stats.pending, get_suffix(stats.pending))).dim().to_string());
      }
      if stats.failed > 0 {
        println!("{}",
          style(format!("{} test{} failed", stats.failed, get_suffix(stats.failed))).red().to_string()
        );
        print!("\n\n");
        for line in &stats.error_lines {
          println!("{}", line);
          println!("");
        }
      }
      print!("\n\n");
    },
    Reporter::Dot => {
      let mut stats = DotReporterStats {
        passed: 0,
        failed: 0,
        pending: 0,
        dots: vec![]
      };
      get_dots(suite, &mut stats);
      print!("\n\n");
      for line in &stats.dots {
        print!("{}", line);
      }
      print!("\n\n");
      println!("{}", style(format!("{} passing", stats.passed)).green().to_string());
      println!("{}", style(format!("{} pending", stats.pending)).cyan().to_string());
      println!("{}", style(format!("{} failed", stats.failed)).red().to_string());
      print!("\n\n");
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
      print!("\n\n");
      get_list(suite, &mut stats, suite.name.to_string());
      print!("\n\n");
      println!("{}", style(format!("{} passing {}", stats.passed, duration.to_string())).green().to_string());
      println!("{}", style(format!("{} pending", stats.pending)).cyan().to_string());
      println!("{}", style(format!("{} failed", stats.failed)).red().to_string());
      print!("\n\n");
    },
    Reporter::Tap => {
      let mut lines = vec![];
      let mut count = 0;
      get_tap_list(suite, &mut lines, &mut count, suite.name.to_string());
      print!("\n\n");
      println!("{}", style(format!("1..{}", count)).green().to_string());
      for line in &lines {
        println!("{}", line);
      }
      print!("\n\n");
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
      print!("\n\n");
      println!("### Lab Results start ###");
      print!("\n");
      println!("Running {} test{}", count, get_suffix(count));
      print!("\n\n");
      get_list_for_rust(suite, &mut stats, suite.name.to_case(Case::Snake));
      print!("\n");
      let passed = green(&format!("{} passed", stats.passed));
      let ignored = cyan(&format!("{} ignored", stats.pending));      
      if stats.failed == 0 {
        println!("test result: ok. {}; 0 failed; {}; 0 measured; 0 filtered out", passed, ignored);
      } else {
        let failed = red(&format!("{} failed", stats.failed));
        println!("{}", red("failures:"));
        for line in &stats.error_lines {
          println!("    {}", red(line));            
        }
        print!("\n");
        println!("test result: {}. {}; {}; {}; 0 measured; 0 filtered out", red("FAILED"), passed, failed, ignored);
      }
      print!("\n");
      println!("### Lab Results end ###");
      print!("\n\n");
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
      print!("\n\n");
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
      print!("\n\n");
    }
  }    
}
