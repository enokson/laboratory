
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

    // let's bring in our functions
    use super::*;

    // and now let's bring in our lab tools
    use laboratory::{LabResult, describe, expect, NullState};

    // here we define our single rust test
    #[test]
    fn test() -> LabResult {

        // We have two different functions that we
        // want to test in our crate. So, let's
        // describe our crate and nest our functions
        // under that umbrella.
        let test_results = describe("Crate", |ctx| {
            
            ctx.describe("add_one()", |ctx| {
                
                ctx.it("should return 1 to when passed 0", |_| {

                    expect(add_one(0)).to_equal(1)

                });

            })

            .describe("add_two()", |ctx| {
                
                ctx.it("should return 2 to when passed 0", |_| {

                    expect(add_two(0)).to_equal(2)

                });

            });

        }).state(NullState).run();

        // the run method returns a result. So, if any of
        // our tests fail the result will return an error as well.
        // I catch the error in this example, but generally you should
        // allow the suite to return the result to the function. Feel
        // free to uncomment the "return Err(msg);" line to see the result
        // in the terminal.
        if let Err(_msg) = test_results {
            // return Err(msg);
        }

        Ok(())

    }

}
