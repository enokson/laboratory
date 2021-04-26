extern crate laboratory;

fn main() {
    panic_at_the_disco(false);
}

fn panic_at_the_disco(should_panic: bool) {
    if should_panic {
        panic!("at the disco");
    }
}

// Sometimes it is necessary to ensure that a 
// program will panic under certain conditions.
// Laboratory comes with two functions to test whether
// a function being tested had indeed panicked without crashing
// the underlying Laboratory test runner.

#[cfg(test)]
mod tests {

    use laboratory::{describe, LabResult, should_panic, should_not_panic, NullState};
    use super::*;

    #[test]
    fn panic_test() -> LabResult {
        
        describe("panic_at_the_disco()", |suite| {

            suite.it("should panic when passed true", |_| {

                should_panic(|| { panic_at_the_disco(true); })

            }).it("should not panic when passed false", |_| {

                should_not_panic(|| { panic_at_the_disco(false); })

            });

        }).state(NullState).ignore_errors().run()

    }

}