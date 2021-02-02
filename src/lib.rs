#![allow(dead_code)]

// TODO: fix examples

mod assertion;
mod suite;

pub use suite::{describe, Suite};
pub use assertion::{expect, should_panic, should_not_panic};

pub type LabResult = Result<(), String>;