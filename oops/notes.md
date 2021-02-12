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