// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        // * we can simply add the ref keyword before p in the match expression.
        // * This will create a reference to the Point value without moving it
        // * out of the y variable.
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }

    // * this y would be empty without the ref above
    y; // Fix without deleting this line.
}
