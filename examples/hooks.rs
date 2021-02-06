
fn always_return_true() -> bool { true }

fn main() {
    always_return_true();
}

#[cfg(test)]
mod tests {

    use super::always_return_true;
    use laboratory::{describe, expect, LabResult};

    #[test]
    fn test() -> LabResult {

        // In this suite we want to use hooks to
        // perform actions before and after our tests.
        // The actions we want to run in this scenario is simply
        // outputting to stdout.
        describe("always_return_true()", |ctx| {

            // We want to run this action before all
            // all tests in this suite is ran. This action
            // will only be ran once.
            ctx.before_all(|| {

                println!("\n\n  before_all hook called");

            })

            // We want to run this action just before every test
            // in this suite (and child suites). Since we have two tests this action
            // will be ran twice.
            .before_each(|| {

                println!("  before_each hook called");

            })

            // likewise, we also have actions we want to run
            // after our tests.
            .after_each(|| {

                println!("  after_each hook called");

            }).after_all(|| {

                println!("  after_all hook called");

            })

            .it("should return true", |_| {

                expect(always_return_true()).to_be(true)

            })

            .it("should return true again", |_| {

                expect(always_return_true()).to_be(true)

            });


        }).run()

    }

}
