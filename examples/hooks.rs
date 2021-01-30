
fn always_return_true() -> bool { true  }

fn main() {
    always_return_true();
}

#[cfg(test)]
mod tests {

    use super::always_return_true;
    use laboratory::{describe, it, expect};

    #[test]
    fn test() {

        // In this suite we want to use hooks to
        // perform actions before and after our tests.
        // The actions we to run in this scenario is simply
        // outputting to to stdout.
        describe("always_return_true")

            // We want to run this action before all
            // all tests in this suite is ran. This action
            // will only be ran once.
            .before_all(|_| {

                println!("\n\n  before_all hook called");
                Ok(())

            })

            // We want to run this action just before every test
            // in this suite. Since we have two tests this action
            // will be ran twice.
            .before_each(|_| {

                println!("  before_each hook called");
                Ok(())

            })

            // likewise, we also have actions we want to run
            // after our tests.
            .after_each(|_| {

                println!("  after_each hook called");
                Ok(())

            }).after_all(|_| {

                println!("  after_all hook called");
                Ok(())

            }).specs(vec![

                it("should return true", |_| {
                    expect(always_return_true()).to_be(true)
                }),

                it("should return true again", |_| {
                    expect(always_return_true()).to_be(true)
                })

            ]).run();

    }

}
