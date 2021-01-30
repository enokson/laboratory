
fn main() {
    add_one(0);
    add_two(0);
}

fn add_one (x: u64) -> u64 { x + 1 }
fn add_two (x: u64) -> u64 { x + 5 }

#[cfg(test)]
mod tests {

    use super::*;
    use laboratory::{describe, it, expect};
    use std::path::Path;

    #[test]
    fn suite() {

        describe("Package").suites(vec![

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

        ]).json_pretty().export_to(Path::new("./examples/output/test-result.json").to_str().expect("Could make new path"))
        .run().unwrap();

    }

}