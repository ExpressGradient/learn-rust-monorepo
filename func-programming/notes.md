# Functional Language Features: Iterators and Closures
Programming in a functional style often includes using functions as values by passing them in arguments, returning them from other functions and assigning them to variables for later execution and so forth.  

# Closures: Anonymous Functions that Can Capture Their Environment
Rust's closures are anonymous functions you can save in a variable or pass as arguments to other functions.  
Closures don't require you to annotate the types of the parameters, or the return value like `fn` functions do.  

## Closure Type Inference and Annotation
Type annotations are required on functions because they're part of an explicit interface exposed to your users.  
But closures aren't used in an exposed interface like this; they're stored in variables and used without naming them and exposing them to the users of our library.  
Closures are usually short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler is reliably able to infer the types of the parameters and the return type, similar to variables.  

## Capturing the Environment with Closures
Unlike functions, closures can capture values from the scope in which they're defined.  
When a closure captures a value from its environment, it uses memory to store the values for use in the closure body.  
This use of memory is overhead that we don't want to pay in more common cases where we want to execute code that doesn't capture its environment, use functions for this case.  
This is done in 3 ways, which directly map to the three ways a function can take a parameter; take ownership, borrowing mutably, and borrowing immutably.  
These are encoded in the three `Fn` traits as follows:  
* `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure's environment. To consume the captured variables, the closure must take ownership of these variables  and move them into the closure when it is defined. The `Once` means the closure can't take ownership of the same variables more than once.  
* `FnMut` can change the environment because it mutably borrows values.  
* `Fn` borrows values from the environment immutably.  

# Processing a Series of Items with Iterators
The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.  
In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up. Once we've created an iterator, we can use it in a variety of ways.  

## The `Iterator` Trait and the `next` Method
All iterators implement a trait named `Iterator` that is defined in the standard library.  
The `Iterator` trait only requires implementors to define one method: the `next` method, which returns one item of the iterator at a time wrapped in `Some` and, when the iteration is over, returns `None`.  
The iterator must be mutable to call the `next` method, since calling the method changes the internal state of the iterator.  
But for a `for` loop, we took the ownership of the iterator and made it mutable behind the scenes.  
The `iter` method produces immutable references of a vector. If we want to take ownership and return the values, use `into_iter` method.  
If we want to create mutable reference, we can call `iter_mut` instead of `iter`.  

## Methods that Consume the Iterator
The `Iterator` trait has a number of different methods with default implementations provided by the standard library.  
Some of these methods call the `next` method in their definition. Methods that call `next` are called consuming adaptors, because calling them uses up the iterator.  

## Methods that Produce Other Iterators
Other methods defined on the `Iterator` trait, known as Iterator adaptors, allow you to change iterators into different kinds of iterators.  
We need to consume the iterators because they are lazy.  

## Using Closures that Capture Their Environment
Filter Demo

## Zip method
Use `zip` method to pair an iterator with another.  

# Comparing Performance: Loops vs. Iterators
Iterators are faster than `for` loops. Iterators, although high level of abstraction, get compiled down to roughly the same code as if you'd written the lower-level code yourself.  
They are one of the Rust's zero cost abstractions, by which we mean using the abstraction imposes no additional runtime overhead.  
Unrolling is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop.