// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

// * option 1: set the return type to result () (think void) and the same type of error
// * defined as the return type for total_cost()
// * then add Ok(()) to return as result
// * Notice the ? at the end of the call to total_cost(), indicating that it can
// * propagate an error
// fn main() -> Result<(), ParseIntError> {
//     let mut tokens = 100;
//     let pretend_user_input = "8";

//     let cost = total_cost(pretend_user_input)?;

//     if cost > tokens {
//         println!("You can't afford that many!");
//     } else {
//         tokens -= cost;
//         println!("You now have {} tokens.", tokens);
//     }
//     Ok(())
// }

// * option 2: Leave function without a return type, but use a match statement
// * to return the value OR the error
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = match total_cost(pretend_user_input) {
        Ok(cost) => Ok(if cost > tokens {
            println!("You can't afford that many!");
        } else {
            tokens -= cost;
            println!("You now have {} tokens.", tokens);
        }),
        Err(error) => Err(error),
    };
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
