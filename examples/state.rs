
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

    use laboratory::{describe, expect, LabResult, Suite};
    use super::{always_return_true, add_one, add_two};
    use std::cell::{RefCell, RefMut};
    use std::fmt::{Debug};
    use std::rc::Rc;

    #[test]
    fn test() -> LabResult {

        enum State {
            I32(i32),
            String(String)
        }

        fn imported_suite() -> Suite<State> {
            describe("imported suite", |suite| {
                suite.it("should do something cool", |spec| {
                    // let state = spec.state.borrow();
                    // println!("/counter from imported suite: {}", state.get("/counter").unwrap());
                    expect(true).to_be(true)
                });
            })
        }

        describe("My Crate", |suite| {

            suite.before_all(|state| {

                state.insert("/counter", State::I32(0));
               
            }).before_each(|state| {

                if let State::I32(count) = state.get_mut("/counter").unwrap() {
                    *count += 1;
                };

            }).after_each(|state| {

                if let State::I32(count) = state.get_mut("/counter").unwrap() {
                    *count += 1;
                };

            }).after_all(|state| {

                if let State::I32(count) = state.get_mut("/counter").unwrap() {
                    println!("/counter: {:?}", count);
                };

            }).describe("add_one()", |suite| {

                suite.it("should return 1", |spec| {

                    expect(add_one(0)).to_be(1)

                }).it("should return 2", |_| {
                
                    expect(add_one(1)).to_be(2)

                });

            }).describe("add_two()", |suite| {

                suite.before_all(|state| {
                    
                    state.insert("/add_two()", State::String("hello".to_string()));

                }).before_each(|state| {

                }).after_each(|state| {

                }).after_all(|state| {

                    if let State::String(string) = state.get_mut("/add_two()").unwrap() {
                        string.push_str(", world!")
                    }

                    if let State::String(string) = state.get_mut("/add_two()").unwrap() {
                        println!("/add_two(): {:?}", string);
                    }
                    
                }).it("should return 2", |spec| {

                    expect(add_two(0)).to_be(2)

                }).it("should return 4", |_| {

                    expect(add_two(2)).to_be(4)

                });

            }).describe("always_return_true()", |suite| {


                suite.it("should always return true", |_| {

                    expect(always_return_true()).to_be(true)

                });

            })
            .describe_import(imported_suite()).it("should do something", |spec| {
                expect(true).to_be(true)
            });

        }).rust().run()

    }

}
