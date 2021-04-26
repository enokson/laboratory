# Suites

## describe()
The describe function is the ideal way to create a top level suite.
```rust
describe("my_function()", |suite| {

}) // => Suite
```

All options passed to the suite are also passed along to any child suites.

### Suite.run()
The run method begins the process of running each test in the suite.
```rust
describe("my_function()", |suite| {

}).run() // => LabResult
```
### Suite.state()
Because the suite contains a generic type, that type must be known at compile time. The compiler can infer the type if a developer ever assigns a value to the state. But in many cases using state is unnecessary and so is never assigned a value or type. This leaves the compiler unhappy holding an unknown generic type. To alleviate the situation, Laboratory provides the suite with the state() method. Here a developer can give the suite a state type. It could be any type, but Laboratory does provide a NullState type to explicitly state a null value.
```rust
describe("my_function()", |suite| {

}).state(NullState).run() // => LabResult
```

### Suite.ignore_errors()
Laboratory is considered a layer 2 test runner that runs on top of rust's layer 1 test runner. That being the case, running a test will return a result from both laboratory and rust. 

By default failing a lab test will result in a failed rust test as well, however, it may be beneficial in some cases for a failed lab result not be passed to the layer 1 test runner. 

For example, in the Laboratory [examples](https://github.com/enokson/laboratory/tree/master/examples) section, there are many tests that fail but fails on purpose as part of the example and so is actually a success.
```rust
describe("my_function()", |suite| {

}).ignore_errors().run() // => LabResult
```

## Reporters
### Suite.spec()
### Suite.min()
### Suite.dot()
### Suite.list()
### Suite.tap()
### Suite.rust()
### Suite.json()
### Suite.json_pretty()
A suite provides the above methods to produce the desired reporting style. Take a look at the [reporters](reporters.md) section to choose which style is right for your project. The default style is spec.
```rust
describe("my_function()", |suite| {

}).json_pretty().run() // => LabResult
```

## Time Durations
### Suite.nano()
### Suite.micro()
### Suite.milis()
### Suite.sec()
A suite provides the above methods to produce the desired time metrics for reporting. The default time duration metric is nano.
```rust
describe("my_function()", |suite| {

}).micro().run() // => LabResult
```