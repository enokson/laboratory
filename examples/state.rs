
fn always_return_true() -> bool { true  }
fn add_one(n: i32) -> i32 { n + 1 }
fn add_two(n: i32) -> i32 { n + 2 }

fn main() {
    let _true = always_return_true();
    let _one = add_one(0);
    let _two = add_two(0);
}

#[cfg(test)]
mod tests {

    // use super::*;
    // use laboratory::{describe, expect, LabResult};
    // use std::cell::{RefCell, RefMut};
    // use std::fmt::{Debug};
    // use std::rc::Rc;

    // // We want a counter to count each time a hook or test is called

    // #[derive(Debug)]
    // struct Counter {
    //     suite: String, // the name of the suite
    //     call_count: u8 // the number of times a hook or test was called
    // }

    // impl Counter {
    //     fn new(suite: &str) -> Counter {
    //         Counter {
    //             suite: String::from(suite),
    //             call_count: 0
    //         }
    //     }
    //     fn update(&mut self) {
    //         self.call_count += 1;
    //         println!("  {} hit count: {}", self.suite, self.call_count);
    //     }
    // }

    // #[test]
    // fn test() -> LabResult {

    //     // Here we will define a function to handle all the hook calls
    //     let hook_handle = |counter: RefMut<Counter>| {
    //         counter.update();
    //     };

    //     describe("My Crate", |ctx| {

    //         let parent_counter = Rc::new(RefCell::new(Counter::new("Parent Counter")));

    //         ctx.before_all(move || {

    //             let counter = Rc::clone(&parent_counter);
    //             hook_handle(counter.borrow_mut())

    //         }).before_each(move || {

    //             let counter = Rc::clone(&parent_counter);
    //             hook_handle(counter.borrow_mut())

    //         }).after_each(move || {

    //             let counter = Rc::clone(&parent_counter);
    //             hook_handle(counter.borrow_mut())

    //         }).after_all(move || {

    //             let counter = Rc::clone(&parent_counter);
    //             hook_handle(counter.borrow_mut());
    //             println!("{:#?}\n\n", Rc::clone(&parent_counter).borrow());

    //         }).describe("add_one()", |ctx| {

    //             ctx.it("should return 1", |_| {

    //                 let counter = Rc::clone(&parent_counter);
    //                 hook_handle(counter.borrow_mut());
    //                 expect(add_one(0)).to_be(1)

    //             }).it("should return 2", |_| {

    //                 let counter = Rc::clone(&parent_counter);
    //                 hook_handle(counter.borrow_mut());
    //                 expect(add_one(1)).to_be(2)

    //             });

    //         }).describe("add_two()", |ctx| {

    //             let child_rounter = Rc::new(RefCell::new(Counter::new("Child Counter")));

    //             ctx.before_all(move || {

    //                 let counter = Rc::clone(&child_rounter);
    //                 hook_handle(counter.borrow_mut())
    
    //             }).before_each(move || {
    
    //                 let counter = Rc::clone(&child_rounter);
    //                 hook_handle(counter.borrow_mut())
    
    //             }).after_each(move || {
    
    //                 let counter = Rc::clone(&child_rounter);
    //                 hook_handle(counter.borrow_mut())
    
    //             }).after_all(move || {
    
    //                 let counter = Rc::clone(&child_rounter);
    //                 hook_handle(counter.borrow_mut())
    
    //             }).it("should return 2", |_| {

    //                 let counter = Rc::clone(&child_rounter);
    //                 hook_handle(counter.borrow_mut());
    //                 expect(add_two(0)).to_be(2)

    //             }).it("should return 4", |_| {

    //                 let counter = Rc::clone(&child_rounter);
    //                 hook_handle(counter.borrow_mut());
    //                 expect(add_two(2)).to_be(4)

    //             });

    //         }).describe("always_return_true()", |ctx| {

    //             ctx.it("should always return true", |_| {

    //                 let counter = Rc::clone(&parent_counter);
    //                 hook_handle(counter.borrow_mut());
    //                 expect(add_one(0)).to_be(1)

    //             });

    //         });

    //     }).run()

    // }

}
