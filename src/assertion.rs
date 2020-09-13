use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::panic::catch_unwind;



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
    pub fn equals(&self, control: T) -> Result<(), String> {
        if self.result == control {
            Ok(())
        } else {
            Err(format!("Expected {:#?} to equal {:#?}", self.result, control))
        }
    }
    pub fn to_equal(&self, control: T) -> Result<(), String> {
        self.equals(control)
    }
    pub fn to_be(&self, control: T) -> Result<(), String> {
        self.equals(control)
    }
    pub fn to_not_equal(&self, control: T) -> Result<(), String> {
        if self.result != control {
            Ok(())
        } else {
            Err(format!("Expected {:#?} to equal {:#?}", self.result, control))
        }
    }
    pub fn to_not_be(&self, control: T) -> Result<(), String> {
        self.to_not_equal(control)
    }
    pub fn to_panic<H>(&self) -> Result<(), String> {
        let tmp_result = catch_unwind(|| {

        }).is_ok();
        if tmp_result == false {
            Ok(())
        } else {
            Err(format!("Expected"))
        }
    }
}

