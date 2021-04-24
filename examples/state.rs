
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

    use laboratory::{describe, expect, LabResult};
    use super::{always_return_true, add_one, add_two};
    use std::cell::{RefCell, RefMut};
    use std::fmt::{Debug};
    use std::rc::Rc;


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

        // let imported_suite = describe("imported suite", |suite| {
        //     suite.it("should do something cool", |spec| {
        //         let state = spec.state.borrow();
        //         println!("/counter from imported suite: {}", state.get("/counter").unwrap());
        //         expect(true).to_be(true)
        //     });
        // });

        describe("My Crate", |suite| {

            // let mut state = suite.state.borrow_mut();
            // state.insert(0, 0);
            // state.insert(1, 0);
            // drop(state);

            suite.before_all(|state| {

                state.insert("/counter", 0);
               
            }).before_each(|state| {

                let mut count = state.get_mut("/counter").unwrap();
                *count += 1;
                
            }).after_each(|state| {

                let mut count = state.get_mut("/counter").unwrap();
                *count += 1;

            }).after_all(|state| {

                let count = state.get("/counter").unwrap();
                println!("/counter: {:?}", count);
              
            }).describe("add_one()", |suite| {

                suite.it("should return 1", |spec| {

                    let state = spec.state.borrow();
                    let counter = state.get("/counter").unwrap();
                    println!("/counter from spec: {:?}", counter);
                    expect(add_one(0)).to_be(1)

                }).it("should return 2", |_| {
                
                    expect(add_one(1)).to_be(2)

                });

            }).describe("add_two()", |suite| {

                suite.before_all(|state| {
                    
                    state.insert("/add_two()", 0);

                }).before_each(|state| {
                    
                    let mut count = state.get_mut("/add_two()").unwrap();
                    *count += 1;

                }).after_each(|state| {
                    
                    let mut count = state.get_mut("/add_two()").unwrap();
                    *count += 1;

                }).after_all(|state| {

                    let count = state.get("/add_two()").unwrap();
                    println!("/add_two(): {:?}", count);
                    
                }).it("should return 2", |spec| {

                    let state = spec.state.borrow();
                    println!("/add_two spec: {}", state.get("/add_two()").unwrap());

                    expect(add_two(0)).to_be(2)

                }).it("should return 4", |_| {

                    expect(add_two(2)).to_be(4)

                });

            }).describe("always_return_true()", |ctx| {


                ctx.it("should always return true", |_| {

                    expect(always_return_true()).to_be(true)

                });

            });
            // .describe_import(imported_suite);

        }).rust().run()

    }

}
