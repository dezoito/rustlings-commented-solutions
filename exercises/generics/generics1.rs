// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // * define de &str as the type for Vec<>
    // * since we are pushing a string slice into it
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
