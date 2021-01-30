fn main() {
    add_one(0);
}

fn add_one (x: u64) -> u64 { x + 1 }

#[cfg(test)]
mod tests {

    use super::*;
    use laboratory::{describe, it, expect, it_only};

    #[test]
    fn suite() {

        describe("add_one()").specs(vec![

            it("should return 1", |_| {
                expect(add_one(0)).to_equal(1)
            }),

            it_only("should return 2", |_| {
                expect(add_one(1)).to_equal(2)
            }),

            it("should return 3", |_| {
                expect(add_one(2)).to_equal(3)
            })

        ]).run().unwrap();

    }
}