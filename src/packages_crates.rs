/// Packages and Crates
/*

Crate: binary or library
- crate root: src file that compiler starts from and makes root module
- groups related functionary in a scope (easy to share for multiple projects)

Package: 1+ crates to provide a set of functionality
- Cargo.toml: describes how to build crates
- Contain rules:
  - library crates: 0 or 1; No more
  - binary crates: as many
    - location: src/bin
    - each file will be separate binary crate
  - min: at least 1 crate (lib or bin)

Cargo
- passes crate root files to rustc to build library/binary
- convention:
  - src/main.rs is the crate root of a binary crate (same name as pkg)
  - src/lib.rs  is the crate root of a library crate

 */
fn package_placeholder() {}

/// Scope and Privacy
/*

Modules use cases: organize code for reuse, readability, privacy
- modules hold: other modules, structs, enums, constants, traits, function

Paths:
- absolute: start from crate root (name or literal `crate`)
- relative: start from current mod (`self`, `super`, identifier)
- separate identifiers: `::`

Example:

restaurant
    front_of_house
        hosting
            add_to_waitlist
            seat_at_table
        serving
            take_order
            serve_order
            take_payment
    eat_at_restaurant

if you move 'front_of_house' and 'eat_at_restaurant'
    -> into mod: 'customer_experience'
: then update abs path to "add_to_waitlist"
, but relative path is valid

if you moved 'eat_at_restaurant' function separately
    -> into 'dining'
: then abs path to 'add_to_waitlist' stays the same
, but relative path needs to be updated

Preference: abs paths: since more likely to move code definitions and item calls
- independently

Privacy:
- all items (fn, method, struct, enum, mod, const) are private by default
- items in parent module cannot use private items in child
- but child modules can use items in ancestor modules
- Child mod: wrap and hide implementation details
    but able to see the context they're are defined in
    i.e. restaurant analogy: back office is private to customers, but
        office managers can see everything in the restaurant
 */
fn privacy_placeholder() {}

/// Use Paths
/*

Functions: use parent
    - shows that the function isn't locally defined
All else: use full path
    - just convention
    Exception: same name
        Solution: `as` keyword to rename locally

Re-exporting Names with `pub use`
- bring name into scope (use) but private
- pub + use -> brings item into scope and available for others to use
- as if defined in the code scope
Why?
    - useful when internal structure is different from how programmers
    - would consider the domain
    i.e. customers don't think think about a restaurant into front, and back
    - shouldn't have to think like that -> just 1 structure 'restaurant'

Example:
pub use crate::front_of_house::hosting

allows external code to call: hosting::add_to_waitlist()
 */


fn keyword_as() {
    // use std::fmt::Result;
    // use std::io::Result as IoResult;
    // fn function1() -> Result {}
    // fn function2() -> IoResult<()> {}
}

/// External Packages
/*
Adding package to Cargo.toml as a dependency
- Cargo will download the package and any of its dependencies from crates.io

bring into scope with `use`

Nested Paths
use std::io;
use std::io::Write;

-> use::io::{self, Write};

Global Operator
- brings all public items into path
- harder to distinguish when names were brought into scope vs already defined
- often used in `tests` module
use std::collections::*;
 */
fn external_packages() {}

