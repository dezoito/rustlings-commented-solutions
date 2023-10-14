// clippy3.rs
//
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;

    // * "my_option" is initialized with None, so the code below
    // * will always panic:
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }

    // * We have to remove the unwrap() call
    // * and do something with var
    if my_option.is_none() {
        println!("The current value of my_option is {:?}", my_option);
    }

    // * We were missing a comma
    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    // * The resize part of the code below mutates the vector
    // * to length zero, effectively returning [], but you can't do it
    // * in a single line
    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    // println!("This Vec is empty, see? {:?}", my_empty_vec);

    // * It would make sense to rewrite it like this, but then
    // * Clippy will tell you to clear() the vector to make it empty
    // let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    // my_empty_vec.resize(0,5);

    // * So this is how we fix all warnings:
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
