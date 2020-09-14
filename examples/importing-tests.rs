fn main() {
    add_one::add_one(0);
    multiply_by_two::multiply_by_two(1);
}

pub mod add_one {

    pub fn add_one (x: u64) -> u64 { x + 1 }
    #[cfg(test)]
    pub mod tests {

        use super::*;
        use laboratory::{describe, it, expect, Suite};
        pub fn suite() -> Suite {

            describe("add_one()").specs(vec![

                it("should return 1", |_| {
                    expect(add_one(0)).to_equal(1)
                }),

                it("should return 2", |_| {
                    expect(add_one(1)).to_equal(2)
                })

            ])

        }

    }

}
pub mod multiply_by_two {
    pub fn multiply_by_two (x: u64) -> u64 { x * 2 }
    #[cfg(test)]
    pub mod tests {

        use super::*;
        use laboratory::{describe, it, expect, Suite};
        pub fn suite() -> Suite {

            describe("multiply_by_two()").specs(vec![

                it("should return 2", |_| {
                    expect(multiply_by_two(1)).to_equal(2)
                }),

                it("should return 4", |_| {
                    expect(multiply_by_two(2)).to_equal(4)
                })

            ])

        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use laboratory::{describe};

    #[test]
    fn test() {

        describe("Package").suites(vec![

            add_one::tests::suite(),
            multiply_by_two::tests::suite()

        ]).run();

    }

}
