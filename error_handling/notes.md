# Error Handling
In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile.  
There are two types of errors in Rust, recoverable and unrecoverable errors.  
In recoverable errors, you can just report the error and retry the operation again.
Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of the array.  

# Unrecoverable Errors with `panic!`
When the `panic!` macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.  
This most commonly occurs when a bug of some kind has been detected, and it's not clear to the programmer how to handle the error.  

## Unwinding the Stack or Aborting in Response to a Panic
By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters.  
This process of unwinding is a lot of work. But, there is an alternative to this, aborting immediately, which ends the program without cleaning up.  
Memory that was used by the program will be cleaned up by the operating system.  
This also makes the size of the resulting binary small.  
Add `panic = 'abort'` in cargo.toml to enable this behavior.

## Calling `panic!`
When we call `panic!` macro, the program compiles, but exits at runtime with our message, and the line where we called panic.  
In other cases, calling panic might not be in our code, we would be calling some other code with has panic, so it would be showing the line where panic is called.  
We can use the backtrace to figure out which part of our code is calling the panic code.  

## Using a `panic!` Backtrace
If a `panic!` call comes from a library or which is not in Rust source code, the error would point at that line, not at the code we wrote.  
So, we need to go through the backtrace to see which part of our code triggers the panic call.  
To see the entire backtrace, we need to set `RUST_BACKTRACE=1` environment variable before running the code.  
A backtrace is a list of functions that have been called to get to this point. The point of origin will be at the top.  
These functions might include core Rust code, standard library code, or the crate we're using.  

# Recoverable Errors with `Result`
Most errors aren't serious enough to require the program to stop entirely.  
Sometimes, when a function fails, it's for a reason that you can easily interpret and respond to.  
For example, if you try to open a file and that operation fails because the file doesn't exist, you might want to create the file instead of terminating the process.  
`Result` is an enum defined as having two variants, `Ok` and `Err`.  
```rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```
`T` represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error will be returned in a failure case within the `Err` variant.  
`std::fs::File::open` is a method which returns a `Result<std::fs::File, std::io::Error>`.  
This return type means the call to `File::open` might succeed and return a file handle that we can read or write to, or it might fail because it didn't find a file, or we might not have permission to access the file.  
Just like the Option enum, the Result enum and it's variants have been brought into scope by the prelude.  
We can write a `match` statement to return file for `Ok` and `panic!` for `Err`.  

## Matching on Different Errors
Our code will panic no matter what error we get, instead what we can do is, create the file on not found error and panic on other errors.  
`io::Error` is a struct in the `std` library, this error has a method called `kind`, returns an `ErrorKind` enum. ErrorKind has a lot of variants and `NotFound` is one of them.  

## Shortcuts for Panic on Error: `unwrap` and `expect`
`unwrap` is a method on `Result` enum, if the Result value is Ok variant, this method will return the value inside in it.  
If the Result value is Err variant, this method will call the `panic!` macro.  
`expect` is another method on `Result` enum, which lets us choose the panic message.

## Propagating Errors
If we're writing a function whose implementation calls something that might fail, instead of handling the error within this function, we can return the error to the calling code so that it can decide what to do.  
This is known as propagating the error and gives more control to the calling code.

## A Shortcut for Propagating Errors: the `?` Operator
If `?` operator is placed after a `Result` value and if its variant is `Ok`, then it'll return the value inside Ok to the expression, or else the `Err` will be returned from the whole function as if we used the `return` keyword.  
Error values that have `?` operator on them go through the `from` function defined in the `From` trait in the standard library, which is used to convert errors from one type into another.  
When the `?` operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function. 

# To `panic!` or Not to `panic!`
If we call `panic!`, we are making a decision that the error from the code is unrecoverable, whether if it's recoverable or not.  
If we return `Result`, we give the calling code options rather than make decisions for it.  
Returning `Result` is a good default choice when you're defining a function that might fail.  
In rare situations, it's more appropriate to write code that panics instead of returning a `Result`.  

## Examples, Prototype Code, and Tests
If we're writing an example to illustrate a concept, having robust error handling code in the example will make the example less clear.  
Methods like `unwrap` and `expect` are very handy when prototyping, before you are ready to decide how to handle errors.  
If a method call fails in a test, you'd want the whole test to fail, even if that method isn't the functionality under test.  

## Cases in Which You Have More Information Than the Compiler
Sometimes, we really know that the code we're writing does not panic at all, like parsing a string containing a valid IP address.  
We don't want to have robust error handling there, since we know the code doesn't fail. Using `unwrap` is the best choice.  

## Guidelines for Error Handling
It is better that your code panic when it's possible that it might end up in bad state.  
A bad state is when some assumption, guarantee, contract or invariant has been broken, such as when invalid values, contradictory values or missing values are passed to your code.  
* The bad state is not something that's expected to happen occasionally.
* Your code after this point needs to rely on not being in this bad state.
* There's not a good way to encode this information in the types you use.

`panic!` is often appropriate if you're calling external code that is out of your control, and it returns and invalid state that you have no way of fixing.  
However, when a failure is expected, it's more appropriate to return a `Result` than to make a `panic!` call.  
For example, a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit.  
In these cases, returning a `Result` indicates that failure is an expected possibility that the calling code must decide how to handle.  

When your code performs operations on values, your code should verify the values are valid first and panic if the values aren't valid.  
Functions have contracts: their behaviour is only guaranteed if the inputs meet particular requirements.  
Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug, and it's not the kind of error you want the calling code to have to explicitly handle.  
Having lots of error checks in all of your functions would be verbose and annoying. Fortunately, you can use Rust's type system(and thus type checking the compiler does) to do many checks for you.  