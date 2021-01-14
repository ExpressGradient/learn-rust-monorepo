# Generic Types, Traits and Lifetimes
Generics is a tool for handling duplication of concepts. They are abstract stand-ins for concrete types or other properties.  
When we're writing code, we can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.  
Functions can take parameters of some generic type instead of a concrete type, like `i32` or `String`.  

## Removing Duplication by Extracting a Function
1. Identify duplicate code.
2. Extract the duplicate code into the body of the function and specify the inputs and return values of the code in the function signature.
3. Update the instances of duplicated code to call the function instead.

# Generic Data Types
We can use generics to create definitions for items like function signatures or structs, which we can then use with many and different concrete data types.  

## In Function Definitions
When defining a function that uses generics, we place the generics in the signature of the function.  
Doing so makes our code more flexible and provides more functionality to callers of our function while preventing code duplication.  
When we use a type parameter name in a function signature, we have to declare the type parameter name before we use it.  
To define a generic function, place type name declarations inside angle brackets, `<>`, between the name of the function and the parameter list.  
The function `largest` is generic over some type `T`.  
Generic Type Parameters are Camel Cased by convention.

## In Struct Definitions
We can also define structs to use a generic type parameter in one or more fields using `<>` syntax.  
The syntax for using generics in struct definitions is similar to that used in function definitions.  
We can even have more than one generic type. But using more than a few makes your code hard to read.  
When you need lots of generic types in your code, it could indicate that your code needs restructuring into smaller pieces.  

## In Enum Definitions
We can define enums to hold generic data types in their variants.  
Some enums with generic types are `Option<T>` and `Result<T, E>`.  

## In Method Definitions
We can implement methods on structs and enums and use generic types in their definitions too.  
We declare `T` just after `impl` so we can use it to specify that we're implementing methods on type `Point<T>`.  
We could implement methods only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type.  
Generic type parameters in a struct definition aren't always the same as those you use in that struct's method signatures.  

## Performance of Code Using Generics
Rust implements generics in such a way that your code doesn't run any slower using generic types than it would with concrete types.  
This is done by monomorphization of the code using generics at compile time.  
Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.  
The compiler does the opposite of the steps we used to create the generic function by extracting duplicate code and adding a generic type.  
Because of this process of monomorphization, we pay no runtime cost for using generics and also it is extremely efficient at runtime.

# Traits: Defining Shared Behavior
A trait tells the Rust compiler about functionality a particular type has and can share with other types.  
We can use traits to define shared behavior in an abstract way.  
We can use trait bounds to specify that a generic type can be of any type that has certain behavior.  

## Defining a Trait
A type's behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types.  
A trait contains only method signatures instead of complete methods.  
```rust
trait TraitName {
    fn function_name(&self) -> String;
}
```
Each type implementing this trait must provide its own custom behavior for the body of the method.  
A trait can have multiple methods in its body.  

## Implementing a Trait on a Type
If we have a type called TypeName, TraitName's implementation for that type would be,
```rust
impl TraitName for TypeName {
    fn function_name(&self) -> String {
        // method implementation
    }
}
```
The trait needs to be a public trait for another crate to implement it.  
We can implement a trait on a type only if either the trait, or the type is local to our crate. We can't implement external traits on external types.  
This restriction is part of a property of programs called coherence, and more specifically the orphan rule.  
This rule ensures that other people's code can't break your code and vice versa.  
Without the rule, two crates could implement the same trait for the same type, and Rust wouldn't know which implementation to use.  

## Default Implementations
Instead of ending the method signature with `;` you can go on and add implementation to it, which becomes the default behavior of that method.  
Then as we implement the trait on a particular type, we can keep or override each method's default behavior.  
Default implementations can call other methods in the same trait, even if those methods don't have a default implementation.

## Traits as Parameters
We can define functions to accept arguments, which types have certain trait implementations.  
```rust
fn function_name(arg: &impl TraitName) {}
```

## Trait Bound Syntax
The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a trait bound.  
```rust
fn function_name<T: TraitName>(arg: &T) {}
```
This longer form is equivalent to the previous section but is more verbose.  
The trait bound syntax can express more complexity in other cases. For example, we can have two parameters that implement a Trait.

## Specifying Multiple Traits Bounds with the `+` Syntax
We can also specify more than one trait bound using the `+` syntax.
```rust
fn function_name<T: Trait1 + Trait2>(arg: &T) {}
```

## Clearer Trait Bounds with `where` Clauses
Using too many trait bounds has its downsides. It makes function signature hard to read.  
Rust has alternate syntax for specifing the trait bounds inside a `where` clause after the function signature.

## Returning Types that Implement Traits
We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait.  
The ability to return a type that is only specified by the trait it implements is especially useful in the context of closures and iterators.  
However, you can only use `impl Trait` if you're returning a single type.

## Using Trait Bounds to Conditionally Implement Methods
By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement specific traits.  
We can also conditionally implement a trait for any type that implements another trait. These implementations are called blanket implementations and are extensively used in the standard library.  
For example,
```rust
impl<T: Display> ToString for T {
    // implementation...
}
```
We can call the `to_string` method defined by the `ToString` trait on any type that implements the `Display` trait.  
Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior.

# Validating References with Lifetimes
Every reference has a lifetime, which is the scope for which that reference is valid.  Most of the time, lifetimes are implicit and inferred, just like types are inferred.  
We must annotate lifetimes when the lifetimes of references could be related in a few different ways.  
Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.  

## Preventing Dangling References with Lifetimes
The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it's intended to reference.  

## The Borrow Checker
The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.

## Generic Lifetimes in Functions
If we implement the `longest` function, we get an error because Rust can't tell whether the reference being returned refers to `x` or `y`.  

## Lifetime Annotation Syntax
Lifetime annotations don't change how long any of the references live.  
Just as functions can accept any type when the signature specifies a generic lifetime parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.  
Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.  
One lifetime annotation by itself doesn't have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other.  

## Lifetime Annotation in Function Signatures
As with generic type parameters, we need to declare generic lifetime parameters inside angle brackets between the function name and the parameter list.  
The function signature now tells Rust that for some lifetime `'a`, the function takes two parameters, both of which are string slices that live atleast as long as lifetime `'a`.  
The return slice from the function will also live atleast as long as lifetime `'a` too.  

## Thinking in Terms of Lifetimes
When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.  
Lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.  
Once they're connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

## Lifetime Annotations in Struct Definitions
It's possible for structs to hold references, but in that case we need to add a lifetime annotation on every reference in the struct's definition.  

## Lifetime Elision
Lifetime elision rules are a set of particular cases that the compiler will consider, and if your code fits in these cases, you don't need to write the lifetimes explicitly.  
They are:
1. Each paramter that is a reference gets its own lifetime parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output parameters.

## The Static Lifetime
`'static` lifetime for a reference means, it can live for the entire duration of the program.  
All string literals have static lifetime because the text of the string is stored directly in the program's binary, which is always available.