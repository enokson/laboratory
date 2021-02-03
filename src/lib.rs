#![allow(dead_code)]

/* 

TODO: finish examples/state.rs
TODO: add examples for new features: retries, slow, skip
TODO: add example for dynamic tests with if statements and for loops
TODO: rewrite example comments for updates
TODO: build docs


*/

mod assertion;
mod suite;

pub use suite::{describe, Suite};
pub use assertion::{expect, should_panic, should_not_panic};

pub type LabResult = Result<(), String>;