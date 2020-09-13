
fn no_op() -> bool { true  }

fn main() { no_op(); }

#[cfg(test)]
mod tests {

    use super::no_op;
    use laboratory::{describe, it, expect};

    #[test]
    fn test() {

        describe("no_op").before_all(|_| {
            println!("\n\nbefore hook called");
        }).before_each(|_| {
            println!("before_each hook called");
        }).after_each(|_| {
            println!("after_each hook called");
        }).after_all(|_| {
            println!("after_all hook called");
        }).specs(vec![

            it("should do nothing", |_| {
                expect(no_op()).to_be(true)
            }),

            it("should do nothing again", |_| {
                expect(no_op()).to_be(true)
            })

        ]).run();

    }

}
