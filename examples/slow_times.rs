fn main() {
    add_one(0);
    add_two(0);
}

fn add_one (x: u64) -> u64 { x + 1 }
fn add_two (x: u64) -> u64 { x + 2 }

#[cfg(test)]
mod tests {

    use super::*;
    use laboratory::{describe, expect, LabResult, NullState};
    use std::{
        time::Duration,
        thread::sleep
    };

    #[test]
    fn suite() -> LabResult {

        describe("add_one()", |suite| {

            suite.it("should return 1", |_| {

                sleep(Duration::from_millis(2500));
                expect(add_one(0)).to_equal(1)

            }).slow(3);

        }).state(NullState).sec().run()

    }
}