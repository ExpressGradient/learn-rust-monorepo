# Object-Oriented Programming Features of Rust
Many competing definitions describe what OOP is; some definitions would classify Rust as object-oriented, but other definitions would not.  

# Characteristics of Object-Oriented Languages
Rust is influenced by many programming paradigms, including OOP and Functional Programming.  
OOP languages share certain common characteristics, namely objects, encapsulation, and inheritance.  

## Objects Contain Data and Behavior
Object-oriented programs are made up of objects. An object packages both data, and the procedures that operate on that data. The procedures are typically called methods or operations.  
Using this definition, Rust is object-oriented: `struct` and `enum` have data, and `impl` block provide method on `struct` and `enum`.  

## Encapsulation that Hides Implementation Details
Encapsulation means that the implementation details of an object aren't accessible to code using that object.  
The only way to interact with an object is through its public API; the code using the object shouldn't be able to reach into the object's internals and change data or behavior directly.  
We can use the `pub` keyword to decide which modules, types, functions and methods in our code should be public, and by default everything is private.  
If encapsulation is a required aspect for a language to be considered object-oriented, then Rust meets that requirement.  

## Inheritance as a Type System and as Code Sharing
Inheritance is a mechanism whereby an object can inherit from another object's definition, thus gaining parent object's data and behavior without us having to define them again.  
If a language must have inheritance to be an object-oriented language then Rust is not one. There is no way to define a struct that inherits the parent struct fields and implementation methods.  
We choose inheritance for two main reasons. One is for a reuse of code: you can implement particular behavior for one type, and inheritance enables you to use reuse that implementation for a different type.  
We can share Rust code using default trait method implementations instead.  
The other reason to use inheritance relates to the type system: to enable a child type to be used in the same places as the parent type.  
This is also called polymorphism, which means that you can substitute multiple objects for each other at runtime if they share certain characteristics.  
Rust uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is sometimes called bounded parametric polymorphism.  
Inheritance has recently fallen out of favor as a programming design solution in many programming languages because it's often at risk of sharing more code than necessary.  
In addition, some languages will only allow a subclass to inherit from one class, further restricting the flexibility of a program's design.

# Using Trait Objects That Allow for Values of Different Types
## Defining a Trait for Common Behavior
A trait object points to both an instance of a type implementing our specified trait, and a table used to lookup trait methods on that type at runtime.  
Trait objects are more like objects in other languages in the sense that they combine data and behavior.  
But trait objects differ from traditional objects in that we can't add data to a trait object.  
Trait objects aren't generally useful as objects in other languages; their specific purpose is to allow abstraction across common behavior.  
Duck typing in dynamically typed languages: if it walks like a duck and quacks like a duck, then it must be a duck.  

## Trait Objects Perform Dynamic Dispatch
Monomorphization does static dispatch, which is when compiler knows what method you're calling at compile time.  
This opposed to dynamic dispatch, which is when the compiler can't tell which method you're calling.  
In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.  
When we use trait objects, Rust must use dynamic dispatch.

## Object Safety Is Required for Trait Objects
You can only make object-safe traits into trait objects. Some complex rules govern all the properties that make a trait object safe, but in practice only two rules are relevant.  
A trait is object-safe if all the methods defined in the trait have the following properties:
* The return type isn't `Self`.
* There are no generic parameters.
The `Self` keyword is just an alias for the type we're implementing the traits or methods on.  
  
# Implementing an Object-Oriented Design Pattern
The State pattern is an object-oriented design pattern.  
The crux of the pattern is that a value has some internal state, which is represented by a set of state objects.  
And the value's behavior changes based on the internal state.  
Each state object is responsible for its own behavior and for governing when it should change into another state.  
The value that holds a state object knows nothing about the different behavior of the states or when to transition between states.  

## Trade-offs of the State Pattern
One downside of the state pattern is that, because the states implement the transitions between states, some states are coupled to each other.  
Another downside is that logic is duplicated.