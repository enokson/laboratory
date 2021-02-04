use crate::suite::Speed;

pub struct SpecContext {
  pub retries_: Option<u32>,
  pub slow_: Option<u128>,
  pub speed_result: Speed
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

pub struct Spec {
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
