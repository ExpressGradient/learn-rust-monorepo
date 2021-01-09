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