#[macro_use] extern crate laboratory;

fn main() {}

fn panic_at_at_disco(should_panic: bool) {
    if should_panic {
        panic!("at the disco");
    }
}

#[cfg(test)]
mod tests {

    use laboratory::{describe, it};
    // use std::panic::{set_hook, take_hook, catch_unwind};

    use super::*;

    #[test]
    fn test() {
        describe("panic_at_at_disco()")
            .specs(vec![

                it("should panic when passed true", |_| {
                    should_panic!(panic_at_at_disco, || { panic_at_at_disco(true); })
                }),

                it("should not panic when passed false", |_| {
                    should_not_panic!(panic_at_at_disco, || { panic_at_at_disco(false); })
                })

            ]).run();
    }

}