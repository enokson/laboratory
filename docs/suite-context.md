# SuiteContext
Each Suite has a SuiteContext as a member. The context can be accessed in the describe function's callback closure.

```rust
describe("my_function()", |suite_context| {

}) // => Suite
```

## Hooks
### SuiteContext.before_all()
### SuiteContext.before_each()
### SuiteContext.after_each()
### SuiteContext.after_all()
Checkout the [hooks](hooks.md) section to learn more.

## Specs

### SuiteContext.it()
### SuiteContext.it_skip()
### SuiteContext.it_only()
The it() method is the primary way of defining a spec. By appending _skip a developer can easily pass over the test.

In contrast it_only() will tell the suite to only run this spec and skip all others in the block (not children blocks).

```rust
describe("my_function()", |suite_context| {
    
    // defines the spec
    suite_context.it("should do stuff", |_| {

    }); // => SuiteContext

    // skips the spec, not running the test
    suite_context.it_skip("should do stuff", |_| {

    }); // => SuiteContext

    // skips all other tests
    suite_context.it_only("should do stuff", |_| {

    }); // => SuiteContext

}) // => Suite
```

### SuiteContext.spec()
 just as there is a context for the suite, there is also a context for each spec. And that context can be accessed from the callback closure. However, since the test would  be immediately ran, whatever options you would like to include will not be included until after the test is completed. So, the suite also provides a spec() method where a developer can include any additional options on a per spec basis before the test is ran.

So if a developer wanted a test to try four times before moving on, the following example would NOT produce the desired result as the test would already be completed before the suite knew to retry.
```rust
describe("my_function()", |suite_context| {
    
    suite_context.it("should do stuff", |spec| {

        spec.retries(3); // => SpecContext

    }); // => SuiteContext

}) // => Suite
```
Here is the solution to the issue.
```rust
describe("my_function()", |suite_context| {
    
    suite_context.spec(|spec| {

        spec.it("should do stuff", |_| {

        }.retries(3)); // => SpecOptions

    }); // => SuiteContext

}) // => Suite
```
In the above example the spec's actual test closure is held until all other options have been passed to the spec and then ran.

## Child Suites
### SuiteContext.describe()
### SuiteContext.describe_skip()
### SuiteContext.describe_only()
### SuiteContext.describe_import()
### SuiteContext.describe_import_skip()
### SuiteContext.describe_import_only()
The above methods are used to describe child suites. Appending _skip will pass over all other suites in that block and _only will pass over all other suites except that one.

The import methods are used for importing suites from other locations in one's code, such as other modules. Likewise, the _skip and _only methods are used to exclude tests.

```rust

fn suite_from_other_module<T>() -> Suite<T> {

    describe("#method_4()", |suite_context| {
    
    })

}

describe("MyStruct", |suite_context| {
    
    // defines a child suite
    suite_context.describe("#method_1()", |suite_context| {

    }); // => SuiteContext

    // skips a child suite
    suite_context.describe_skip("#method_2()", |suite_context| {
        
    }); // => SuiteContext

    // excludes all other child suites
    suite_context.describe_only("#method_3()", |suite_context| {
        
    }); // => SuiteContext

    // imports a foreign suite as a child
    suite_context.describe_import(suite_from_other_module());

}) // => Suite
```

### SuiteContext.skip()
This method is used to tell the parent suite that this suite should be skipped.
```rust
describe("MyStruct", |suite_context| {
    
    // this suite will be skipped
    suite_context.describe("#method_1()", |suite_context| {

        suite_context.skip(); // => SuiteContext

    }); // => SuiteContext

    suite_context.describe("#method_2()", |suite_context| {

    }); // => SuiteContext

}) // => Suite
```

## SuiteContext.retries()
This method is used to tell the parent suite that each spec should retry x number of times if that spec should fail.
```rust
describe("my_temperamental_function()", |suite_context| {
    
    suite_context.retries(3); // => SuiteContext

}) // => Suite
```

## SuiteContext.slow()
This method is used to tell the parent suite when any spec should be considered "slow". The number given to the slow() method should coincide with the suite's time duration metric. If the metric is in milliseconds then the number given to the method should also be in milliseconds.

If the time taken for the test to run comes in under half the time given, the test will be considered "fast" and will be displayed in green. If the time taken comes in over half the time given, but under the threshold, then the test will be considered "on-time" and will be displayed in yellow. Lastly, if the time taken comes in above the time given, the test is considered "slow" and will be displayed in red.
```rust
describe("my_time_intensive_function", |suite_context| {
    
    suite_context.slow(2000); // => SuiteContext

}).milis().run() // => Suite
```