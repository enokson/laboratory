
fn main() {
    add_one::add_one(0);
    multiply_by_two::multiply_by_two(1);
}

// In this crate we have two
// public modules: 
//   - add_one, and
//   - multiply_by_two

// here is the first module
pub mod add_one {

    // and here is its function that we want to test
    pub fn add_one (x: u64) -> u64 { x + 1 }

    #[cfg(test)]
    pub mod tests {

        use super::*;
        use laboratory::{describe, expect, Suite};

        // Here is where we will define our suite.
        // Notice that this function returns a Suite struct.
        // Also notice that no other methods are called on this suite.
        pub fn suite<T>() -> Suite<T> {

            describe("add_one()", |ctx| {
                
                ctx.it("should return 1", |_| {
                    expect(add_one(0)).to_equal(1)
                })

                .it("should return 2", |_| {
                    expect(add_one(1)).to_equal(2)
                });

            })

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
        use laboratory::{describe, expect, Suite};

        // Again, we will define a function that returns a Suite struct
        pub fn suite<T>() -> Suite<T> {

            describe("multiply_by_two()", |ctx| {

                ctx.it("should return 2", |_| {
                    expect(multiply_by_two(1)).to_equal(2)
                })

                .it("should return 4", |_| {
                    expect(multiply_by_two(2)).to_equal(4)
                });

            })

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
    use laboratory::{describe, LabResult, NullState};

    #[test]
    fn test() -> LabResult {

        // Describe the crate.
        describe("My Crate", |context| {
            context
                .describe_import(add_one::tests::suite())
                .describe_import(multiply_by_two::tests::suite());
        }).state(NullState)

        // Now we can run our tests with any other options
        .run()

    }

}
