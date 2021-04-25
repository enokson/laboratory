
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
        // Notice the "suites" method that takes a Vec
        // as its argument. This is where we can describe
        // Foo's members and methods.
        describe("Foo", |ctx| {

            // Here we can describe the "new" associated function
            ctx.describe("#new()", |ctx| {

                ctx.it("should return an instance of Foo with two members", |_| {

                    let foo = Foo::new();
                    expect(foo.line).to_be(String::new())?;
                    expect(foo.count).to_equal(0)

                });

            })

            // Now we will describe the "append" method
            .describe("#append()", |ctx| {

                ctx.it("should append \"fizzbuzz\" to Foo#line", |_| {

                    let mut foo = Foo::new();
                    foo.append("fizzbuzz");
                    expect(foo.line).to_be("fizzbuzz".to_string())

                });

            })

            // Finally, we will describe the "increase" method
            .describe("#increase()", |ctx| {

                ctx.it("should increase Foo#count by 1", |_| {

                    let mut foo = Foo::new();
                    foo.increase();
                    expect(foo.count).to_equal(1)

                });

            });

        }).state(NullState).milis().run()

    }

}


