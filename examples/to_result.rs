
fn always_return_true() -> bool { true }
fn always_return_false() -> bool { true }

fn main() {
    let _true = always_return_true();
    let _false = always_return_false();
}

#[cfg(test)]
mod tests {

    use laboratory::{LabResult, describe, expect, it};
    use crate::{always_return_true, always_return_false};

    #[test]
    fn test() -> LabResult {

        let result =  describe("Crate").suites(vec![

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

        ]).run().unwrap().to_result();

        if let Err(msg) = result {
            // the resulting error is being caught for the puposes of this example,
            // but we can still print it out (also for the purposes of this example.)
            // In the real world we would just let it fail
            // If you want to see what would happen, simply comment out this block of code
            println!("{}", msg);
            return Ok(())
        }

        result

    }

}
