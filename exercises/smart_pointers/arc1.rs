// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

// * Suggested reading:
// * https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    // * Arc (short for "Atomic Reference Counting")
    // * is used to share ownership of the numbers vector across multiple threads.
    // * It garantees thread safety (multiple threads can read the data as it's immutable)
    let shared_numbers = Arc::new(numbers);

    // * This will store a vector where each item is a handle for a thread
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        // * Each thread is given access to child_numbers, which is a clone of shared_numbers.
        // * This is efficient because Arc only maintains a reference count, not the data itself.
        let child_numbers = Arc::clone(&shared_numbers);

        // * Each spawned thread takes ownership of its child_numbers clone.
        // * the *move* keyword is used in closures to indicate that the closure should take
        // * ownership of the variables it captures from its surrounding scope.
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }

    // * this part is used to wait for each spawned thread to finish its
    // * execution before proceeding further.
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
