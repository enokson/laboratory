#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]

mod assertion;
mod spec;
mod suite;
mod reporter;
mod state;
mod suite_result;
mod spec_result;

pub use suite::{Suite};
pub use state::State;
pub use assertion::Expect;
pub use spec::Spec;
use std::fmt::{Debug};
pub use serde::{Deserialize, Serialize};


#[macro_export]
macro_rules! should_panic {
    ($name:expr, $handle: expr) => {

        {
            use std::panic::{ catch_unwind, set_hook, take_hook };

            set_hook(Box::new(|_| {
                // println!("");
            }));
            let tmp_result = catch_unwind(|| {
                ($handle)();
            }).is_ok();
            let _ = take_hook();
            if tmp_result == false {
                Ok(())
            } else {
                Err(format!("Expected {} to panic but it didn't", stringify!($name)))
            }

        }

    };
}

#[macro_export]
macro_rules! should_not_panic {
    ($name:expr, $handle: expr) => {

        {
            use std::panic::{ catch_unwind, set_hook, take_hook };

            set_hook(Box::new(|_| {
                // println!("");
            }));
            let tmp_result = catch_unwind(|| {
                ($handle)();
            }).is_ok();
            let _ = take_hook();
            if tmp_result == true {
                Ok(())
            } else {
                Err(format!("Expected {} to panic but it didn't", stringify!($name)))
            }

        }

    };
}

pub fn expect<T>(result: T) -> Expect<T>
    where T: PartialEq + Debug
{
    Expect::new(result)
}

pub fn describe(name: &'static str) -> Suite {
    Suite::new(name.to_string())
}

pub fn describe_skip(name: &'static str) -> Suite {
    Suite::new(name.to_string()).skip()
}

pub fn it <H>(name: &'static str, handle: H) -> Spec
where
    H: FnMut(&mut State) -> Result<(), String> + 'static
{
    Spec::new(name.to_string(), handle)
}

pub fn it_skip<H>(name: &'static str, handle: H) -> Spec
    where
        H: FnMut(&mut State) -> Result<(), String> + 'static
{
    Spec::new(name.to_string(), handle).skip()
}

pub fn it_only<H>(name: &'static str, handle: H) -> Spec
    where
        H: FnMut(&mut State) -> Result<(), String> + 'static
{

    Spec::new(name.to_string(), handle).only()
}

#[cfg(test)]
mod tests {

    use std::fs::remove_file;
    use std::path::Path;
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct Foo(i32);

    impl Foo {
        pub fn new() -> Foo { Foo(0) }
    }

    fn add_one(n: u64) -> u64 { n + 1 }
    fn add_one_false(n: u64) -> u64 { n + 2 }
    fn simple_spec_pass() -> Spec {
        it("should return 1", |_| {
            expect(add_one(0)).to_equal(1)?;
            expect(add_one(0)).equals(1)?;
            expect(add_one(0)).to_be(1)?;
            expect(add_one(0)).to_not_equal(2)?;
            expect(add_one(0)).to_not_be(2)?;
            match expect(add_one(0)).to_not_equal(1) {
                Ok(_) => Err("".to_string()),
                Err(_) => Ok(())
            }
        })
    }
    fn simple_spec_fail() -> Spec {
        it("should return 1", |_| { expect(add_one_false(0)).to_equal(1) })
    }
    fn spec_skip() -> Spec {
        it_skip("should return 1", |_| {
            expect(add_one(0)).to_equal(1)?;
            expect(add_one(0)).equals(1)?;
            expect(add_one(0)).to_be(1)?;
            expect(add_one(0)).to_not_equal(2)?;
            expect(add_one(0)).to_not_be(2)?;
            match expect(add_one(0)).to_not_equal(1) {
                Ok(_) => Err("".to_string()),
                Err(_) => Ok(())
            }
        })
    }
    fn spec_only() -> Spec {
        it_only("should return 1", |_| {
            expect(add_one(0)).to_equal(1)?;
            expect(add_one(0)).equals(1)?;
            expect(add_one(0)).to_be(1)?;
            expect(add_one(0)).to_not_equal(2)?;
            expect(add_one(0)).to_not_be(2)?;
            match expect(add_one(0)).to_not_equal(1) {
                Ok(_) => Err("".to_string()),
                Err(_) => Ok(())
            }
        })
    }
    fn simple_suite_pass() -> Suite {
        describe("add_one()")
            .specs(vec![ simple_spec_pass() ])
    }
    fn simple_suite_fail() -> Suite {
        describe("add_one_false()").specs(vec![ simple_spec_fail() ])
    }
    fn simple_suite_skip() -> Suite {
        simple_suite_pass().suites(vec![
            describe_skip("add_two()").suites(vec![ simple_suite_pass() ])
        ])
    }
    fn suite_skip_spec() -> Suite {
        let mut skipped_spec = spec_skip();
        skipped_spec.ignore = false;
        simple_suite_pass().specs(vec![
            simple_spec_pass(),
            skipped_spec
        ])
    }
    fn suite_with_only_spec() -> Suite {
        simple_suite_pass().specs(vec![
            simple_spec_pass(),
            simple_spec_pass(),
            spec_only()
        ])
    }


    #[test]
    fn test_all() {

        // simple spec pass
        simple_suite_pass().spec().run();

        // min
        simple_suite_pass().min().run();

        // json
        simple_suite_pass().json().run();

        // json pretty
        simple_suite_pass().json_pretty().run();

        // export json
        let output_path = "/tmp/laboratory-output.json";
        simple_suite_pass().json().export_to(output_path).run();
        remove_file(output_path);

        // simple fail
        simple_suite_fail().run();
        simple_suite_fail().suites(vec![ simple_suite_pass() ]).min().run();

        // skip a nested suite
        simple_suite_skip().run();

        // skip a spec
        suite_skip_spec().run();

        // exclude all specs but one
        suite_with_only_spec().run();

        // suite with state and hooks
        let _foo: Foo = simple_suite_pass().state(Foo::new())
            .before_all(|state| {
                let foo: Foo = state.get_state();
                state.set_state(foo);
            })
            // .before_each(|_| {  })
            // .after_each(|_| {  })
            // .before_all(|_| {  })
            .suites(vec![
                simple_suite_pass().inherit_state()
            ]).run().to_state();

        // describe("panic").specs(vec![
        //     it("should panic", |_| {
        //         should_panic!(panic, || { panic!("i will panic") })?;
        //         should_panic!(panic, || {  })
        //     }),
        //     it("should not panic", |_| {
        //         should_not_panic!(panic, || {  })?;
        //         should_not_panic!(panic, || { panic!("i will panic") })
        //     })
        // ]).run();

    }



}