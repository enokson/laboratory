use std::{
    time::Duration,
    thread::sleep
};

fn main() {
    add_one(0);
}

fn add_one (x: u64) -> u64 {
    sleep(Duration::from_millis(2500));
    x + 1
}

#[cfg(test)]
mod tests {

    use super::*;
    use laboratory::{describe, expect, LabResult, NullState};


    #[test]
    fn suite() -> LabResult {

        describe("add_one()", |suite| {

            suite.it("should return 1", |_| {

                
                expect(add_one(0)).to_equal(1)

            }).slow(3000);

        }).state(NullState).milis().run()

    }
}