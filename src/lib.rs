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


pub use suite::{describe, Suite, NullState};
pub use suite_context::SuiteContext;
pub use spec::SpecContext;
pub use assertion::{expect, should_panic, should_not_panic};
pub type LabResult = Result<(), String>;
