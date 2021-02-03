use chrono::offset::Utc;
use chrono::DateTime;
use console::{style, Style};
use serde::{Serialize};
use serde_json::{to_string, to_string_pretty};
use std::{fmt::Display, rc::Rc};
use std::time::{Instant, SystemTime};
use convert_case::{Case, Casing};

use crate::LabResult;

/* 

FIX: JSON reporter does not output endtime
TODO: implement random spec iteration order
TODO: implement async support

*/

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

pub enum Reporter{
  Spec,
  Min,
  Dot,
  List,
  Rust,
  Tap,
  Json(bool) // true = pretty
}

#[derive(Debug, Clone, Copy)]
enum Speed {
  Fast,
  OnTime,
  Slow
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

#[derive(Debug, Clone, Copy)]
enum Duration {
  Nano(u128),
  Micro(u128),
  Mil(u128),
  Sec(u128)
}
impl Duration {
  pub fn to_string(&self) -> String {
    match self {
      Duration::Nano(duration) => format!("({}ns)", duration),
      Duration::Micro(duration) => format!("({}μs)", duration),
      Duration::Mil(duration) => format!("({}ms)", duration),
      Duration::Sec(duration) => format!("({}sec)", duration)
    }
  }
}


#[derive(Debug, Clone, Copy)]
pub enum DurationType {
  Nano,
  Micro,
  Mil,
  Sec
}

type Depth = u32;
type Index = u32;
type ErrorMessage = String;
type SpecName = String;
type PassCount = u32;
type FailCount = u32;

enum SpecReportLine {
  Suite(Depth, String),
  SpecPass(Depth, String, SpeedDisplay),
  SpecFail(Depth, Index, String),
  SpecIgnored(Depth, String),
  SummaryPass(u32, SpeedDisplay),
  SummaryFail(u32, u32),
  Fail(Index, String, String)
}

pub struct SpecContext {
  retries_: Option<u32>,
  slow_: Option<u128>,
  speed_result: Speed
}
impl SpecContext {
  pub fn new() -> SpecContext {
    SpecContext {
      retries_: None,
      slow_: None,
      speed_result: Speed::Fast
    }
  }
  pub fn retries(&mut self, count: u32) -> &mut Self {
    self.retries_ = Some(count);
    self
  }
  pub fn slow(&mut self, count: u128) -> &mut Self {
    self.slow_ = Some(count);
    self
  }
  pub fn get_retries(&self) -> Option<&u32> {
    self.retries_.as_ref()
  }
  pub fn get_slow(&self) -> Option<&u128> {
    self.slow_.as_ref()
  }

}

struct Spec {
  pub name: String,
  pub order: Option<u32>,
  pub only: bool,
  pub hook: Box<dyn Fn(&mut SpecContext) -> Result<(), String> + 'static>,
  pub result: Option<Result<(), String>>,
  pub duration: u128,
  pub context:  SpecContext,
  pub skip: bool
}
impl Spec {
  pub fn new(name: String, hook: Box<dyn Fn(&mut SpecContext) -> Result<(), String>>) -> Spec {
    let context = SpecContext::new();
    Spec {
      name,
      order: None,
      only: false,
      hook,
      result: None,
      duration: 0,
      context,
      skip: false,
    }
  }
}

pub struct SuiteContext {
  after_all_hook: Option<Rc<dyn Fn() + 'static>>,
  after_each_hook: Option<Rc<dyn Fn() + 'static>>,
  before_all_hook: Option<Rc<dyn Fn() + 'static>>,
  before_each_hook: Option<Rc<dyn Fn() + 'static>>,
  specs: Vec<Spec>,
  suites: Vec<Suite>,
  retries_: Option<u32>,
  skip_: bool,
  slow_: Option<u128>,
  passed: u32,
  failed: u32,
  ignored: u32
}
impl SuiteContext {
  pub fn new() -> SuiteContext {
    SuiteContext {
      after_all_hook: None,
      after_each_hook: None,
      before_all_hook: None,
      before_each_hook: None,
      specs: vec![],
      suites: vec![],
      retries_: None,
      skip_: false,
      slow_: None,
      passed: 0,
      failed: 0,
      ignored: 0
    }
  }
  pub fn before_all<H: Fn() + 'static>(&mut self, hook: H) -> &mut Self {
    self.before_all_hook = Some(Rc::new(hook));
    self
  }
  pub fn before_each<H: Fn() + 'static>(&mut self, hook: H) -> &mut Self {
    self.before_each_hook = Some(Rc::new(hook));
    self
  }
  pub fn after_all<H: Fn() + 'static>(&mut self, hook: H) -> &mut Self {
    self.after_all_hook = Some(Rc::new(hook));
    self
  }
  pub fn after_each<H: Fn() + 'static>(&mut self, hook: H) -> &mut Self {
    self.after_each_hook = Some(Rc::new(hook));
    self
  }
  pub fn it<S, H>(&mut self, name: S, hook: H) -> &mut Self
    where 
      S: Into<String> + Display,
      H: Fn(&mut SpecContext) -> Result<(), String> + 'static
  {
    self.specs.push(Spec::new(name.to_string(), Box::new(hook)));
    self
  }
  pub fn it_skip<S, H>(&mut self, name: S, hook: H) -> &mut Self
  where 
    S: Into<String> + Display,
    H: Fn(&mut SpecContext) -> Result<(), String> + 'static
 {
    let mut spec = Spec::new(name.to_string(), Box::new(hook));
    spec.skip = true;
    self.specs.push(spec);
    self
  }
  pub fn it_only<S, H>(&mut self, name: S, hook: H) -> &mut Self
  where 
    S: Into<String> + Display,
    H: Fn(&mut SpecContext) -> Result<(), String> + 'static
 {
    let mut spec = Spec::new(name.to_string(), Box::new(hook));
    spec.only = true;
    self.specs.push(spec);
    self
  }
  pub fn describe<S, H>(&mut self, name: S, cb: H) -> &mut Self
  where 
    S: Into<String> + Display,
    H: Fn(&mut SuiteContext) + 'static
 {
    let suite = describe(name, cb);
    self.suites.push(suite);
    self
  }
  pub fn describe_skip<S, H>(&mut self, name: S, cb: H) -> &mut Self
  where 
    S: Into<String> + Display,
    H: Fn(&mut SuiteContext) + 'static
 {
    let mut suite = describe(name.to_string(), cb);
    suite.context.skip_ = true;
    self.suites.push(suite);
    self
  }
  pub fn describe_only<S, H>(&mut self, name: S, cb: H) -> &mut Self
  where 
    S: Into<String> + Display,
    H: Fn(&mut SuiteContext) + 'static
 {
    let mut suite = describe(name.to_string(), cb);
    suite.only = true;
    self.suites.push(suite);
    self
  }
  pub fn describe_import(&mut self, suite: Suite) -> &mut Self {
    self.suites.push(suite);
    self
  }
  pub fn describe_import_skip(&mut self, mut suite: Suite) -> &mut Self {
    suite.context.skip_ = true;
    self.suites.push(suite);
    self
  }
  pub fn describe_import_only(&mut self, mut suite: Suite) -> &mut Self {
    suite.only = true;
    self.suites.push(suite);
    self
  }
  pub fn skip(&mut self) -> &mut Self {
    self.skip_ = true;
    self
  }
  pub fn retries(&mut self, count: u32) -> &mut Self {
    self.retries_ = Some(count);
    self
  }
  pub fn slow(&mut self, count: u128) -> &mut Self {
    self.slow_ = Some(count);
    self
  }
}

pub struct Suite {
  pub name: String,
  pub only: bool,
  pub context: SuiteContext,
  pub duration_type: DurationType,
  pub suite_duration: u128,
  pub total_duration: u128,
  pub depth: u32,
  pub reporter: Reporter,
  pub start_time: String,
  pub end_time: String
}
impl Suite {
  pub fn run(&mut self) -> LabResult {
    Suite::apply_depth_to_suites(self);
    Suite::index_specs(self, &mut 0);
    Suite::ignore_non_onlys(self);
    Suite::apply_hooks(self);
    Suite::apply_duration_type(self);
    Suite::run_specs_and_suites(self);
    Suite::sum_result_counts(self);
    Suite::sum_test_durations(self);
    Suite::apply_slow_settings(self);
    Suite::calculate_speed(self);
    Suite::report_to_stdout(&self);
    if self.context.failed == 0 {
      Ok(())
    } else {
      Err(format!("Expected {} to equal 0", self.context.failed))
    }
  }
  pub fn spec(mut self) -> Self {
    self.reporter = Reporter::Spec;
    self
  }
  pub fn min(mut self) -> Self {
    self.reporter = Reporter::Min;
    self
  }
  pub fn dot(mut self) -> Self {
    self.reporter = Reporter::Dot;
    self
  }
  pub fn list(mut self) -> Self {
    self.reporter = Reporter::List;
    self
  }
  pub fn tap(mut self) -> Self {
    self.reporter = Reporter::Tap;
    self
  }
  pub fn rust(mut self) -> Self {
    self.reporter = Reporter::Rust;
    self    
  }
  pub fn json(mut self) -> Self {
    self.reporter = Reporter::Json(false);
    self
  }
  pub fn json_pretty(mut self) -> Self {
    self.reporter = Reporter::Json(true);
    self
  }
  pub fn nano(mut self) -> Self {
    self.duration_type = DurationType::Nano;
    self
  }
  pub fn micro(mut self) -> Self {
    self.duration_type = DurationType::Micro;
    self
  }
  pub fn milis(mut self) -> Self {
    self.duration_type = DurationType::Mil;
    self
  }
  pub fn sec(mut self) -> Self {
    self.duration_type = DurationType::Sec;
    self
  }
  fn run_specs_and_suites(suite: &mut Suite) {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    suite.start_time = datetime.to_string();
    if let Some(boxed_hook) = &suite.context.before_all_hook {
      let hook = boxed_hook.as_ref();
      (hook)()
    }
    for spec in &mut suite.context.specs {
      if !spec.skip {
        let retries: u32 = {
          if let Some(suite_retries) = suite.context.retries_ {
            if let Some(spec_retries) = spec.context.retries_ {
              spec_retries
            } else {
              suite_retries
            }
          } else {
            if let Some(spec_retries) = spec.context.retries_ {
              spec_retries
            } else {
              0
            }
          }
        };
        for _i in 0..=(retries + 1) {
          if let Some(boxed_hook) = &suite.context.before_each_hook {
            let hook = boxed_hook.as_ref();
            (hook)()
          }
          let start_time = Instant::now();
          let result = (spec.hook.as_ref())(&mut spec.context);
          let duration = start_time.elapsed();
          let duration_int = match suite.duration_type {
            DurationType::Nano => duration.as_nanos(),
            DurationType::Micro => duration.as_micros(),
            DurationType::Mil => duration.as_millis(),
            DurationType::Sec => duration.as_secs() as u128
          };
          spec.result = Some(result);
          spec.duration = duration_int;
          if let Some(boxed_hook) = &suite.context.after_each_hook {
            let hook = boxed_hook.as_ref();
            (hook)()
          }
          if spec.result.as_ref().unwrap().is_ok() {
            break;
          }
        }
      }
      let system_time = SystemTime::now();
      let datetime: DateTime<Utc> = system_time.into();
      suite.end_time = datetime.to_string();
    
    }
    for child_suite in suite.context.suites.iter_mut() {
      if !child_suite.context.skip_ {
        Suite::run_specs_and_suites(child_suite);
      }
    }
    if let Some(boxed_hook) = &suite.context.after_all_hook {
      let hook = boxed_hook.as_ref();
      (hook)()
    }    
  }
  fn index_specs(suite: &mut Suite, count: &mut u32) {
    for spec in suite.context.specs.iter_mut() {
      spec.order = Some(*count);
      *count += 1;
    }
    for suite in suite.context.suites.iter_mut() {
      Suite::index_specs(suite, count);
    }
  }
  fn apply_depth_to_suites(suite: &mut Suite) {
    for child_suite in &mut suite.context.suites {
      child_suite.depth += suite.depth;
    }
  }
  fn report_to_stdout(suite: &Suite) {



    fn get_suffix(n: u32) -> String {
      if n > 1 {
        "s".to_string()
      } else {
        "".to_string()
      }
    }

    match suite.reporter {
      Reporter::Spec => {
        
        fn get_lines(suite: &Suite, depth: u32, stats: &mut MinReporterStats) {

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
            get_lines(child_suite, depth + 1, stats);

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
        
        let mut stats = MinReporterStats {
          passed: 0,
          pending: 0,
          failed: 0,
          error_lines: vec![]
        };
        print!("\n\n");
        get_lines(suite, 0, &mut stats);
        
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

        fn space_per_byte(n: u32) -> String {
          let len = n.to_string().len();
          let mut return_str = String::new();
          for _i in 0..len {
            return_str.push(' ');
          }
          return_str
        }

        fn line_spacing(depth: u32) -> String {
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
                    line_spacing(depth), 
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
            get_lines_for_min(child_suite, stats, format!("{}\n   {}{}", prefix, line_spacing(depth), child_suite.name), depth + 1);
          }
        }

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

        fn get_count(suite: &Suite) -> u32 {
          let mut count = suite.context.specs.len() as u32;
          for child_suite in &suite.context.suites {
            count += get_count(child_suite);
          }
          return count;
        }

        fn get_list(suite: &Suite, stats: &mut MinReporterStats, prefix: String) {
          for spec in &suite.context.specs {
            if let Some(result) = &spec.result {
              if let Err(_msg) = result {
                println!("test {}::{} ... FAILED", prefix, spec.name.to_case(Case::Snake));
                stats.error_lines.push(format!("{}::{}", prefix, spec.name.to_case(Case::Snake)));
                stats.failed += 1;
              } else {
                println!("test {}::{} ... ok", prefix, spec.name.to_case(Case::Snake));
                stats.passed += 1;
              }
            } else {
              println!("test {}::{} ... ignored", prefix, spec.name.to_case(Case::Snake));
              stats.pending += 1;
            }
          }
          for child_suite in &suite.context.suites {
            get_list(child_suite, stats, format!("{}::{}", prefix, child_suite.name.to_case(Case::Snake)));
          }
        }
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
        get_list(suite, &mut stats, suite.name.to_case(Case::Snake));
        print!("\n");
        if stats.failed == 0 {
          println!("test result: ok. {} passed; 0 failed; {} ignored; 0 measured; 0 filtered out", stats.passed, stats.pending);
        } else {
          println!("failures:");
          for line in &stats.error_lines {
            println!("    {}", line);            
          }
          print!("\n");
          println!("test result: FAILED. {} passed; {} failed; {} ignored; 0 measured; 0 filtered out", stats.passed, stats.failed, stats.pending);
        }
        print!("\n");
        println!("### Lab Results end ###");
        print!("\n\n");
      },
      Reporter::Json(pretty) => {
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
  fn apply_hooks(suite: &mut Suite) {
    for child_suite in suite.context.suites.iter_mut() {
      if let Some(hook) = &suite.context.before_each_hook {
        if let None = child_suite.context.before_each_hook {
          child_suite.context.before_each_hook = Some(hook.clone());
        }
      }
      if let Some(hook) = &suite.context.after_each_hook {
        if let None = child_suite.context.before_each_hook {
          child_suite.context.before_each_hook = Some(hook.clone());
        }
      }
    }
  }
  fn ignore_non_onlys(suite: &mut Suite) {
    if suite.context.skip_ == true {
      for spec in &mut suite.context.specs {
        spec.skip = true;
      }
      for child_suite in &mut suite.context.suites {
        child_suite.context.skip_ = true;
      }
    } else {
      let mut has_at_least_one_only = false;
      for spec in &suite.context.specs {
        if spec.only {
          has_at_least_one_only = true;
          break;
        }
      }
      if has_at_least_one_only {
        for spec in &mut suite.context.specs {
          if !spec.only {
            spec.skip = true;
          }
        }
      }
      has_at_least_one_only = false;
      for child_suite in &suite.context.suites {
        if child_suite.only {
          has_at_least_one_only = true;
          break;
        }
      }
      if has_at_least_one_only {
        for child_suite in &mut suite.context.suites {
          if !child_suite.only {
            child_suite.context.skip_ = true;
          }
        }
      }
    }    
    for child_suite in &mut suite.context.suites {
      Suite::ignore_non_onlys(child_suite);
    }
  }
  fn apply_retries(suite: &mut Suite) {
    if let Some(retries) = suite.context.retries_ {
      for spec in &mut suite.context.specs {
        if let None = spec.context.retries_ {
          spec.context.retries_ = Some(retries);
        }
      }
      for child_suite in &mut suite.context.suites {
        if let None = child_suite.context.retries_ {
          child_suite.context.retries_ = Some(retries);
        }
      }
      for child_suite in &mut suite.context.suites {
        Suite::apply_retries(child_suite);
      }
    }
  }
  fn apply_duration_type(suite: &mut Suite) {
    for child_suite in &mut suite.context.suites {
      child_suite.duration_type = suite.duration_type;
    }
  }
  fn apply_slow_settings(suite: &mut Suite) {
    if let Some(slow_setting) = suite.context.slow_ {
      for spec in &mut suite.context.specs {
        if let None = spec.context.slow_ {
          spec.context.slow_ = Some(slow_setting);
        }
      }
      for child_suite in &mut suite.context.suites {
        if let None = child_suite.context.slow_ {
          child_suite.context.slow_ = Some(slow_setting);
        }
      }
    }
    for child_suite in &mut suite.context.suites {
      Suite::apply_slow_settings(child_suite);
    }
  }
  fn sum_result_counts(suite: &mut Suite) {
    for spec in &suite.context.specs {
      if let Some(result) = &spec.result {
        match result {
          Ok(_) => {
            suite.context.passed += 1;
          },
          Err(_) => {
            suite.context.failed += 1;
          }
        }
      } else {
        suite.context.ignored += 1;
      }
    }
    for child_suite in &mut suite.context.suites {
      Suite::sum_result_counts(child_suite);
    }
  }
  fn sum_test_durations(suite: &mut Suite) {
    for spec in &suite.context.specs {
      suite.suite_duration += spec.duration;
      suite.total_duration += spec.duration;
    }
    for child_suite in &mut suite.context.suites {
      Suite::sum_test_durations(child_suite);
      suite.total_duration += child_suite.total_duration;
    }
  }
  fn calculate_speed(suite: &mut Suite) {
    for spec in &mut suite.context.specs {
      if let Some(slow_time) = spec.context.slow_{
        let fast_time = ((slow_time as f64) / 2.0) as u128;
        if spec.duration > slow_time {
          spec.context.speed_result = Speed::Slow;
        } else if spec.duration <= fast_time {
          spec.context.speed_result = Speed::Fast;
        } else {
          spec.context.speed_result = Speed::OnTime;
        }
      } else {
        spec.context.speed_result = Speed::Fast;
      }      
    }
    for child_suite in &mut suite.context.suites {
      Suite::calculate_speed(child_suite);
    }
  }
}

pub fn describe<S, T>(name: S, cb: T) -> Suite
  where
    S: Into<String> + Display,
    T: Fn(&mut SuiteContext) + 'static
{
  let mut context = SuiteContext::new();
  (cb)(&mut context);
  Suite {
    name: name.to_string(),
    only: false,
    context,
    duration_type: DurationType::Nano,
    depth: 0,
    suite_duration: 0,
    total_duration: 0,
    reporter: Reporter::Spec,
    start_time: String::new(),
    end_time: String::new()
  }
}

#[cfg(test)]
mod tests {
  use std::rc::Rc;
  use std::cell::RefCell;
  use super::*;
  use super::super::*;

  #[test]
  fn describe_a_suite() -> LabResult {
    
    describe("my suite", |ctx| {

      let state = Rc::new(RefCell::new(0));

      let state_2 = state.clone();
      ctx.before_all(move || {
        let mut state_ref = state_2.as_ref().borrow_mut();
        *state_ref += 1;
      });

      let state_3 = state.clone();
      ctx.before_each(move || {
        let mut state_ref = state_3.as_ref().borrow_mut();
        *state_ref += 1;
      });

      ctx.after_all(move || {
        // println!("count: {}", state.as_ref().borrow());
      });

      // ctx.retries(5);

      ctx.it("should do timely stuff", |_ctx| {
        should_panic(|| {
          panic!("help!");
        })
      });

      ctx.it_skip("should do stuff", |ctx| {

        ctx.retries(0);
        ctx.slow(5000);

        println!("test 2");
        Ok(())
      });

      ctx.it("should do stuff", |ctx| {

        ctx.retries(10);

        println!("test 3");
        Ok(())
      });

      ctx.it_skip("should do stuff", |_ctx| {

        println!("test 4");
        Err("failed".to_string())

      });

      ctx.describe("sub module 1", |ctx| {

        ctx.before_all(|| {
          println!("sub module 1 is running.");
        });

        ctx.it("should do stuff", move |_ctx| {
          // Err("I am a failure".to_string())
          Ok(())
        });
        
      });

      ctx.describe("sub module 2", |ctx| {

        ctx.before_all(|| {
          println!("sub module 2 is running.");
        });

        ctx.it("should do stuff", move |_ctx| {
          println!("sub module 2 test 1 running.");
          Ok(())
        });

        ctx.it("should do stuff", move |_ctx| {
          println!("sub module 2 test 2 running.");
          Ok(())
        });
        
      });

    }).min().nano().run()

  }

}