use std::io;
use std::io::Write;

fn main() {
    let mut n = String::new();

    print!("Choose a value for n: ");
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n).expect("Failed to read n.");

    let mut n: u128 = n.trim().parse().expect("Non-integer value input!");
    let nth: u128 = n;

    //
    if n < 1 {
        println!("The Fibonacci Series is indexed on 1!");
        return;
    }

    //
    let mut x0: u128;
    let mut x1: u128 = 1;
    let mut x2: u128 = 1;

    //
    while n > 1 {
        x0 = x1;
        x1 = x2;
        x2 = x0 + x1;

        n = n - 1;
    }

    println!("(n={nth}) nth element: {x1}")
}
