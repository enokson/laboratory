fn main() {
    add_one(0);
}

fn add_one (n: u64) -> u64 { n + 1 }

#[cfg(test)]
mod tests {

    use std::borrow::BorrowMut;

    use super::*;
    use laboratory::{describe, expect, LabResult};

    #[test]
    fn suite() -> LabResult {

        describe("add_one()", |suite| {

            suite.before_all(|state| {
                state.insert("count", 0);
            }).before_each(|state| {

                let count = state.get_mut("count").unwrap();
                *count += 1;

            }).spec(|spec| {

                spec.it("should retry ten times", |spec| {

                    if let Some(count) = spec.state.borrow().get("count") {
                        if count > &10 {
                            return Ok(())
                        } else {
                            Err(String::from("fail"))
                        }
                    } else {
                        Err(String::from("fail"))
                    }

                }).retries(10).slow(1000);

            })

            .it("should return 2 when passed 1", |_| {

                expect(add_one(1)).to_equal(2)

            });

        }).milis().run()

    }
}