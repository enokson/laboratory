use std::{
  cell::RefCell,
  fmt::Display,
  rc::Rc
};
use crate::suite::Speed;
use crate::suite_context::State;

pub struct SpecOptions<T> {
  pub name: Option<String>,
  pub retries_: Option<u32>,
  pub slow_: Option<u128>,
  pub hook: Option<Box<dyn Fn(&mut SpecContext<T>) -> Result<(), String> + 'static>>,
}
impl<T> SpecOptions<T> {
  pub fn new() -> SpecOptions<T> {
    SpecOptions {
      name: None,
      retries_: None,
      slow_: None,
      hook: None
    }
  }
  pub fn it<N, H>(&mut self, name: N, cb: H) -> &mut Self
  where
  N: Into<String> + Display,
  H: Fn(&mut SpecContext<T>) -> Result<(), String> + 'static
  {
    self.name = Some(name.to_string());
    self.hook = Some(Box::new(cb));
    self
  }
  pub fn retries(&mut self, retries: u32) -> &mut Self {
    self.retries_ = Some(retries);
    self
  }
  pub fn slow(&mut self, threshold: u128) -> &mut Self {
    self.slow_ = Some(threshold);
    self
  }
}

pub struct SpecContext<T> {
  pub state: Rc<RefCell<State<T>>>,
  pub retries_: Option<u32>,
  pub slow_: Option<u128>,
  pub speed_result: Speed,
  pub attempts: u32,
}
impl<T> SpecContext<T> {
  pub fn new(state: Rc<RefCell<State<T>>>) -> SpecContext<T> {
    SpecContext {
      state,
      retries_: None,
      slow_: None,
      speed_result: Speed::Fast,
      attempts: 0
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

pub struct Spec<T> {
  pub name: String,
  pub order: Option<u32>,
  pub only: bool,
  pub hook: Box<dyn Fn(&mut SpecContext<T>) -> Result<(), String> + 'static>,
  pub result: Option<Result<(), String>>,
  pub duration: u128,
  pub context:  SpecContext<T>,
  pub skip: bool
}
impl<T> Spec<T> {
  pub fn new(name: String, state: Rc<RefCell<State<T>>>, hook: Box<dyn Fn(&mut SpecContext<T>) -> Result<(), String>>) -> Spec<T> {
    let context = SpecContext::new(state);
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
