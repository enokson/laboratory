fn main() {
    add_one(0);
}

fn add_one (n: u64) -> u64 { n + 1 }

#[cfg(test)]
mod tests {

    use std::borrow::BorrowMut;

    use super::*;
    use laboratory::{describe, expect, LabResult, NullState};

    #[test]
    fn suite() -> LabResult {

        // just as there is a context for the suite, there is
        // also a context for each spec. And that context can be
        // accessed from the callback closure. However, since the test would
        // be immediately ran, whatever options you would like to include
        // will not be included until after the test is completed.

        // So, the suite also provides a spec() method where a 
        // developer can include any additional options on a per spec
        // basis before the test is ran.
        // Options such as:
        // skip(),
        // retries()
        // slow()

        describe("add_one()", |suite| {

            suite.spec(|spec| {

                spec.it("should retry ten times", |spec| {

                    println!("attempt number: {}", spec.attempts);

                    if spec.attempts < 10 {
                        Err(String::from("not enough attempts have been performed"))
                    } else {
                        Ok(())
                    }

                }).retries(9).slow(1000);

            })

            .it("should return 2 when passed 1", |_| {

                expect(add_one(1)).to_equal(2)

            });

        }).state(NullState).milis().run()

    }
}