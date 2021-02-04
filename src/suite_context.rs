use std::{fmt::Display, rc::Rc};
use crate::spec::{Spec, SpecContext};
use crate::suite::{Suite, describe};

pub struct SuiteContext {
  pub after_all_hook: Option<Rc<dyn Fn() + 'static>>,
  pub after_each_hook: Option<Rc<dyn Fn() + 'static>>,
  pub before_all_hook: Option<Rc<dyn Fn() + 'static>>,
  pub before_each_hook: Option<Rc<dyn Fn() + 'static>>,
  pub specs: Vec<Spec>,
  pub suites: Vec<Suite>,
  pub retries_: Option<u32>,
  pub skip_: bool,
  pub slow_: Option<u128>,
  pub passed: u32,
  pub failed: u32,
  pub ignored: u32
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
