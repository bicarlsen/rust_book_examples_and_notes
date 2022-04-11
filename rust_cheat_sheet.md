# Rust Cheat Sheet

## Tooling
+ Update Rust: `rustup update`
+ Complile: rustc `<filename>`
+ Build local documentation: `cargo doc --open`
+ **rustfmt**
  + Rust linter
  + Install with `rustup component add rust fmt`
  + run with `cargo fmt`
+ **rustfix**
  + Auto-fixes compiler warnings where possible.
  + Run with `cargo fix`.
+ **Clippy**
  + Collecton of code analyzers.
  + Install with `rustup component add clippy`.
  + Run with `cargo clippy`.
+ IDE integration with `rust-analyzer`.

## Cargo projects
+ **Commands**
  + New Cargo project: `cargo new <name>`
  + Create a new library project: `cargo new <name> --lib`
  + Check compilation: `cargo check`
  + Compile dev build: `cargo build`
  + Compile and run: `cargo run`
  + Compile production build: `cargo build --release`
  + Update dependencies: `cargo update`
    + Only updates packages within SemVer specifications.
  + Run tests: `cargo test`
  + Login to crates.io: `cargo login <api token>`
  + Publish a crate: `cargo publish`
  + Install a crate: `cargo install <package name>`
    + `cargo` can be extended by installing binary crates.
+ **Profiles**
  + A profile specifies a specific package configuration.
  + There are two default profiles: `dev` and `release`.
  + Specified in the `Cargo.toml` `[profile.<profile name>]` sections.
+ **Documentation**
  + Uses `///` for external docs and `//!` nested docs.
    + Nested docs are usually only used at the root level of the crate or a module.
  + Support markdown.
  + Build with `cargo doc`.
  + Common sections are `# Examples`, `# Panics`, `# Errors`, and `# Safety`.
  + Cargo teset runs code blocks in documentation.
+ Use `pub use` to re-export resources.
+ `cargo yank` indicates no new projects should consume a specific version of a package.
  + `cargo yan --vers <x.y.z>`
  + Use `--undo` flag to undo a `yank`.
+ **Workspaces**
  + Bundles multiple related packages.
  + Contained in same folder with a shared root `Cargo.toml` and `Cargo.lock`.
  + Identified with the `[workspace]` title section.
  + Specify packages in `members = [ ... ]`.
  + Path dependencies of packages within the same workspace must be added in the packages `Cargo.toml` `[dependencies]` section.
    + `<dep package name> = { path = "../dep_package" }`
  + Specify package to run with `-p` flag.
    + `cargo run -p <package name>`
    + `cargo test -p <package name>`
  + External dependencies must be added to each packages `Cargo.toml` file if it is consumed.
  + Crates published on crates.io need to be published separately. 

## Rust
+ Printing
  + `pritnln!` macro
    + `{:?}` prints an object using debug mode.
    + `{:#?}` pretty prints an object using debug mode.
  + `dbg!` marco
    + Take ownership of an object, prints the line number and debug output of the expression to `stderr`, and returns ownership.
+ Variables are immutable by default.
  + Use `mut` keyword for mutable variables.
+ `if { ... } else if { ... } else { ... }`
  + Conditions must be `bool`.
+ Function arguments are always evaluated before calling the function.
  + i.e. In `my_fcn( get_arg_fcn() )` `get_arg_fcn` will be run before running, or evaluating whether to run `my_fcn`.

### Feautres

+ **Loops:**
  + `loop { ... }`
    + `'<loop_name>: loop { ... }`
  + `while <condition> { ... }`
  + `for <var> in <obj> { ... }`
+ **Range:**
  + Exclusive end: `<start_index>..<end_index>`
  + Inclusive end: `<start_index>..=<end_index>`
  + If `<start_index>` is excluded it is assumed to be 0.
  + If `<end_index>` is excluded it is assumed to be the length of the object.
+ Reference: `&<var>`.
  + Used to borrow values instead of take owenership.
  + Dereference: `*`
+ String slice: `&s[ <start_index>..<end_index> ]`
  + Have type `&str`
  + A string reference `&String` is equivalent to the slice of the whole string `&string[ .. ]`.
  + String literals have the types of string slices `&str`.

### Structs + Enums

+ **Structs:**
  + Defined with keyword `struct`.
  + Tuple structs are defined as `struct <Name>( [ <field types>... ] )`.
    + Fields accessed with dot notation as tuples, `x.2`.
  + Unit structs defined as `struct <Name>;`.
  + Methods are implemented inside and `impl` block.
    + First argmument must be self, which can take ownership `self`, borrow immutably `&self`, or borrow mutably `&mut self`.
      + Taking ownership is rare, and usually only used when transforming the type of the object as to invalidate the original variable.
  + Associated functions are implemented in the `impl` block without a `self` argument.
    + An instance of the strcut is not required to use an associated function. 
      + Similar to class or static methods.
    + Run as `<Struct name>::<fn name>( [args...] )`

+ **Enums:**
  + Defined with `enum` keyword.
  + Accessed with `::` notation.
  + Variants can have associated data.
    + `enum <Name> { <Variant>(<Data type>)[, ...] }`
    + `<Name>::<Variant>( <Data value>[ , ...] )`
    + Variants can be used as functions that accept data arguments.
  + Can define methods on enum types using `impl`.
  + `Option` enum represents a value that could be something or nothing.
    + Has `None` and `Some( <T> )` variants.
+ `match`
  + Similar to `switch` statements.
  + `match <value> { <pattern> => { <action> }, ... }`
  + Pattern can bind to values for use in the code execution using `<pattern>( <parameter name> )`.
  + Matches must be exhaustive.
  + Can use a catch-all arm by excluding pattern and binding to value or using the wildcard `_`
    + `match <value> { ..., <variable name> => { <action> } }`
    + `match <value> { ..., _ => { <action> } }`
    + Can ignore unspecified values using a unit tuple like `match <value> { ..., _ => () }`.
+ `if let` acts like `match` with only one specified arm, ignoring the rest.
  + `if let <pattern> = <value expression> { ... }`
  + Can include an `else` statement.
+ **Collections**
  + **Vector** stores a variable number of values.
    + `let v: Vec<i32> = Vec::new();`
    + All elements are of the same type.
      + Can use an enum to group types.
    + `elm: &<T> = v[ <index> ];`
    + `elm: Option<&T> = v.get( <index> );`
  + **String**
  + **Hashmap** is a key-value store.
    + Hashmaps take ownership of the variables used as keys and values.
    + `insert` overwrites values if they exist.
    + `entry` allows you to check if a key already exists in the hashmap.
      + Retuns an `Entry` object that has the `or_insert` method on it.

### Errors

+ Two classes of errors: _recoverable_ and _unrecoverable_.
+ Recoverable error use the type `Result<T, E>`.
  + Use `match` on variants `Ok` and `Err` to distinguish between success and error result.
+ Unrecoverable error call the `panic!` macro.
+ `unwrap` and `expect` return the result of a `Result` if it is `Ok`, otherwise calls `panic!`.
+ `?` is used for error propogation.
  + If the result is `Ok` the value inside the `Ok` is returned.
  + If the result is `Err` the `Err` will be returned as the function's return value.
    + Error is automatically converted to type specified as the function's.
  + Can be used with `Option` types as well.
+ `main` can return integers (0 for normal, or error code) or `Result<(), E>`.

### Generics

+ Functions: `fn my_fcn<T>( p1: T[, ...] )`
+ Structs: `struct MyStruct<T> {x: T[, ...] }`
+ Methods: `impl <T[, U ...]> MyStruct<T[, U ...]>`
  + Can restrict methods to certain types by specifying them in the definition. `impl MyStruct<i32>`
+ Specify default concrete type using `<PlaceholderType=ConcreteType>`.

### Traits

+ Describes a certain behavior. Similar to implementing an interface.
  + Specifies functions and call signatures.
+ Declared using the `trait` keyword.
+ Implemented using `impl <Trait> for <Struct>`.
+ To implement a trait on a type at lleast th trait or the type must be locally defined.
+ Can include default implementations.
  + `impl` block must still be defined to use default implmentations, but can be empty.
+ Specify that a function argument or return type should implement a trait using `impl <Trait>` type.
  + `fn my_fcn( item &impl TraitOne ) -> impl TraitTwo { ... }`
  + Returning a trait only works if a single type is being returned.
  + Can also use _trait bound_ with generics as `fn my_fcn<T: MyTrait>( item: &T )`.
  + Use `+` to union traits as `fn my_fcn( &(impl TraitOne + TraitTwo ) )` or `fn my_fcn<T: TraitOne + TraitTwo>( item: &T )`.
  + Can also use `where` clause after the function signature as ` fn my_fcn<T, U>( t: &T, u: &U ) -> i32 where T: TraitOne + TraitTwo, U: TraitTwo + TraitThree { ... }`.
  + Use `Self` type as a type alias.
+ _Associated types_ connect a type placeholder with a trait so it can be used in the trait's methods' definition.
  + Identified with the `type` keyword and used by `Self::<type name>`.
+ **Supertraits** allow trait dependency.
  + Specified with `MyTrait: DependentTrait`
+ **newtype**
  + Allows implementation of a trait on a type that is not local to the current crate.
  + Works by using a thin wrapper of a tuple struct around the type which the trait is to befined on.
  + Can implement `Defref` on the wrapper to proxy calls on the wrapper type to the original type.
  + e.g. `struct Wrapper( ExternalType )`
+ **Trait object** points to an object that implements a trait.
  + Uses the `dyn` keyword to specify the required trait.
  + As opposed to a generic type which can only be replaced with one concrete type at a time, the trait object allows for multiple concrete types.
  + Allows behaviros similar to duck typing in dynamically typed languages, while still allowing for type checking at compile time.
  + Only _object-safe_ traits can be made into trait objects.
    + A trait is object safe if noe of it's methods
      + Return `Self`.
      + Use generic types.

### Lifetimes

+ Used to describe how lifetimes relate to each other when using references to ensure the reference lasts long enough to be used.
  + Mostly needed to connect the lifetime of borrowed input argument to the return value if it contains a value from a borrowed input argument.
+ Lifetime parameters start with an apostrophe `'` and are placed after the `&` of a reference.
  + Only used in function signature, not in function body.
  + Lifetimes on arguments are called _input parameters_, lifetimes on return values are called _output lifetimes_.
+ Can use generic lifetimes by defining them inside angle brackets `<>`.
  + `fn<'a>( &'a elm ) -> &'a i32 { ... }`
  + `impl<'a> MyStruct<'a> { ... }
+ When returning references, the lifetime of the return typ emust match the lifetime of one of the parameters.
+ **Elision rules** are built-in lifetime rules to reduce explicit annotation. If lifetimes can be determined from these rules no explicit annotation is required.
  1. Each reference argument gets its own litetime parameter.
  2. If there is exactly one input lifetime parameter it is assigned to all output lifetimes parameters.
  3. For a method, if there are multiple input lifetimes and one of them is `&self` or `&mut self` the lifetime of self is assigned to all output lifetime parameters.
+ The `'static` keyword indicates the reference can live for the entire duration of the program.
+ Lifetime annotations and generic types go in the same list.
  + `fn my_fcn<'a, T>( x: &'a str, y: T ) -> &'a str { ... }`

### Closures
+ Allows a function call with context to be stored for later execution.
  + Similar to `lambda` functions.
+ `let closure_fn = |<params>| { ... };`
  + Called like a function `closure_fn( <args> )`
+ Type annotations are usually not required but can still be included jsut as they would be for a function.
+ Closure types are inferred from its first call and locked in.
+ Every closure has its own type, even it all the type parameters of the call signature are identical.
+ **Memoization:** Closures can be used in a struct to allow for memoization.
  + Use generics and trait bounds for specifying the type.
  + All closures implement at least one of the trait bounds `Fn`, `FnMut`, `FnOnce`.
    + e.g. `Fn( u32 ) -> u32`
+ Closures can capture their environment in three traits:
  + `FnOnce` consumes the variables it captures by taking ownership.
  + `FnMut` borrows values mutably.
  + `Fn` borrows values immutably.
  + The trait is inferred based on how each value is used.
  + The `move` keyword forces the closure to take ownership of the values.
    + Mostly used when moving a closure to a new thread.
    + `let clsr = move |x| x + 1;`

### Iterators

+ Implement the `Iterator` trait.
  + Has a single type `Item`.
  + Has a single function `next( &mut self ) -> Option<Self::Item>;`.
    + Returns the item value wrapped in `Some`, or `None` to kill the iterator.
    + Item values are immutable references.
    + Calling next changes the internal state of the iterator.
+ Therea are three standard call to create an iterator.
  + `iter` returns immutable references to values.
  + `iter_mut` returns mutable references to the values.
  + `into_iter` takes ownership of the values.
+ Methods that call next are called _consuming adaptotrs_.
+ _Iterator adaptors_ changes an interator into a different type of iterator.
  + e.g. `map`, `filter`, `zip`.
+ Iterators are lazy, menaing no action is taken until a consuming adaptor attempts to consume the iterator.
  + e.g. `collect`, `for ... in`, `sum`.

### Smart pointers

+ Pointers with metadata and enhanced functionality.
  + Can own data.
  + e.g. `String`, `Vec<T>`, `Box<T>`, `Rc<T>`, `Ref<T>`, `RefMut<T>`, `RefCell<T>`.
+ Usually implmented via `struct`s.
  + Implements the `Deref` and `Drop` traits.
+ **Deref** and **DerefMut** traits
  + Customizes the behavior the dereference operator `*`.
  + Implements the `deref( &self ) -> &Self::Target` method.
  + _Deref coercion_ converts one reference type into another.
    + From `&T` to `&U` when `T: Deref<Target = U>`.
    + From `&mut T` to `&mut U` when `T: DerefMut<Target = U>`.
    + From `&mut T` to `&U` when `T: Deref<Target = U>`.
+ **Drop** trait
  + Determines what happens when a value goes out of scope.
  + Implements `drop( &mut self )`.
  + To drop a value early must use the `std::me::drop` function.
    + `drop( <variable> );`
+ **Box<T>**
  + Stores data on the heap.
  + Good for:
    + If you don't know the size of the data at compile time.
    + Need to transfer ownership of a large amount of data without copying it.
    + Need to own a value with a particular trait, but don't care about the type.
      + Known as a _trait object_.
+ **Rc<T>**
    + Reference counter allows for multiple immutable ownership.
      + Use `Rc::clone( <variable> )` to create new owner.
    + Useful for allocating data on the heap with multiple readers, without knowledge at compile time of which owner will end last.
    + Only for use in single-threses scenarios.
    + Can get reference count with `Rc::strong_count( <variable> )` function.
    + If `T` is a `RefCell` the value can have multiple owners and be mutable.
    + Can create a _weak reference_ using `Rc::downgrade( <variable> )`.
        + Creates a borrowed reference.
        + Must check the reference is still valid before using by calling the `upgrade` method on a `Weak<T>` object.
          + Returns an `Option<Rc<T>>`.
+ **RefCell<T>**
  + Allows _interior mutability_ which allows data mutation even when there are immutable references to the data.
  + Enforces borrowing rules at runtime instead of compile time.
    + If rule is broken at runtime the program will panic.
  + Useful if as a programmer you can ensure the program is correct, even if the compiler can't.
  + Only for use in single-threaded scenarios.
  + Use values with `borrow()` and `borrow_mut()` methods.
    + Return objects of type `Ref<T>` and `RedMut<T>`, respectively.
      + Both implement the `Deref` trait.
    + Must obey borrow rules at runtime.

### Threading

  + Rust uses a 1:1 threading model.
    + Green threading libraries exist for M:N threading.
  + `thread::spawn` creates a new thread, accepting a closure.
    + Returns an owned `JoinHandle` object.
      + `join` method used to wait for thread to complete.
  + Child threads are terminated when the parent thread exists regardless of whether they are complete.
  + **Message passing** accomplished using channels.
    + Channels are multiple producer, single consumer (mpsc).
    + Transmitter has `send` method.
      + Multiple producers created by `clone`ing the transmitter.
    + Receiver has `recv` and `try_recv` methods.
    + Can iterate over a receiver to receive messages until channel is closed.
  + **Mutex**
    + Call `lock` to acquire the mutex.
      + Returns a smart pointer of type `MutexGuard` wrapped in a `LockResult`.
      + `MutexGuard` automatically unlocks the mutex when it drops out of scope.
    + Mutex provides _interior mutability_.
    + Can create deadlocks.
  + Often need to use **atomic** version of primatives for thread safety.
    + `Rc` -> `Arc`
  + `Send` and `Sync` traits
    + `Send` indicates ownership can be sent across threads.
    + Almost all types in Rust are `Send`.
    + If a type is composed of parts that all implement `Send` it is automatically marked `Send`.
  + `Sync` indiates that the objecct can be referenced across threads.
    + Any type is `Sync` if an immutable reference to it is `Send`.
    + All primataives are `Sync`.
    + Any type who has all components `Sync` is also `Sync`.

### Patterns

+ `match`
  + Must be exhaustive.
  + Use underscore `_` to match anything.
  + Can match multiple patterns in an arm using the pipe `|`.
  + Match guard can be used to impose additional conditions.
    + Match guard precedence is lower than the pipe operator `|`.
    + e.g. `match x { Some( y ) if y < 5 => { ... }, ... }`
+ `if let`
  + `if let`, `else if let`, `else let`.
  + Can be mixed with normal `if` statements.
  + Variables in the pattern are not valid until the scope is entered.
    + i.e. you cannot use the pattern variables in the conditional statement itself.
+ `while let` loop
  + Runs a `while` loop as long as the pattern matches.
+ `for ... in ...` loop
+ `let` statements
  + `let <pattern> = <expression>`
  + Used for destructuring.
+ Function and closure parameters are patterns.
  + Can destructure arguments in the call signature.
+ Ranges can be used as patterns.
  + Only numeric oor character ranges are allowed.
  + `match x { 1..=5 => { ... }, ... }` will match x in { 1, 2, 3, 4, 5 }.
  + `match x { 'a'..='e' => { ... }, ... }` will match x in { 'a', 'b', 'c', 'd', 'e' }.
+ **Irrefutable** patterns match for any possible value passed to it.
  + Otherwise called **refutable**
  + `let` and `for` loops can only accept irrrefutable patterns.
  + `if let` and `while let` can accept refutable or irrefutable patterns.
  + `match` statement arms accept only refutable patterns, except in one arm (the default).
+ Can desctructure `structs`.
  ```
  struct Point {
    x: i32,
    y: i32,
  }

  let p = Point{ x: 0, y: 7 };
  let Point{ x, y } = p;

  match p {
    Point { x, y: 0 } => { ... },
    ...
  }
  ```
+ Can destructure `enum`s.
  + When destructuring an enum with data, the pattern must match the data layout.
+ Patterns can nest.
+ Ignoring values is done with
  + A root underscore `_` to match the entire pattern but not bind to anything.
    + Useful for function parameters to ignore a required argument (e.g. to implement a trait).
  + An underscore `_` within another pattern to match anything but not bind to it.
  + A name that starts with an underscore `_<name>` to not be warned about an unused variable.
  + `..` to match the remaining components of a pattern without binding to any of them.
    + Must be unambiguous.
    + e.g. `( a, .., d ) = ( 1, 2, 3, 4 )`
+ `@` bindings
  + Creates a variable that holds a value at the same time it is being tested to match a pattern.

### Unsafe Rust

+ Use the `unsafe` keyword to enter unsafe mode within a scope.
+ Allows you to
  + Dereference raw pointers.
    + `*const T` and `*mut T`, where `*` is part of the type name, not a dereference.
    + Can ignore borrowing rules.
    + Can be null or point to invalid memory.
    + Don't implment automatic cleanup.
    + Can be defined outside an `unsafe` block, but not dereferenced.
  + Call unsafe functions or methods.
    + Entire functions can be marked unsafe, or just inner blocks can.
    + Used to create a safe API for unsafe code.
    + `extern` can be used to call code written in another language or allow other languages to call Rust functions.
      + To call code from other languages creates a _foreign function interface_ (FFI).
      + To create code for other languages creates an _application binary interface_ (ABI).
      + Must use `#[no_mangle]` when creating code for other lanugages.
        + May not require `unsafe`.
  + Access or modify a mutable static variable.
    + Global variables are called `static`.
    + Written in SCREAMING_SNAKE_CASE by convention.
    + Only allowed to have `'static` lifetime.
    + Always point to the same memory address.
    + Can be mutated, but must be in an `unsafe` block.
  + Implement an unsafe trait.
    + A trait is unsafe if any of its methods are unsafe.
    + Mark the `trait`, method, and `impl` `unsafe`.
  + Access fields of `union`s.
    + Usually used to interface with C `union`s. 

### Fully qualified syntax

+ If an object has multiple functions with the same name (e.g. from implementing multiple traits that happens to have the same method name) the specific function can be called by passing a reference to the pbject to the qualified method.
+ If a function with the name is implemented directly on the type it is called by default.
+ Because associated functions don't have `self` references you must indicate the type as `< <MyType> as <MyTrait> >::my_fcn()`
+ In general the syntax is `<Type as Trait>::function( receiver_if_method, arg1, ... )`
+ e.g. `MyTrait::my_duplicate_function( &my_obj )`
+ e.g. `<MyType as MyTrait>::my_duplicate_function()`

### Function pointers

+ Use the type `fn` with the call signature.
+ Enums with data are implmented as function pointers.
+ Closures can be returned behind a pointer.
+ e.g. `fn repeat( f: fn( i32 ) -> i32, ... ) { ... }`

### Macros

+ Macros are code that write other code, known as _metaprogramming_.
+ There are _declarative_ and _procedural_ macros.
  + There are three types of procedural macros.
    + `#[derive]` macros that specify code added to `struct`s and `enum`s.
    + Attribute-like macros that define custom attributes.
    + Function-like macros that operate on the tokens specified as their arguments.
+ Macros can take a variable number of arguments.
+ Macros must be defined and in scope before use.
+ **Declarative macros**
  + Similar to a `match` statement, but the matched pattern is the literal Rust source code passed to the macros and the arm patterns are the structure of the code.
+ Defined with the `macro_rules!` construct.
+ `#[macro_export]` annotation indicates a macro should be available when its crate is in scope.
+ **Procedural macros**
  + Take code as input, operate on it, and return the new code as output.
  + Definitions must reside in their own special crate.
  + Must bring the `proc_macro` crate into scope.
  + Defined as a function with the attribute `#[<my_macro>]`.
    + Function signature is `pub fn my_macro( input: TokenStream ) -> TokenStream { ... }`
  + **Custom derive macro**
    + The macro and trait names should match.
    + Macro must be defined in its own library crate named `<macro name>_derive` which is inside the primary library crate of the macro.
    + Both the macro and macro derive crates must be brought into scope and published separately.
    + Mark as a derived macro in the derive macro `Cargo.toml` file as `proc-macro = true` under the `[lib]` section.
    + Likely need the `syn` crate to use `syn::parse` to parse the input code into a syntax tree.
    + Likely need the `quote` crate to turn the syntax tree back into a code string.
    + Must mark the derive function with the `#[proc_macro_derive(<macro name>)]` attribute.
      + Allows users to write `#[derive(<macro name>)]` to derive the functionality.
    + Functions should `panic!` on errors.
  + **Attribute-like macros**
    + Create new attributes.
    + Work the same as custom derive macros.
    + Tagged with `#[proc_macro_attribute]`.
    + Call signature of `pub fn <macro name>( attr: TokenStreak, item: TokenStream ) -> TokenStream { ... }`
        + `attr` is the contents of the attribute passed into the macro.
        + `item` is the body of the item the atribute is attached to.
  + **Function-like macros**
    + Can take an unknown number of arguments.
    + Called as a function.
    + Tagged with `#[proc_macro]`.
    + Call signature of `pub fn <macro name>( input: TokenStream ) -> TokenStream { ... }`.
        + `input` is whatever is passed into the macro

### Comments

+ `//!` Inner line doc comment.
+ `///` Outer line doc comment.
+ `/* ... */` Block comment.
+ `/*! ... */` Inner block doc comment.
+ `/** ... */` Outer block doc comment.

### Language features

+ Can overload operators in the `std::ops` crate.
+ `type` keyword is used to create type aliases.
  + e.g. `type MyInt = i32;`
+ The _empty_ or _never type_ is denoted with a bang `!`.
  + Used to denote when a function never returns, called _diverging funtions_.
  + Expressions of type `!` can be coerced into any other type.
  + e.g. `fn forever() -> ! { ... }`
+ **Dynamically sized types**
  + A type whose size can only be known at runtime.
  + Must be used as a reference with dynamic data, pointing to the memory location of the value along with metadata about it, such as its length.
  + `Sized` trait determines whether or not a type's size is known at compile time.
    + `Sized` is automatically added to every bound of a generic function.
      + e.g. `fn generic_fn<T: Sized>( t: T ) { ... }`
      + This can be relaxed by explicitly stating `fn generic_fcn<T: ?Sized>( t: &T ) { ... }`
        + The parameter must also be placed behind a pointer if the `Sized` trait requirement is removed.
+ **Raw identifiers**
  + Syntax that allows keyword use.
  + Indicated with a `r#` prefix.


## Ownership
+ Each value has a variable which is its owner.
+ A value can only have a single owner at a time.
+ When the owner goes out of scope, that memory  is deallocated.
+ Immutable reference:
  + `fn read_val( val: $String )`
  + `read_val( &s )`
  + Can have multiple immutable references.
+ Mutable reference:
  + `fn change_val( val: &mut String )`
  + `change_val( &mut s )`
  + Can only have one mutable reference to a piece of data at a time.
  + Can not have a mutable reference if an immutable one already exists.

## Crate and Modules
+ A package can contain multiple binary crates and optionally one library crate.
+ Multiple packages can be grouped into a single workspace.
+ **Crates**
  + Can be of type `binary` or `library`.
  + Macros must live in their own crate.
+ **Packages**
  + Group of crates containing related functionality.
  + Contains a `Cargo.toml` file to describe how to build its crates.
  + Can contain at most one library crate.
    + Names `lib.rs`.
  + Multiple binary crates can be used by placing them in the `src/bin/` directory.
    + Each file is its own binary crate.
+ **Modules**
  + Used to organize and control the privacy of code within a crate.
  + Defined using `mod` keyword.
  + Modules can be nested.
  + Paths are specified using `::` as a delimeter.
    + `crate` keyword indicates root of the package for absolute paths.
    + `super` keyword refers to a module's parent.
  + Public APIs are marked with the `pub` keyword.
  + If a `struct` contains private fields it must implement a public associated function to instantiate instances of it.
  + If an `enum` is marked public all of its variants are public.
  + The `use` keyword brings a path into scope.
    + For relative paths use `self`to start the path.
    + Can specify the path name with `as`.
    + Must use `pub use` to make public and re-export.
    + Can nest paths to bring in multiple paths from same root.
      + `use <pkg>::{ <c1>::<cc1>, <c2>[, ...] };`
    + Can use glob wildcard `*` to bring in all public items from a path.
      + `use <pkg>::<c1>::*;`
  + Can split a module into multiple files by including them with `mod <file>;`.
    + Submodules reside in a folder named as their parent module.
+ Paths to local crates can be specified in the `Cargo.toml` file.

## Tests
+ Test functions are annotated with the `test` attribute as `#[test]`.
+ Run tests with the `cargo test` command.
+ Tests fail if a `panic!` occurs within the test body.
+ Every test runs in its own thread.
+ Use `use super::*;` to bring in all code from outer module into test inner test module.
+ **Testing macros**
  + **`assert!( <expr> )`:** Ensures an expression evaluates to `true`, `panic!`s if it evaluated to `false`.
  + **`assert_eq!( <a>, <b> )`:** Tests equality between two values using `==`.
  + **`assert_ne!( <a>, <b> )`:** Tests inequality between two values using `!=`.
  + **`#[should_panic]`:** Attribute to indicate a function should cause a panic.
    + Use and `expected` parameter to match panic message.
  + **`#[ignore]`:** Attribute to ignore test by default, only running if specified `--ignored` flag is passed to `cargo test` binary.
  + For `assert_eq!` and `assert_ne!` the values must implment the `PartialEq` and `Debug` traits.
  + Additional arguments for `assert`, `assert_eq`, and `assert_ne` passed to any of the macros are passed along to `format!` and can be used as additional debugging messages if the assertion fails. 
+ If a test's return value is set to be a `Result<T, E>` the error message can be set and the `?` operator can be used.
  + Can not use the `should_panic` attribute if teh test retruns `Result<T, E>`
  + To ensure a test returns the `Err` variant use `assert!( value.is_err() )`.
+ **Controlling test runs**
  + Flags are separated with a double dash `--`, `cargo test <cargo flags> -- <binary flags>`.
    + Flags before go to `cargo test`
    + Flags after go to the binary.
  + `cargo test` flags
    + To run a single test, pass the name of the test.
    + Can run multiple tests with similar names by passing a string contained in them.
    + Only run specific integration test files using the `--test <file>` flag. 
  + binary flags
    + `--test-threads=<#>`: How many threads are used to run tests.
      + Use `1` to run in series.
    + `--show-output` prints output of passed tests.
    + `--ignored` only run tests marked `#[ignore]`.
    + `--include-ignored` run all tests, even those marked `#[ignore]`.
+ **Unit tests** should be included in the same file as the code they're testing.
  + Convention is to create a module named `tests` in each file to contain the tests, annotated with `#[cfg(test)]`.
  + **`#[cfg(test)]`:** Annotation used to leave a module out of build binaires, and only inlude it in test binaries.
+ **Integration tests** are included in a `tests` folder at the top level of the project, next to `src`.
  + Each file in `tests` is compiled as its own crate.
  + Place common setup code in `tests` folder using `tests/<crate name>/mod.rs` so it won't be run as a test crate.
    + Files in subdirectories of `tests` do not get compiled as separate crates.
  + Binary crates can not have integration tests as they are not meant to be consumed.
    + Binary crates can consume their own library crates for functionality and testing.
