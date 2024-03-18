// Author: Diego Ferrer
// Task: 

use std::io; // needed for reading values from stdin
use std::io::Write; // needed to flush output buffer

fn main() {
    // initialize a mutable string
    let mut n = String::new();

    // first prompt
    print!("Choose a value for n: ");

    // read response, store it in n, and flush the buffer
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n).expect("Failed to read n.");

    // convert string response to u128 type
    let mut n: u128 = n.trim().parse().expect("Non-integer value input!");
    let nth: u128 = n;

    // handle n=0 case
    if n < 1 {
        println!("The Fibonacci Series begins where fn (n=1) is 1!");
        return;
    }

    // initialize sequence
    let mut x0: u128;
    let mut x1: u128 = 1;
    let mut x2: u128 = 1;

    // calculate nth fibonacci
    while n > 1 {
        x0 = x1;
        x1 = x2;
        x2 = x0 + x1;

        n = n - 1;
    }

    // print result
    println!("(n={nth}) nth element: {x1}")
}
