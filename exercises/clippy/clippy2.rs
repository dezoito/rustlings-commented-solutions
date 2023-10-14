// clippy2.rs
//
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);

    // * We are not allowed to loop over an Option (code below)
    // for x in option {
    //     res += x;
    // }
    // * But we can use an "if let" statement to check that "option"
    // * is a "Some" variant and execute the inner block
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
