

mod expect;
mod spec;
mod suite;
mod reporter;

use suite::Suite;
use expect::Expect;
use spec::Spec;
use crate::reporter::ReporterType;
use std::fmt::{Debug, Display};

pub fn expect<T>(result: T) -> Expect<T>
    where T: PartialEq + Debug + Display
{
    Expect { result }
}

pub fn describe<S>(name: &'static str) -> Suite<S> {
    Suite::new(name.to_string())
}

pub fn describe_skip<S>(name: &'static str) -> Suite<S> {
    Suite::new(name.to_string()).skip()
}

pub fn it <H>(name: &'static str, handle: H) -> Spec
where
    H: Fn() -> Result<(), String> + 'static
{
    Spec::new(name.to_string(), handle)
}

pub fn it_skip<H>(name: &'static str, handle: H) -> Spec
    where
        H: Fn() -> Result<(), String> + 'static
{
    Spec::new(name.to_string(), handle).skip()
}

pub fn it_only<H>(name: &'static str, handle: H) -> Spec
    where
        H: Fn() -> Result<(), String> + 'static
{
    Spec::new(name.to_string(), handle).only()
}

#[cfg(test)]
mod test {
    use std::cell::{RefCell, RefMut};
    use std::rc::Rc;
    use super::{Spec, Suite, Expect, expect, describe, describe_skip, it, it_skip, it_only};
    use std::borrow::BorrowMut;
    use std::thread;
    use std::time::Duration;

    #[derive(PartialEq)]
    struct Foo {
        pub bar: String
    }

    impl Foo {
        pub fn new (bar: &str) -> Foo {
            Foo {
                bar: bar.to_string()
            }
        }
    }

    #[test]
    fn suite() {

        struct Person {
            name: String
        }
        impl Person {
            pub fn new(name: &str) -> Person {
                Person {
                    name: name.to_string()
                }
            }
            pub fn change_name(&mut self, name: &str) {
                self.name = name.to_string();
            }
        }

        fn add_one (x: u64) -> u64 { x + 1 };

        fn intensive_add_one (x: u64) -> u64 {
            thread::sleep(Duration::from_millis(10));
            x + 1
        }

        describe("Library")
            .state(0)
            .before_all(|state| {
                state
            })
            .before_each(|state| {
                state
            })
            .after_each(|mut state| {
                state += 1;
                state
            })
            .after_all(|mut state| {
                state = 0;
                state
            })

            .suites(vec![

                describe("add_one()")
                    .specs(vec![

                        it("should return 1", || {
                            let result = &add_one(0);
                            expect(result).equals(&1)?;
                            Ok(())
                        }),

                        it("should return 2", || {
                            let result = &add_one(1);
                            expect(result).equals(&2)?;
                            Ok(())
                        })

                    ]),


                describe("Person")
                    .specs(vec![

                        it("should return baxtiyor", || {
                            let baxtiyor = Person::new("baxtiyor");
                            expect(baxtiyor.name).to_be("baxtiyor".to_string())?;
                            Ok(())
                        }),

                        it("should return joshua after changing the person's name", || {
                            let mut joshua = Person::new("baxtyior");
                            joshua.change_name("joshua");
                            expect(joshua.name).to_be("joshua".to_string())?;
                            Ok(())
                        })

                    ]),

                describe("intensive_add_one()")
                    .specs(vec![

                        it("should return 5", || {
                            let result = intensive_add_one(4);
                            expect(result).equals(5)?;
                            Ok(())
                        })

                    ])


            ])
            .run();

    }

}
