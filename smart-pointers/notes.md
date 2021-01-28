# Smart Pointers
A pointer is a general concept for a variable that contains an address in memory. This address refers to, or "points at", some other data.  
Rust's references just borrow the value they point to, they don't have any special capabilities other than referring to data.  
Smart Pointers on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities.  
The concept of smart pointers isn't unique to Rust: smart pointers defined in the standard library provide functionality beyond that provided by references.  
The difference between references and smart pointers is that references are pointers that only borrow data; in contrast, in many cases, smart pointers own the data they point to.  
Both `String` and `Vec<T>` count as smart pointers because they have their own memory and allow you to manipulate it. They also have metadata and extra capabilities or guarantees.  
Smart pointers are usually implemented using structs. The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the `Deref` and `Drop` traits.  

# Using `Box<T>` to Point to Data on the Heap
The most straightforward smart pointer is a box, whose type is written `Box<T>`. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.  
You'll use boxes most often in these situations:
* When you have a type whose size can’t be known at compile time, and you want to use a value of that type in a context that requires an exact size.  
* When you have a large amount of data, and you want to transfer ownership but ensure the data won’t be copied when you do so.  
* When you want to own a value, and you care only that it’s a type that implements a particular trait rather than being of a specific type.  

## Using a `Box<T>` to Store Data on the Heap
Use `Box::new()` to have the value of a `Box` that points to a value allocated on the heap.  
When a box goes out of scope, it will be deallocated. De-allocation happens for the box(stored on the stack), and the data it points to (stored on the heap).  

## Enabling Recursive Types with Boxes
At compile time, Rust needs to know how much space a type takes up. One type whose size can't be known at compile time is a recursive type, where a value can have a part of itself another value of the same type.  
Because this nesting of value could theoretically continue infinitely, Rust doesn't know how much space a value of a recursive type needs.  
However, boxes have a know size, so by inserting a box in a recursive type definition, you can have recursive types.  

## Cons List
A cons list is a data structure that comes from the Lisp programming language and its dialects. In Lisp, the `cons` function (short for "construct function") constructs a new pair from its two arguments, which usually are a single value and another pair.  
These pairs containing pairs form a list. Each item in a cons list contains two elements: the value of the current item and the next item. The last item in the list contains only a value called `Nil` without a next item.  
The cons list isn't a commonly used data structure in Rust. Most of the time when you have a list of items in Rust, `Vec<T>` is a better choice to use.  
In the enum `List`, Rust can't figure out how much space it needs to store a `List` value.  

## Computing the Size of a Non-Recursive Type
The maximum space needed for an enum value is the space it would take to store the largest of its variants. Contrast this with what happens when Rust tries to determine how much space a recursive type like the `List` needs.  

## Using a `Box<T>` to Get a Recursive Type with a Known Size
Rust can't figure out how much space to allocate for recursively defined types, so the compiler gives an error, but the error includes a helpful suggestion to insert "indirection" meaning to store the value indirectly by storing a pointer to the value instead.  
Because a `Box<T>` is a pointer, Rust always knows how much space a `Box<T>` needs: a pointer's size doesn't change based on the amount of data it's pointing to.  
Boxes provide only the indirection and heap allocation; they don't have any other special capabilities. They also don't have performance overhead, so they can be useful in cases like the cons list where the indirection is the only feature we need.  
The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation.