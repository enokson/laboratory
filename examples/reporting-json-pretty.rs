
fn main() {
    let _one = add_one(0);
    let _two = add_two(0);
}

fn add_one (x: u64) -> u64 { x + 1 }
fn add_two (x: u64) -> u64 { x + 5 }

#[cfg(test)]
mod tests {

    use super::*;
    use laboratory::{describe, it, expect};

    #[test]
    fn suite() {

        // To export to json-pretty we will simply call
        // the json_pretty method on the suite.
        describe("My Crate").suites(vec![

            describe("add_one()").specs(vec![

                it("should return 1", |_| {
                    expect(add_one(0)).to_equal(1)
                }),

                it("should return 2", |_| {
                    expect(add_one(1)).to_equal(2)
                })

            ]),

            describe("add_two()").specs(vec![

                it("should return 2", |_| {
                    expect(add_two(0)).to_equal(2)
                })

            ])

        ]).json_pretty().run();

    }
}