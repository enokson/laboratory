fn main() {
    add_one(0);
    add_two(0);
}

fn add_one (x: u64) -> u64 { x + 1 }
fn add_two (x: u64) -> u64 { x + 5 }

#[cfg(test)]
mod tests {

    use super::*;
    use laboratory::{describe, expect, LabResult, NullState};

    #[test]
    fn suite() -> LabResult {

        describe("Package", |suite| {

            suite.describe("add_one()", |suite| {

                suite.it("should return 1", |_| {

                    expect(add_one(0)).to_equal(1)

                }).it("should return 2", |_| {

                    expect(add_one(1)).to_equal(2)
                    
                });

            }).describe("add_two()", |suite| {

                suite.it("should return 2", |_| {

                    expect(add_two(0)).to_equal(2)
                    
                });

            });

        }).state(NullState).list().ignore_errors().run()

    }
}