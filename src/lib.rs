#![allow(dead_code)]

/* 

TODO: finish examples/state.rs
TODO: add examples for new features: retries, slow, skip
TODO: add example for dynamic tests with if statements and for loops
TODO: rewrite example comments for updates
TODO: build docs


*/

mod assertion;
mod reporter;
mod suite;
mod suite_context;
mod spec;
mod data;


pub use suite::{describe, Suite};
pub use assertion::{expect, should_panic, should_not_panic};
pub use data::Data;
pub type LabResult = Result<(), String>;