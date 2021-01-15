# Writing Automated Tests
Correctness in our programs is the extent to which our code does what we intend to do.  
Rust is designed with a high degree of concern about the correctness of programs, but correctness is complex and not easy to prove.  
Rust's type system shoulders a huge part of this burden, but the type system cannot catch every kind of incorrectness.  
As such, Rust includes support for writing automated software tests within the language.

# How to Write Tests
Tests are Rust functions that verify that the non-test code is functioning in the expected manner.  
The bodies of test functions typically perform these three actions:
1. Set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect.

## The Anatomy of a Test Function
A simple test is a function that's annotated with the `test` attribute. Attributes are metadata about pieces of Rust code.  
To change a function into a test function, add `#[test]` on the line before `fn`. When you run your tests with `cargo test` command, Rust builds a test runner binary that runs functions annotated with the `test` attribute and reports on whether each test passes or fails.  
When we make a new library project with Cargo, a test module with a test function in it is automatically generated for us.  
The `0 measured` statistic is for benchmark tests that measure performance. The `Doc-tests project-name`, is for the results of any documentation tests.  
Tests fail when something in the test function panics. Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.  

## Checking Results with the `assert!` Macro
The `assert!` macro provided by the standard library, takes in an argument that evaluates to a Boolean. If the value is `true`, `assert!` does nothing and the test passes. If the value is `false`, the `assert!` macro calls the `panic!` macro, which causes the test to fail.  

## Testing Equality with the `assert_eq!` and `assert_ne!` Macros
A common way to test functionality is to compare the result of the code under test to the value you expect the code to return to make sure they're equal.  
Standard library provides two macros `assert_eq!` and `assert_ne!` which tests equality and non equality more conveniently than `assert!`.  
These macros also print the two values if the assertion fails, which makes it easier to see why the test failed.  

## Adding Custom Failure Messages
You can also add a custom message to be printed with the failure message as optional arguments to the `assert!`, `assert_eq!` and `assert_ne!` macros.  
Any arguments specified after the required arguments are passed along to the `format!` macro.  

## Checking for Panics with `should_panic`
In addition to checking that our code returns the correct values we expect, it's also important to check that our code also handles error conditions as we expect.  
If we add `should_panic` attribute to our test function, it passes if the code inside it will panic and vice versa.  
`should_panic` has an attribute called expected which takes in a string, and the test will pass if the panic message contains a substring of the expected message.  

## Using `Result<T, E>` in Tests
We can also write tests that use `Result<T, E>` and we can return an `Err` instead of panicking.  
The test function will now have a return type, and we return `Ok(T)` if the test passes and `Err(E)` if the test fails instead of calling `assert` macros.  
Writing tests, so they return a `Result<T, E>` enables you the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant.  
You can't use the `should_panic` annotation on tests that use `Result<T, E>`