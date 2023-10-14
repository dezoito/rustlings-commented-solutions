// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// * To invoke a macro you have to add a "!" to its name
fn main() {
    my_macro!();
}
