use std::cmp::PartialEq;


pub struct Expect<T>
    where
        T: PartialEq,
{
    pub result: T
}
impl<T> Expect<T>
    where
        T: PartialEq,
{
    pub fn expect(result: T) -> Expect<T> {
        Expect { result }
    }
    pub fn equals(&self, right: T) -> Result<(), String> {
        if self.result == right {
            Ok(())
        } else {
            Err("Comparisons do not match".to_string())
        }
    }
    pub fn to_equal(&self, right: T) -> Result<(), String> {
        self.equals(right)
    }
    pub fn to_be(&self, right: T) -> Result<(), String> {
        self.equals(right)
    }
}
