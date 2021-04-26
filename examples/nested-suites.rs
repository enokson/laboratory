
// Here we will define a struct with
// two members, an associated function,
// and two methods.
struct Foo {
    line: String,
    count: i32
}
impl Foo {
    pub fn new() -> Foo {
        Foo { line: String::new(), count: 0 }
    }
    pub fn append(&mut self, str: &str) {
        self.line += str;
    }
    pub fn increase(&mut self) {
        self.count += 1;
    }
}

fn main () {
    let mut foo = Foo::new();
    foo.append("fizzbuzz");
    foo.increase();
}

#[cfg(test)]
mod tests {

    // Pull the Foo struct into scope
    use super::*;

    // Now pull in the lab tools
    use laboratory::{LabResult, describe, expect, NullState};

    // Define a single test
    #[test]
    fn test() -> LabResult {

        // Now we can describe Foo.
        // Notice how the callback closure takes an argument name "suite".
        // the suite variable is the suite's context and many child suites, specs 
        // and options can be defined using that struct.
        
        // The suite context struct has a method named describe where a developer
        // can append child suites that are relevant to the parent such as 
        // a method or member of a struct. 
        describe("Foo", |suite| {

            // Here we can describe the "new" associated function
            suite.describe("#new()", |suite| {

                suite.it("should return an instance of Foo with two members", |_| {

                    let foo = Foo::new();
                    expect(foo.line).to_be(String::new())?;
                    expect(foo.count).to_equal(0)

                });

            })

            // Now we will describe the "append" method
            .describe("#append()", |suite| {

                suite.it("should append \"fizzbuzz\" to Foo#line", |_| {

                    let mut foo = Foo::new();
                    foo.append("fizzbuzz");
                    expect(foo.line).to_be("fizzbuzz".to_string())

                });

            })

            // Finally, we will describe the "increase" method
            .describe("#increase()", |suite| {

                suite.it("should increase Foo#count by 1", |_| {

                    let mut foo = Foo::new();
                    foo.increase();
                    expect(foo.count).to_equal(1)

                });

            });

        }).state(NullState).milis().run()

    }

}


