use std::io::{self, Write};

fn main() {
    println!(" *** Fibonnaci Generator ***");
    print!("> Type your Fib number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Could not read line");
    let nth: i64 = input.trim().parse().unwrap();

    println!("> Result: {}", fib(nth))
}

fn fib(nth: i64) -> i64 {
    if nth > 2 {
        fib(nth - 1) + fib(nth - 2)
    }
    else{
        1
    }
}
