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
