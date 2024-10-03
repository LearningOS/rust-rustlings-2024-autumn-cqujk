// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

#![feature(decl_macro)]
mod macros {
    pub macro my_macro {
        () => {
            println!("Check out my macro!");
        }
    }
}

fn main() {
    macros::my_macro!();
}
