use std::rc::Rc;
use std::cell::RefCell;

use laboratory::{ describe, expect, should_panic, LabResult };

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
        Err("I am a failure".to_string())
        // Ok(())
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

  }).rust().nano().run()

}
