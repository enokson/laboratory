use std::cmp::PartialEq;
use std::fmt::{Debug, Display};

pub struct Expect<T>
    where
        T: PartialEq + Debug + Display,
{
    pub result: T
}
impl<T> Expect<T>
    where
        T: PartialEq + Debug + Display,
{
    pub fn expect(result: T) -> Expect<T> {
        Expect { result }
    }
    pub fn equals(&self, control: T) -> Result<(), String> {
        if self.result == control {
            Ok(())
        } else {
            Err(format!("Expected {} to equal {}", self.result, control))
        }
    }
    pub fn to_equal(&self, control: T) -> Result<(), String> {
        self.equals(control)
    }
    pub fn to_be(&self, control: T) -> Result<(), String> {
        self.equals(control)
    }
}
