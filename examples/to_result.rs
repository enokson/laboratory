
fn always_return_true() -> bool { true }
fn always_return_false() -> bool { true }

fn main() {
    let _true = always_return_true();
    let _false = always_return_false();
}

#[cfg(test)]
mod tests {

    use laboratory::{describe, it, expect};
    use crate::{always_return_true, always_return_false};

    #[test]
    fn test() -> Result<(), String> {

        describe("Crate").suites(vec![

            describe("always_return_true").specs(vec![

                it("should return true", |_| {

                    expect(always_return_true()).to_be(true)

                })

            ]),

            describe("always_return_false").specs(vec![

                it("should return false", |_| {

                    expect(always_return_false()).to_be(false)

                })

            ])

        ]).run().to_result()

    }

}
