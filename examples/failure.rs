
// add_one should add one to any number,
// but a typo results in 6 being added
fn add_one (n: i32) -> i32 { n + 6 }

fn add_two (n: i32) -> i32 { n + 2 }

fn main() {
    add_one(0);
    add_two(0);
}

#[cfg(test)]
mod tests {

    // let call in our functions
    use super::*;

    // and now let's bring in our lab tools
    use laboratory::{describe,it,expect};

    // define our single rust test
    #[test]
    fn test() {

        // We have two different functions that we
        // want to test in our crate. So, let's
        // describe our crate and nest our functions
        // under that umbrella.
        describe("Crate").suites(vec![

            describe("add_one()").specs(vec![

                it("should return 1 to when passed 0", |_| {

                    expect(add_one(0)).to_equal(1)

                })

            ]),

            describe("add_two()").specs(vec![

                it("should return 2 to when passed 0", |_| {

                    expect(add_two(0)).to_equal(2)

                })

            ])

        ]).run().unwrap();

    }

}
