
fn always_return_true() -> bool { true }
fn add_one(n: i32) -> i32 { n + 1 }
fn add_two(n: i32) -> i32 { n + 2 }

fn main() {
    let _true = always_return_true();
    let _one = add_one(0);
    let _two = add_two(0);
}

#[cfg(test)]
mod tests {

    use laboratory::{describe, expect, LabResult, Suite};
    use super::{always_return_true, add_one, add_two};

    #[test]
    fn test() -> LabResult {

        // Using state for tests is straight forward process
        // with Laboratory.

        // The state is simply a Rc<RefCell<HashMap<&'static str, T>>.
        // Depending on where you try to access the state, sometimes
        // the outer Rc and RefCell containers are abstracted away.
        // An example of this are the hooks where one has direct
        // access to the HashMap.

        // Since we are using a HashMap, any number of key-value pairs
        // can be used as state values to be read and updated throughout
        // the Laboratory test runner.

        describe("My Crate", |suite| {

            suite.before_all(|state| {

                state.insert("/counter", 0);
               
            }).before_each(|state| {

                let mut counter = state.get_mut("/counter").unwrap();
                *counter += 1;

            }).after_all(|state| {

                println!("counter: {:?}", state.get("/counter").unwrap());

            }).describe("add_one()", |suite| {

                suite.it("should return 1", |spec| {

                    let counter = spec.state.borrow().get("/counter").unwrap();

                    expect(add_one(0)).to_be(1)

                }).it("should return 2", |_| {
                
                    expect(add_one(1)).to_be(2)

                });

            }).describe("add_two()", |suite| {

                suite.before_all(|state| {
                    
                    state.insert("/add_two()/counter", 0);

                }).before_each(|state| {

                    let mut counter = state.get_mut("/add_two()/counter").unwrap();
                    *counter += 1;

                }).after_all(|state| {

                    println!("add_two counter: {:?}", state.get("/add_two()/counter").unwrap());
                    
                })
                .it("should return 2", |spec| {

                    expect(add_two(0)).to_be(2)

                }).it("should return 4", |_| {

                    expect(add_two(2)).to_be(4)

                });

            });

        }).run()

    }

}
