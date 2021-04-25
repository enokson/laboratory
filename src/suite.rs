use chrono::offset::Utc;
use chrono::DateTime;
use std::{fmt::Display};
use std::time::{Instant, SystemTime};

use crate::LabResult;
use crate::suite_context::SuiteContext;
use crate::reporter::{
  Reporter,
  report_to_stdout
};

#[derive(Debug, Clone, Copy)]
pub enum Speed {
  Fast,
  OnTime,
  Slow
}

#[derive(Debug, Clone, Copy)]
pub enum Duration {
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

pub struct NullState;

pub struct Suite<T> {
  pub name: String,
  pub only: bool,
  pub cb: Box<dyn Fn(&mut SuiteContext<T>)>,
  pub context: SuiteContext<T>,
  pub duration_type: DurationType,
  pub suite_duration: u128,
  pub total_duration: u128,
  pub depth: u32,
  pub reporter: Reporter,
  pub start_time: String,
  pub end_time: String
}
impl<T> Suite<T> {
  pub fn new<N, H>(name: N, cb: H) -> Suite<T> where
  N: Into<String> + Display,
  H: Fn(&mut SuiteContext<T>) + 'static
  {
    let context = SuiteContext::new();
    Suite {
      name: name.to_string(),
      only: false,
      cb: Box::new(cb),
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
  pub fn run(&mut self) -> LabResult {
    Suite::apply_depth_to_suites(self);
    Suite::index_specs(self, &mut 0);
    Suite::ignore_non_onlys(self);
    Suite::apply_hooks(self);
    Suite::apply_state(self);
    Suite::apply_duration_type(self);
    Suite::run_specs_and_suites(self);
    Suite::sum_result_counts(self);
    Suite::sum_test_durations(self);
    Suite::apply_slow_settings(self);
    Suite::calculate_speed(self);
    report_to_stdout(&self);
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
  pub fn state(self, state: T) -> Self {
    self.context.state.borrow_mut().insert("/", state);
    self
  }
  fn run_specs_and_suites(suite: &mut Suite<T>) {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    suite.start_time = datetime.to_string();
    (suite.cb)(&mut suite.context);
    if let Some(boxed_hook) = &suite.context.before_all_hook {
      let hook = boxed_hook.as_ref();
      (hook)(&mut suite.context.state.borrow_mut())
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
        let attempts = 1 + retries;
        for _i in 1..=attempts {
          if let Some(boxed_hook) = &suite.context.before_each_hook {
            let hook = boxed_hook.as_ref();
            (hook)(&mut suite.context.state.borrow_mut())
          }
          spec.context.attempts += 1;
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
            (hook)(&mut suite.context.state.borrow_mut())
          }
          if spec.result.as_ref().unwrap().is_ok() {
            break;
          }
        }
      }    
    }
    for child_suite in suite.context.suites.iter_mut() {
      if !child_suite.context.skip_ {
        Suite::run_specs_and_suites(child_suite);
      }
    }
    if let Some(boxed_hook) = &suite.context.after_all_hook {
      let hook = boxed_hook.as_ref();
      (hook)(&mut suite.context.state.borrow_mut())
    }
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    suite.end_time = datetime.to_string();
  }
  fn index_specs(suite: &mut Suite<T>, count: &mut u32) {
    for spec in suite.context.specs.iter_mut() {
      spec.order = Some(*count);
      *count += 1;
    }
    for suite in suite.context.suites.iter_mut() {
      Suite::index_specs(suite, count);
    }
  }
  fn apply_depth_to_suites(suite: &mut Suite<T>) {
    for child_suite in &mut suite.context.suites {
      child_suite.depth += suite.depth;
    }
  }
  fn apply_hooks(suite: &mut Suite<T>) {
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
  fn apply_state(suite: &mut Suite<T>) {
    for child_suite in suite.context.suites.iter_mut() {
      child_suite.context.state = suite.context.state.clone();
    }
    // for spec in suite.context.specs.iter_mut() {
    //   spec.context.state = suite.context.state.clone();
    // }
  }
  fn ignore_non_onlys(suite: &mut Suite<T>) {
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
  fn apply_retries(suite: &mut Suite<T>) {
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
  fn apply_duration_type(suite: &mut Suite<T>) {
    for child_suite in &mut suite.context.suites {
      child_suite.duration_type = suite.duration_type;
    }
  }
  fn apply_slow_settings(suite: &mut Suite<T>) {
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
  fn sum_result_counts(suite: &mut Suite<T>) {
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
  fn sum_test_durations(suite: &mut Suite<T>) {
    for spec in &suite.context.specs {
      suite.suite_duration += spec.duration;
      suite.total_duration += spec.duration;
    }
    for child_suite in &mut suite.context.suites {
      Suite::sum_test_durations(child_suite);
      suite.total_duration += child_suite.total_duration;
    }
  }
  fn calculate_speed(suite: &mut Suite<T>) {
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

pub fn describe<T, S, H>(name: S, cb: H) -> Suite<T>
  where
    S: Into<String> + Display,
    H: Fn(&mut SuiteContext<T>) + 'static
{
  let context = SuiteContext::new();
  // (cb)(&mut context);
  Suite {
    name: name.to_string(),
    only: false,
    cb: Box::new(cb),
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
