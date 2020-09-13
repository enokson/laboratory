
fn add_one (n: i32) -> i32 { n + 6 }

fn add_two (n: i32) -> i32 { n + 2 }

fn main () {
    add_one(0);
    add_two(0);
}

#[cfg(test)]
mod tests {

    use laboratory::{describe,it,expect};
    use super::*;

    #[test]
    fn test() {

        describe("Package").suites(vec![

            describe("add_one()").specs(vec![

                it("should return 1 to when given 0", |_| {
                    expect(add_one(0)).to_equal(1)
                })

            ]),

            describe("add_two()").specs(vec![

                it("should return 2 to when given 0", |_| {
                    expect(add_two(0)).to_equal(2)
                })

            ])

        ]).run();

    }

}