use std::rc::Rc;
use std::cell::RefCell;

use laboratory::{ describe, expect, should_panic, LabResult };

#[test]
fn describe_a_suite() -> LabResult {
  
  describe("my suite", |ctx| {

    ctx.before_all(move |state| {
      state.insert("counter", 0);
    });

    ctx.before_each(move |state| {

    });

    ctx.after_all(move |state| {
      // println!("count: {}", state.as_ref().borrow());
    });

    ctx.describe("child", |suite| {
      suite.it("test", |spec| {
        expect(true).to_be(true)
      });
    });





  }).rust().nano().run()

}
