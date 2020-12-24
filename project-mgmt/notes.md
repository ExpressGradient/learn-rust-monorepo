# Introduction
As a project grows, you can organize code by splitting into multiple modules and multiple files.  
A package can contain multiple binary crates and optionally one library crate.  

In addition to grouping functionality, encapsulating implementation details let you reuse the code at a higher level.  

Rust has a number of features that allow you to manage your code's organization, including which details are exposed, which details are private, and what names are in each scope in your programs.  
The features are known as the module system. They include:
* Packages: A Cargo feature that lets you build, test and share crates.
* Crates: A tree of modules that produces a library or executable.
* Modules and use: Let you control the organization, scope or privacy of paths.
* Paths: A way of naming an item, such as a struct, function or module. 

# Packages and Crates
A crate is a binary or library.  
The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.  
A package is one or more crates that provide a set of functionality.  
A package contains a `cargo.toml` that describes how to build those crates.  

When we create a project using `cargo new <project-name>` command, it creates a package, and in that project, there'll be a binary crate with same name as the package and `main.rs` is the crate root.  
If the package contains a `lib.rs`, then the compiler will know that, there is a library crate with same name as package name and `lib.rs` is the crate root.

A package can contain both library crate and binary crate.  
The rule is that the package must contain 0 or 1 library crate and 1 or many binary crates.  
All the other binary crates are placed in `src/bin`. Each file will be a seperate binary crate.

# Defining Modules to Control Scope and Privacy
Modules lets us organize code within a crate into groups for readability and reuse.  
Modules also control privacy of items.  
Crate root forms a `crate` module at the root of the module tree.

# Paths for referring to an Item in the Module Tree
A path can take two forms:
* An absolute path starts from a crate root by using a crate name or literal `crate`.
* A relative path starts from the current module and uses `self`, `super` or an identifier in the current module.  

Modules aren't useful only for organizing your code. They also define Rust's privacy boundary.  
All the items are private by default.  

## Exposing Paths with the `pub` keyword
Making a module `pub` doesn't make its contents public. The `pub` keyword on a module only let's code in its ancestor modules refer to it.

## Starting Relative Paths with `super`
`Super` keyword lets us go up the Path.

## Making Structs and Enums Public
If we use `pub` before a struct definition, we make the struct public, but the struct fields are still private.  
But if we make an enum public, all of its variants are public.

# Bringing Paths into Scope with the `use` keyword
If we have a function and that function's path is similar to this `module1::module2::function`, then we can bring that function into scope using the `use` keyword like this `use module1::module2::function` and we can normally access the function as if its in the local scope.  
Use keyword also checks privacy before bringing anything into scope.

## Creating Idiomatic `use` Paths
The idiomatic way to bring a function into scope is by bringing only its parent's module into scope, not the whole function.  
This helps us differentiate between local and other module functions.  
But this is different for structs and enums, the idiomatic way is to bring the whole path.  
There is no strong reason behind this idiom, it's just the convention that has emerged and everyone got used to reading it that way.
The exception to this idiom is if we're bringing two items with the same name into scope with `use` statements, because Rust doesn't allow that.

## Providing New Names with the `as` keyword
If we bring two items from two different modules having same name, Rust gives us as an error.  
A solution to this is, we can specify a new local name to any of the item using the `as` keyword.

## Re-exporting Names with `pub use`
When we bring a name into scope with the `use` keyword, the name available with the new scope is private.  
To enable the new scope to be called from other modules, we can use `pub use`.  
This technique is called re-exporting, because we're bringing an item into scope but also making that item available for others to bring into scope.

## Using Nested Paths to Clean Up Large `use` Lists
If we're using multiple items defined in the same module or same crate, listing each item on its own line can take up a lot of vertical space in our files.  
Instead, we can use nested paths to bring the same items into scope in one line.  

You can even use the `self` keyword while using `use`
```rust
use std::io::{self, Write};
```

## The Glob Operator
If we want to bring all public items defined in a path into scope, we can specify that path followed by `*`, the glob operator.

# Splitting Modules into Different Files
If we create a file named `admin.rs`, place it in `src` directory, and then define a module in the file called `finance`.  
We can load that module into `lib.rs` by doing 

```rust
mod admin;
pub use crate::admin::finance;
```

The semicolon instead of block tells that to load the modules instead of defining it.