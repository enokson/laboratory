# Hooks
Laboratory comes with four different types of hooks.
* before all
* before each
* after each
* after all

The hooks can be set by calling the respective method from the suite parameter.
```rust
describe("add_one()", |suite| {

    suite.before_all(|_| {

        println!("\n\n  before_all hook called");

    }).before_each(|_| {

        println!("  before_each hook called");

    }).after_each(|_| {

        println!("  after_each hook called");

    }).after_all(|_| {

        println!("  after_all hook called");

    });

})
```
The callback from any of these methods also pass the suite's state as a parameter.
```rust
describe("add_one()", |suite| {

    suite.before_all(|state| {

        state.insert("/counter", 0);

    }).before_each(|state| {

        let mut counter = state.get_mut(&"/counter").unwrap();
        *counter += 1;

    }).after_each(|state| {

        println!("Counter: {}", state.get(&"/counter").unwrap());

    }).after_all(|state| {

        println!("Final Count: {}", state.get(&"/counter").unwrap());

    });

})
```

The before each and after each hooks will be called for each test ran in the suite and also in every child suite.
