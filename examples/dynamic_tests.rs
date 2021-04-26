fn main() {
    add_one(0);
}

fn add_one (n: u64) -> u64 { n + 1 }

#[cfg(test)]
mod tests {

    use super::*;

    use laboratory::{describe, expect, LabResult, NullState};

    #[test]
    fn suite() -> LabResult {

        describe("add_one()", |suite| {

            for i in 0..100 {
                suite.it(format!("should return {} when passed {}", i + 1, i), move |_| {
                    expect(add_one(i)).to_equal(i + 1)
                });
            }

        }).state(NullState).milis().run()

    }
}