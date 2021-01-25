# More About Cargo and Cargo.io
## Customizing Builds with Release Profiles
Cargo has two main profiles: the `dev` profile Cargo uses when you run `cargo build` and the `release` profile Cargo uses when you run `cargo build --release`.  
The `dev` profile is defined with good defaults for development, and the `release` profile is defined with good defaults for release builds.  
Cargo has default settings for each of the profiles that apply when there aren't any `[profile.*]` sections in the project's Cargo.toml file.  
```toml
[profile.dev]
opt-level = 1

[profile.release]
opt-level = 3
```  
The `opt-level` setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3.  

## Publishing a Crate to Crates.io
Crates.io is a open source crate registry.  

### Documentation Comments
Documentation comments start with `///` and support Markdown.  
```rust
/// Takes a u32 and returns its square
///
/// # Example
/// ```rust
/// let input_num: u32 = 2;
/// let input_num_square: u32 = my_crate::compute_square(input_num);
/// ```
pub fn compute_square(input_num: u32) -> u32 {
    input_num * input_num
}
```  
We can generate HTML from this documentation comment using `cargo doc`. This command runs the `rustdoc` tool distributed with Rust and puts the generated HTML documentation in the target/doc directory.  
Run `cargo doc --open` to build the HTML and open the result in a web browser.  


### Commonly Used Sections
Commonly used sections in the documentation by the crate authors are Panics, Errors, Safety.  

### Testing Documentation Comments
Running `cargo test` will also run the code inside the documentation comments.  

### Setting Up a Crates.io Account
After signing up in the crates.io website, get the API token and login from the command line using `cargo login <INSERT_API_KEY_HERE>`

### Adding Metadata to a New Crate
Add the metadata of a package like name, license, edition, description etc. to the `[package]` section of `Cargo.toml`.  

### Publishing to Crates.io
A crate publish is permanent. The version can never be overwritten, and the code cannot be deleted. There is no limit to the number of crate versions you can publish.  
Use `cargo publish` to publish the crate to crates.io.  

### Publishing a New Version of an Existing Crate
Use semantic versioning rules and update the `version` value in Cargo.toml file. Then just `cargo publish` again.  

### Removing Versions from Crates.io with `cargo yank`
You can't remove previous versions of a crate, you can prevent any future projects from adding them as a dependency. This is useful when a crate version is broken for one reason or another.  
Cargo supports yanking a crate version. Yanking a version prevents new projects from starting to depend on that version while allowing all existing projects that depend on it to continue to download and depend on that version.  
To yank a version of crate, run `cargo yank --vers <INSERT_VERSION_NUM_HERE>`.  
To undo yanking, run `cargo yank --vers <INSERT_VERSION_NUM_HERE> undo`.  
A yank does not delete any code.  

## Cargo Workspaces
A workspace is a set of packages that share the same Cargo.lock and output directory.  
The root Cargo.toml won't have a `[package]` section. Instead it will start with a `[workspace]` section that will allow us add members to the workspace by specifying the path to the package with our binary crate.  
The workspace has one target directory at the top level for compiled artifacts to be placed into, the inner packages won't have their own output directories.  
Even if we run `cargo build` inside the inner packages, the compiled artifacts will still end up in the root level target directory.  
If a package x depends on package y in a workspace, add y's path to the dependencies in x's Cargo.toml.  
To run a binary crate from the workspace, use `cargo run -p <INSERT_CRATE_NAME_HERE>`.  
If the x package depends on an external crate, add it to its `[dependencies]` section in Cargo.toml. The workspace Cargo.lock keeps track of the dependency and every crate in the workspace can use the same dependency and same version.  
If the y package also wants to use the external crate downloaded by x package, just mention it in y's `[dependencies]` section in Cargo.toml and use it normally.  

## Installing Binaries from Crates.io with `cargo install`
The `cargo install` command allows you to install and use binary crates locally. You can only install packages that have binary targets.  
All binaries installed with `cargo install` are stored in the installation root's bin folder.