// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // * This is what the error message suggests
    // * "blue".to_string()

    // * This is what the hint wanted (both seem to be interchangeable)
    String::from("blue")
}
