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