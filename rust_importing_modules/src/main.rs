/*
packages and crates

A crate can come in one of two forms.
- binary crate : programs which can be compiled to be an executable. main() required.
- library crate : Define funtionality intended to be shared with multiple projects.

A package is an bundle of one or more crates that provides a set of functionality.
This includes a 'Cargo.toml' file which describes how to build those crates.
Cargo itself is actually a package that contains the binary crate for the cli you've been using to build your code.

Package can contain as amny binary crates you like but, only one library crate, and should contain at least one crate whether it's a binary or library.
*/

/* rules cheat sheet
1. start from crate root.
- lib.rs/main.rs
2. declaring modules. 
Say, you declare a “garden” module with mod garden;
- Inline, within curly brackets that replace the semicolon following mod garden
- In the file src/garden.rs
- In the file src/garden/mod.rs
3. declaring sub-modules. 
You might declare "vegies" module inside mod garden.
- Inline, directly following mod vegetables, within curly brackets instead of the semicolon
- In the file src/garden/vegetables.rs
- In the file src/garden/vegetables/mod.rs
4. Paths to code in modules.
For example, an Asparagus type in the garden vegetables module would be found at 
crate::garden::vegetables::Asparagus.
5. the "use" keyword 
You can create shortcuts for items to reduce repetition.
use crate::garden::vegetables::Asparagus; -> You can just write "Asparagus" within that scope.
*/

extern crate rust_importing_modules as lib;
use crate::garden::vegetables::Asparagus;
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I am growing {:?}", plant);
    /* using lib.rs in main.rs for explanation */
    // using "lib" in main : they are same level of crate, not module.
    rust_importing_modules::front_of_house::hosting::add_to_waitlist();
    lib::eat_at_restaurant(); // can use "as" for alias.
    
    
}
