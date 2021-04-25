extern crate laboratory;

fn main() {
    panic_at_the_disco(false);
}

fn panic_at_the_disco(should_panic: bool) {
    if should_panic {
        panic!("at the disco");
    }
}

#[cfg(test)]
mod tests {

    use laboratory::{describe, LabResult, should_panic, should_not_panic, NullState};
    use super::*;

    #[test]
    fn panic_test() -> LabResult {
        
        describe("panic_at_the_disco()", |ctx| {

            ctx.it("should panic when passed true", |_| {

                should_panic(|| { panic_at_the_disco(true); })

            }).it("should not panic when passed false", |_| {

                should_not_panic(|| { panic_at_the_disco(false); })

            });

        }).state(NullState).run()

    }


}