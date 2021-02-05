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

# Treating Smart Pointers Like Regular References with the `Deref` Trait
By implementing `Deref` in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.  

## Following the Pointer to the Value with the Dereference Operator
A regular reference is a type of pointer, and one way to think of a pointer is an arrow to a value stored somewhere else.  
For example, if we create a reference to an `i32` value and then use the dereference operator to follow the reference to the data.  

## Using `Box<T>` Like a Reference
We can dereference a `Box<T>` just like a regular reference.

## Defining Our Own Smart Pointer
The `Box<T>` type is ultimately defined as a tuple struct with one element. We'll also define a `new` function to match the `new` function defined on `Box<T>`.  
Our `MyBox<T>` can't be dereferenced because we haven't implemented that ability on our type. To enable dereferencing with the `*` operator, we implement the `Deref` trait.  

## Treating a Type Like a Reference by Implementing the `Deref` Trait
The `Deref` trait, provided by the standard library, requires us to implement one method named `deref` that borrows `&self` and returns the inner data.  
The `type Target = T` defines an associated type for the `Deref` trait to use. Associated types are a slightly different way of declaring a generic parameter.  
The `deref` method gives the compiler the ability to take a value of any type that implements `Deref` and call the `deref` method to get a `&` reference that it knows how to dereference. So, `*y` turns to `*(y.deref())`.  

## Implicit Deref Coercions with Functions and Methods
Deref coercion is a convenience that Rust performs on arguments to functions and methods. It works only on types that implement the `Deref` trait.  
Deref coercion converts such a type into a reference to another type. For example, deref coercion can convert `&String` to `&str` because `String` implements the `Deref` trait such that it returns `str`.  

# Running Code on Cleanup with the `Drop` Trait
The `Drop` trait lets you customize what happens when a value is about to go out of scope.  
Specify the code to run when a value goes out of scope by implementing the `Drop` trait.  
The `Drop` trait requires you to implement one method named `drop` that takes a mutable reference to self.  
The `Drop` trait is included in the prelude, so we don't need to bring it into the scope.  
Variables are dropped in the reverse order of their creation.  

## Dropping a Value Early with `std::mem::drop`
It's not straightforward to disable the automatic `drop` functionality. The whole point of `Drop` trait is that it's taken care of automatically.  
Occasionally, you might want to clean up a value early. One example is when using smart pointers that manage locks: you might want to force the `drop` method that releases the lock so that other code in the same scope can acquire the lock.  
Rust doesn't let us call `drop` explicitly because Rust would still automatically call `drop` on the value at the end of its scope.  
So, if we need to force a value to be cleaned up early, we can use the `std::mem::drop` function.  
The `std::mem::drop` function is different from the `drop` method in the `Drop` trait. We call it by passing the value we want to force to be dropped early as an argument. The function is in the prelude.  

# `Rc<T>` the Reference Counted Smart Pointer
There are cases when a single value might have multiple owners. For example, in graph data structures, multiple edges point to the same node, and that node is conceptually owned by all the edges that point to it.  
A node shouldn't be cleaned up unless it doesn't have any edges pointing to it.  
To enable multiple ownership, Rust has a type called `Rc<T>`, which is an abbreviation for reference counting.  
The `Rc<T>` type keeps track of number of references to a value which determines whether a value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.  
We use the `Rc<T>` type when we want to allocate some data on heap for multiple parts of our program to read, and we can't determine at compile time which part will finish using data last.  
`Rc<T>` is only for use in single-threaded scenarios.  

## Using `Rc<T>` to Share Data
`Rc::clone` doesn't make deep copies, it only increments the reference count, which doesn't take much time.  

## Cloning an `Rc<T>` Increases the Reference Count
By calling the `Rc::strong_count` function, we can get the reference count of a value.  
The implementation of `Drop` trait decreases the reference count automatically when an `Rc<T>` value goes out of scope.  
Via immutable references, `Rc<T>` allows you to share data between multiple parts of your program for reading only.

# `RefCell<T>` and the Interior Mutability Pattern
Interior mutability is a design pattern in Rust that allows you to mutate data even when there are no Immutable references to that data; normally this action is disallowed by the borrowing rules.  
To mutate the data, the pattern uses `unsafe` code inside a data structure to bend Rust's usual rules that govern mutation and borrowing.  
The `unsafe` code involved is then wrapped in a safe API, and the outer type is still immutable.  

## Enforcing Borrowing Rules at Runtime with `RefCell<T>`
Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data it holds.  
With references and `Box<T>`, the borrowing rules' invariants are enforced at compile time. With `RefCell<T>`, these invariants are enforced at runtime.  
If you break these rules, with `RefCell<T>`, your program will panic and exit.  
The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, whereas they are disallowed by the compile-time checks.  
The `RefCell<T>` type is useful when you're sure your code follows the borrowing rules, but the compiler is unable to understand and guarantee that.  
Similar to `Rc<T>`, `RefCell<T>` is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.  

## Interior Mutability: A Mutable Borrow to an Immutable Value
A consequence of borrowing rules is that when you have an immutable value, you can't borrow it mutably.  
However, there are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code.  
Using `RefCell<T>` is one way to get the ability to have interior mutability.  
The borrowing rules in the compiler allows this interior mutability, and borrowing rules are checked at runtime instead.  
If you violate these rules, you'll get a `panic!` instead of a compile-time error.  

## Mock Objects
Use `RefCell::new` to make an immutable interior mutable. To get a mutable reference, use the `borrow_mut` and `borrow` to get an immutable reference.  

## Keeping Track of Borrows at Runtime with `RefCell<T>`
The `borrow` method returns the smart pointer type `Ref<T>`, and `borrow_mut` returns the smart pointer type `RefMut<T>`.  
Both types implement `Deref`, so we can treat them like regular references.  
The `RefCell<T>`  keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active.  
Every time we call `borrow`, the `RefCell<T>` increases its count of how many immutable borrows are active.  
When a `Ref<T>` value goes out of scope, the count of immutable goes down by a one.  
`RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time.  

## Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`
If you have an `Rc<T>` that holds `RefCell<T>`, you can get a value that can have multiple owners and that you can mutate.  
The standard library has other types that provide interior mutability, such as `Cell<T>`, which is similar except that instead of giving references to the inner value, the value is copied in and out of `Cell<T>`.  
There's also `Mutex<T>`, which offers interior mutability that's safe to use across threads.