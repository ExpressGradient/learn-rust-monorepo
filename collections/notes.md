# Common Collections - Introduction
Collections are useful data structures in Rust which are stored on the heap, which means the amount of data does not need to be known while compile time.  
The data can grow and shrink as the program runs.  
Each kind of collection has different capabilities and cost.  
Three common collections include:
* Vector: It allows you to store a variable number of values next to each other.
* String: It is a collection of characters.
* Hash Map: It allows you to associate a value with a particular key.

# Storing Lists of Values with Vectors
`Vec<T>` Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.  
Vectors can only store values of same type.

## Creating a New Vector
To create a new, empty vector, we can call the `Vec::new` function.  
Vectors are implemented using generics since Rust doesn't know what kind of elements we intend to store.  
Rust also provides a macro `vec!` to create a `Vec<T>` that has initial values, and also it can automatically infer types.

## Updating a Vector
If any variable needs to change its value, it should be `mut`. Same goes with vectors.  
To push elements into the vector, use the `push` method.

## Dropping a Vector
Like any `Struct`, if a vector goes out of scope, it is freed.  
When vector gets dropped, all of its contents are dropped.

## Reading elements of Vectors
There are two ways of reading elements of a vector.  
1. By using the normal array access syntax, `&vector[3]` which gives reference of 3rd element of that vector.
2. By using the `get` method. This method returns `Option<&T>`.

If you use `[]` and access an element of invalid index, the code will panic.  
But if you use `get`, then it simply returns `None`.  
If you want your program to crash, use `[]`. If you want to handle things instead of crashing, use `get`.  

If you access an element, and then push an element to the vector, you get an error.  
Because, an immutable and a mutable borrow cannot exist in the same scope.  
Not only that, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space.  
If that is the case, then the immutable reference might be pointing to deallocated memory, so borrowing rules kick in to prevent this.  

## Iterating over the Values in a Vector
We can use a `for` loop to iterate over an immutable reference of a vector.  
In order to make changes to the elements, we need to iterate over a mutable reference of a vector.  
To change that the mutable reference refers to, we have to use the dereference operator `*`.

## Using an Enum to Store Multiple Types
Rust needs to know what types will be in the vector at compile time, so it knows exactly how much memory on the heap will be needed to store each element.  
If you don't know the exhaustive set of types the program will get at runtime store in a vector, the enum technique won't work.  
Instead, you can use a trait object.

# Storing UTF-8 Encoded Text with Strings
Rust has only one string type in the core language, which is the string slice `str` that is usually seen its borrowed form `&str`.  
String slices are references to some UTF-8 encoded string data stored elsewhere.  
The `String` type, which is provided by Rust's standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.  
Rust also includes a number of other string types, such as `OsString`, `OsStr` and others.  
Library crates can provide even more options for storing string data.

## Creating a New String
Many of the same operations available with `Vec<T>` are available with `String` as well.  
Use the `new` function to create a string.  
If any type has a `Display` trait, use `to_string()` method to convert it into `String`.  
We can also use the `String::new` function to create a new String.  
Strings are UTF-8 encoded, so we can include any properly encoded data in them.  

## Updating a String
A String can grow in size and its contents can change, just like the contents of a `Vec<T>`, if you push more data into it.  
You can use `+` operator or `format!` macro to concatenate String values.

### Appending to a String with `push_str` and `push`
We can grow a String by using the `push_str` method to append a string slice.  
`push_str` method takes a string slice because we don't want to take ownership of the parameter.  
The `push` method takes a single character as a parameter and adds it to the string.

### Concatenation with the `+` Operator or the `format!` macro
Use `+` to combine two strings. Its syntax is `s1 + &s2`.  
s1 will be no longer valid as we moved it and also we used the reference of s2. It is just the `add` method syntax.  
If you use `+` for combining strings, under the hood, it calls the `add` function,
```rust
fn add(self, s: &str) -> String {}
```  
Now even if we pass a `&String` to the add method, nothing happens.  
Because the compiler will coerce the `&String` to `&str`.  
When we call the `add` method, Rust uses a `deref coercion`, which turns a `&str` into `&str[..]`.  
`add` method also takes in `self` which is not a reference, so s1 will be moved.  
So the `add` method takes the ownership of s1, appends a copy of the contents s2, and then returns ownership of the result.  
This implementation is a lot better than copying.
`format!` macro works in the same of `println!` but it doesn't print output to the screen, just returns a String.  
Using `format!` in code looks clean and easier to read, and also it doesn't take any ownership of its arguments.  

## Indexing Into Strings
If you access parts of String by using Indexing syntax, you'll get an error.  

### Internal Representation
A String is wrapper over a `Vec<u8>`. Each letter takes a byte when encoded in UTF-8.  
So, if you access 0th part of `"hello"`, you'd get 104, not h.  

### Bytes, Scalars and Grapheme Clusters
There are three relevant ways to look at Strings from Rust's perspective: bytes, scalar values and grapheme clusters(letters).  
Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs, no matter what human language the data is in.  
The final reason Rust doesn't allow us to index into a String to get a character is that indexing operations are expected to always take constant time `O(1)`.  
But it isn't possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.  

## Slicing Strings
Indexing into a string is often a bad idea because it's not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice.  
To be more specific in your indexing and indicate that you want a string slice, rather than using `[]` with a single number, you can use `[]` with a range to create a string slice containing particular bytes.  
You should use ranges to create string slices with caution, because doing so can crash your program.  

## Methods for Iterating Over Strings
Use `chars` method if you need to perform operations on individual Unicode Scalar Values and `bytes` method for operations on each raw byte.  
Getting grapheme clusters from string is complex, use an external crate for that.

# Storing Keys with Associated Values in Hash Maps
The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`.  
This is done using a hashing function which determines how it places these keys and values in memory.  
Hash Maps are useful when you want to look up data not by using an index, but by using a key that can be of any type.  

## Creating a New Hash Map
You can create an empty hash map with `new` method and add elements to it using the `insert` method.  
Hash Map is the least used of the three common collections, so it is not included in the items brought into scope automatically in the prelude.  
Hash Maps also have less support from the standard library, there's no built-in macro to construct them.  
Hash Map data is also stored on the heap. Its keys must be of same type and so is its values too.  

## Hash Maps and Ownership
For types that implement `Copy` trait, values are copied into Hash Map.  
For owned values like `String`, the values will be moved and Hash Map will be the owner of those values.  

## Accessing Values in a Hash Map
Use the `get` method to access values from a Hash Map. This method returns an `Option<&V>`. We should be handling the `Option` using either `match` or `if let`.  
We can even iterate over each key/value pair in a Hash Map using a `for` loop.  

## Updating a Hash Map
### Overwriting a Value
Using `insert` method on a key already having a Value overwrites it.  

### Only Inserting a Value If the Key Has No Value
Hash Map API has a method called `entry` which returns an `Entry` enum, this represents a value that might or might not exist.  
The `or_insert` method on `Entry` is defined to return a mutable reference to the Value for the corresponding `Entry` key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.  

### Updating a Value Based on the Old Value
The `entry` method returns a mutable reference `&mut V`.  Use it to update the value.  

## Hashing Functions
By default `Hash Map` uses a cryptographically strong hashing function that can provide resistance to DoS attacks.  
This not the fastest algorithm, but the trade-off for better security that comes with the drop in performance is worth it.  
If you find the default Hash Function is slow, you can switch to another Function by specifying a different Hasher.  
Implement your own Hasher or lookup crates.io for other Hashers.