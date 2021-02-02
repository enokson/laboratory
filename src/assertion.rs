use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::{panic::{catch_unwind, UnwindSafe, set_hook, take_hook}};

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
        if self.result == control {
            Ok(())
        } else {
            Err(format!("Expected {:#?} to be {:#?}", self.result, control))
        }
    }
    pub fn to_not_equal(&self, control: T) -> Result<(), String> {
        if self.result != control {
            Ok(())
        } else {
            Err(format!("Expected {:#?} not to equal {:#?}", self.result, control))
        }
    }
    pub fn to_not_be(&self, control: T) -> Result<(), String> {
        if self.result != control {
            Ok(())
        } else {
            Err(format!("Expected {:#?} not to be {:#?}", self.result, control))
        }
    }
}

pub fn expect<T>(result: T) -> Expect<T>
    where T: PartialEq + Debug
{
    Expect::new(result)
}

pub fn should_panic<T: FnOnce() + UnwindSafe>(closure: T) -> Result<(), String> {
    set_hook(Box::new(|_| {}));
    let result = catch_unwind(|| {
        (closure)()
    });
    let _ = take_hook();
    if result.is_ok() {
        Err("Expected to panic".to_string())
    } else {
        Ok(())
    }    
}
pub fn should_not_panic<T: FnOnce() + UnwindSafe>(closure: T) -> Result<(), String> {
    set_hook(Box::new(|_| {}));
    let result = catch_unwind(|| {
        (closure)()
    });
    let _ = take_hook();
    if result.is_ok() {
        Ok(())
    } else {
        Err("Expected not to panic".to_string())
    }    
}