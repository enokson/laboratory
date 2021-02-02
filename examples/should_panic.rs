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

    use laboratory::{describe, it};
    use super::*;

    #[test]
    fn should_panic() {
        describe("panic_at_the_disco()").specs(vec![

            it("should panic when passed true", |_| {
                should_panic!(panic_at_the_disco, || { panic_at_the_disco(true); })
            }),

            it("should not panic when passed false", |_| {
                should_not_panic!(panic_at_the_disco, || { panic_at_the_disco(false); })
            })

        ]).run().unwrap();

    }


}