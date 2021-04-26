# State

## Overview
At the end of the day the state is simply a HashMap wrapped inside of a Reference Counter and RefCell with <code>&'static str</code> as the key and a generic type as the value, e.g. <code>Rc<RefCell<HashMap<&'static str, T>>></code>. This allows for more than one state value to be held at time, however, it also means all state values must be of the same type. More on that later.

The state resides as a member of the SuiteContext as well as the SpecContext. So, when defining a suite one can access the state inside the callback handle.
```rust
describe("add_one()", |suite| {

    let mut state = suite.state.borrow_mut();
    state.insert("/my-state", "my-value".to_string());    

})
```
also in the callback handle when defining a spec:
```rust
describe("add_one()", |suite| {

    let mut state = suite.state.borrow_mut();
    state.insert("/my-state", "my-value".to_string());
    
    suite.it("should return 1", |spec| {

        let state = spec.state.borrow();
        let my_state = state.get("/my-state").unwrap();
        println!("{}", my_state); // => "my-value"

        expect(add_one(0)).to_be(1)

    })

})
```

The state can also be directly accessed in hook handles as Laboratory passes the state as an argument to those handles.

```rust
describe("add_one()", |suite| {

    suite.before_all(|state| {

        state.insert("/counter", 0);

    }).after_all(|state| {

        println!("count: {}", state.get(&"/counter").unwrap());

    });

})
```

When two differing types are needed to be used as state values there are two decent solutions. The first is to use an enum and the second is to define a struct with members of different types.
```rust
enum State {
    Phrase(String),
    Counter(i32)
}

describe("add_one()", |suite| {

    suite.before_all(|state| {

        state.insert("/counter", State::Counter(0));
        state.insert("/phrase", State:Phrase("Hello, world!".to_string()));

    });

})
```
or
```rust
struct State {
    counter: i32,
    phrase: String
}

describe("add_one()", |suite| {

    suite.before_all(|state| {

        let state_container = State {
            counter: 0,
            phrase: "Hello, world!".to_string()
        };

        state.insert("/state", state_container);

    });

})
```
Since the state is shared throughout the test runner including children suites, it may be advised to use path-like structures for designating namespaces.
```rust
describe("my crate", |suite| {

    suite.before_all(|state| {

        state.insert("/", 0);

    }).describe("add_one()", |suite| {

        suite.before_all(|state| {
    
            state.insert("/add_one()/counter", 0);
    
        }).describe("when given a max int", |suite| {

            suite.before_all(|state| {
        
                state.insert("/add_one()/max-int/counter", 0);
        
            });
        
        });
    
    }).describe("add_two()", |suite| {
        
        suite.before_all(|state| {
    
            state.insert("/add_two()/counter", 0);
    
        });

    });

}).run()
```


## A Simple Example
```rust
fn add_one(n: i32) -> i32 { n + 1 }

fn main() {
    let _one = add_one(0);
}

#[cfg(test)]
mod tests {

    use laboratory::{describe, expect, LabResult, Suite};
    use super::{ add_one };

    #[test]
    fn test() -> LabResult {

        describe("add_one()", |suite| {

            suite.before_all(|state| {

                state.insert("/counter", 0);

            }).after_all(|state| {

                println!("count: {}", state.get(&"/counter").unwrap());

            }).it("should return 1", |spec| {

                let mut state = spec.state.borrow_mut();
                let mut counter = state.get_mut("/counter").unwrap();
                *counter += 1;

                expect(add_one(0)).to_be(1)

            }).it("should return 2", |spec| {
                
                let mut state = spec.state.borrow_mut();
                let mut counter = state.get_mut("/counter").unwrap();
                *counter += 1;
            
                expect(add_one(1)).to_be(2)

            });

        }).run()

    }

}
```
Result:
```
running 1 test
count: 2


### Lab Results Start ###

add_one()
  ✓  should return 1 (1666ns)
  ✓  should return 2 (892ns)
✓ 2 tests completed (2558ns)

### Lab Results End ###
```
