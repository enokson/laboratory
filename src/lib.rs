use std::cmp::PartialEq;

type TestCase = Fn() -> Result<(), String>;

struct Expect<T>
where
    T: PartialEq,
{
    result: T,
}
impl<T> Expect<T>
where
    T: PartialEq,
{
    pub fn expect(result: T) -> Expect<T> {
        Expect { result }
    }
    pub fn equals(mut self, right: T) -> Result<(), String> {
        if self.result == right {
            Ok(())
        } else {
            Err("Comparisons do not match".to_string())
        }
    }
    // pub fn is_true (&self) -> bool {
    //     if self.result == true {
    //         true
    //     } else {
    //         true
    //     }
    // }
}

struct Test<T>
where
    T: Fn() -> Result<(), String>,
{
    name: String,
    test: Option<T>,
    pass: Option<bool>,
    error_msg: Option<String>,
}
impl<T: Fn() -> Result<(), String>> Test<T>
where
    T: Fn() -> Result<(), String>,
{
    pub fn new(name: String) -> Test<T> {
        Test {
            name,
            test: None,
            pass: None,
            error_msg: None,
        }
    }
    pub fn set_test(mut self, test: T) -> Self {
        self.test = Some(test);
        self
    }
    pub fn run(mut self) -> Self {
        match self.test {
            Some(ref test) => match test() {
                Ok(_) => {
                    self.pass = Some(true);
                }
                Err(message) => {
                    self.pass = Some(false);
                    self.error_msg = Some(message);
                }
            },
            None => {}
        }
        self
    }
}

struct TestSuite<T>
where
    T: Fn() -> Result<(), String>,
{
    tests: Vec<Test<T>>,
    suites: Vec<TestSuite<T>>,
}
impl<T: Fn() -> Result<(), String>> TestSuite<T> {
    pub fn new(tests: Vec<Test<T>>, suites: Vec<TestSuite<T>>) -> TestSuite<T> {
        TestSuite { tests, suites }
    }
    pub fn run(mut self) -> Self {
        self.tests = self.tests.into_iter().map(|test| {
            test.run()
        }).collect();
        self.suites = self.suites.into_iter().map(|mut suite| {
            suite.run()
        }).collect();
        self
    }
}

#[cfg(test)]
mod test {
    use super::{Test, TestSuite, Expect};

    #[test]
    fn suite() {
        let sub_suite = TestSuite::new(
            vec![
                Test::new("should_return_1".to_string()).set_test(|| {
                    let add_one = |x| x + 1;
                    let result = add_one(0);
                    Expect::expect(result).equals(1)?;
                    Ok(())
                }),
                Test::new("should_return_2".to_string()).set_test(|| {
                    let add_one = |x| x + 1;
                    let result = add_one(1);
                    Expect::expect(result).equals(2)?;
                    Ok(())
                })
            ], vec![
                TestSuite::new(vec![
                    Test::new("should_return_3".to_string()).set_test(|| {
                        let add_one = |x| x + 1;
                        let result = add_one(2);
                        Expect::expect(result).equals(3)?;
                        Ok(())
                    }),
                    Test::new("should_return_4".to_string()).set_test(|| {
                        let add_one = |x| x + 1;
                        let result = add_one(3);
                        Expect::expect(result).equals(4)?;
                        Ok(())
                    })
                ], vec![])
            ]);
    }
}
