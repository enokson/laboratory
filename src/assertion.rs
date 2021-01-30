use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::panic::catch_unwind;
use super::error::Error;

pub struct Expect<T>
    where
        T: PartialEq + Debug,
{
    pub result: T
}
impl<T> Expect<T>
    where
        T: PartialEq + Debug,
{
    pub fn new(expect: T) -> Expect<T> { Expect { result: expect } }
    pub fn expect(result: T) -> Expect<T> {
        Expect { result }
    }
    pub fn equals(&self, control: T) -> Result<(), Error> {
        if self.result == control {
            Ok(())
        } else {
            Err(Error::Assertion(format!("Expected {:#?} to equal {:#?}", self.result, control)))
        }
    }
    pub fn to_equal(&self, control: T) -> Result<(), Error> {
        self.equals(control)
    }
    pub fn to_be(&self, control: T) -> Result<(), Error> {
        if self.result == control {
            Ok(())
        } else {
            Err(Error::Assertion(format!("Expected {:#?} to be {:#?}", self.result, control)))
        }
    }
    pub fn to_not_equal(&self, control: T) -> Result<(), Error> {
        if self.result != control {
            Ok(())
        } else {
            Err(Error::Assertion(format!("Expected {:#?} not to equal {:#?}", self.result, control)))
        }
    }
    pub fn to_not_be(&self, control: T) -> Result<(), Error> {
        if self.result != control {
            Ok(())
        } else {
            Err(Error::Assertion(format!("Expected {:#?} not to be {:#?}", self.result, control)))
        }
    }
}

