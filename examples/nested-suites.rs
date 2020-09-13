struct Foo {
    bar: String,
    baz: i32
}
impl Foo {
    pub fn new() -> Foo {
        Foo { bar: String::new(), baz: 0 }
    }
    pub fn append(&mut self, str: &str) {
        self.bar += str;
    }
    pub fn increase(&mut self) {
        self.baz += 1;
    }
}

fn main () {
    let mut foo = Foo::new();
    foo.append("fizzbuzz");
    foo.increase();
}

#[cfg(test)]
mod tests {

    // get the test suite
    use laboratory::{describe, it, expect};

    // get the Foo struct
    use super::*;

    // define single test
    #[test]
    fn test() {

        describe("Foo").suites(vec![

            describe("#new()").specs(vec![

                it("should return foo with two members", |_| {

                    let result = Foo::new();
                    expect(result.bar).to_be(String::new())?;
                    expect(result.baz).to_equal(0)?;
                    Ok(())

                })

            ]),

            describe("#append()").specs(vec![

                it("should append fizzbuzz to Foo#bar", |_| {
                    let mut foo = Foo::new();
                    foo.append("fizzbuzz");
                    expect(foo.bar).to_be("fizzbuzz".to_string())
                })

            ]),

            describe("#increase()").specs(vec![

                it("should increase Foo#baz by 1", |_| {
                    let mut foo = Foo::new();
                    foo.increase();
                    expect(foo.baz).to_equal(1)
                })

            ])

        ])
        .run();

    }


}


