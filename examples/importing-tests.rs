
fn main() {
    add_one::add_one(0);
    multiply_by_two::multiply_by_two(1);
}

// In this crate we have two
// public modules: add_one, and multiply_by_two

pub mod add_one {

    // here is a function that we want to test
    pub fn add_one (x: u64) -> u64 { x + 1 }


    #[cfg(test)]
    pub mod tests {

        use super::*;
        use laboratory::{describe, it, expect, Suite};

        // here is where we will define our suite.
        // Notice that this function returns a Suite struct.
        // Also notice that no other methods are called on this suite.
        pub fn suite() -> Suite {

            describe("add_one()").specs(vec![

                it("should return 1", |_| {
                    expect(add_one(0)).to_equal(1)
                }),

                it("should return 2", |_| {
                    expect(add_one(1)).to_equal(2)
                })

            ])

        }

    }

}

// here is our second module
pub mod multiply_by_two {

    // ...and the function we want to test
    pub fn multiply_by_two (x: u64) -> u64 { x * 2 }

    #[cfg(test)]
    pub mod tests {

        use super::*;
        use laboratory::{describe, it, expect, Suite};

        // Again, we will define a function that returns a Suite struct
        pub fn suite() -> Suite {

            describe("multiply_by_two()").specs(vec![

                it("should return 2", |_| {
                    expect(multiply_by_two(1)).to_equal(2)
                }),

                it("should return 4", |_| {
                    expect(multiply_by_two(2)).to_equal(4)
                })

            ])

        }

    }
}

// Now here is where we will import and run our
// tests under one umbrella of the crate.
#[cfg(test)]
mod tests {

    // pull our modules into scope
    use super::*;

    // pull in our lab tools
    use laboratory::{describe};

    #[test]
    fn test() {

        // Describe the crate.
        describe("My Crate")
            .suites(vec![

                // now we will call our functions that simply
                // returns a Suite struct.
                add_one::tests::suite(),
                multiply_by_two::tests::suite()

            ])

            // Now we can run our tests with any other options
            .run().unwrap();

    }

}
