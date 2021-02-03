use laboratory::{describe, expect, LabResult};
use std::cell::RefCell;
use std::fmt::{Debug};
use std::rc::Rc;


fn always_return_true() -> bool { true }
fn add_one(n: i32) -> i32 { n + 1 }
fn add_two(n: i32) -> i32 { n + 2 }

fn main() {
    let _true = always_return_true();
    let _one = add_one(0);
    let _two = add_two(0);
}



// We want a counter to count each time a hook or test is called

#[derive(Debug, Clone)]
struct Counter {
    suite: String, // the name of the suite
    call_count: u8 // the number of times a hook or test was called
}

impl Counter {
    fn new(suite: &str) -> Counter {
        Counter {
            suite: String::from(suite),
            call_count: 0
        }
    }
    fn update(&mut self) {
        self.call_count += 1;
        println!("  {} hit count: {}", self.suite, self.call_count);
    }
}

#[test]
fn test() -> LabResult {

    // Here we will define a function to handle all the hook calls
    let hook_handle = |counter: Rc<RefCell<Counter>>| {
      let mut counter = counter.borrow_mut();
      counter.update();
    };

    describe("My Crate", move |ctx| {

        let parent_counter = Rc::new(RefCell::new(Counter::new("Parent Counter")));
        let before_all_counter = parent_counter.clone();
        let before_each_counter = parent_counter.clone();
        let after_all_counter = parent_counter.clone();
        let after_each_counter = parent_counter.clone();
        let add_one_counter = parent_counter.clone();
        ctx.before_all(move || {

            hook_handle(before_all_counter.clone());

        }).before_each(move || {

          hook_handle(before_each_counter.clone());

        }).after_each(move || {

          hook_handle(after_each_counter.clone());

        }).after_all(move || {

          let counter_rc = after_all_counter.clone();
          hook_handle(counter_rc.clone());
          println!("{:#?}\n\n", counter_rc.clone());

        }).describe("add_one()", move |ctx| {

            let add_one_counter = add_one_counter.clone();
            let counter_1 = add_one_counter.clone();
            let counter_2 = add_one_counter.clone();
            ctx.it("should return 1", move |_| {

              hook_handle(counter_1.clone());
              expect(add_one(0)).to_be(1)

            }).it("should return 2", move |_| {

              hook_handle(counter_2.clone());
              expect(add_one(1)).to_be(2)

            });

        }).describe("add_two()", move |ctx| {

            let child_rounter = Rc::new(RefCell::new(Counter::new("Child Counter")));
            let before_all_counter = child_rounter.clone();
            let before_each_counter = child_rounter.clone();
            let after_all_counter = child_rounter.clone();
            let after_each_counter = child_rounter.clone();
            ctx.before_all(move || {

                // let counter = Rc::clone(&child_rounter);
                // hook_handle(counter.borrow_mut())

            }).before_each(move || {

                // let counter = Rc::clone(&child_rounter);
                // hook_handle(counter.borrow_mut())

            }).after_each(move || {

                // let counter = Rc::clone(&child_rounter);
                // hook_handle(counter.borrow_mut())

            }).after_all(move || {

                // let counter = Rc::clone(&child_rounter);
                // hook_handle(counter.borrow_mut())

            }).it("should return 2", |_| {

                // let counter = Rc::clone(&child_rounter);
                // hook_handle(counter.borrow_mut());
                expect(add_two(0)).to_be(2)

            }).it("should return 4", |_| {

                // let counter = Rc::clone(&child_rounter);
                // hook_handle(counter.borrow_mut());
                expect(add_two(2)).to_be(4)

            });

        }).describe("always_return_true()", |ctx| {

            ctx.it("should always return true", |_| {

                // let counter = Rc::clone(&parent_counter);
                // hook_handle(counter.borrow_mut());
                expect(add_one(0)).to_be(1)

            });

        });

    }).run()

}