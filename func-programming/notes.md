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