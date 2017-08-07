use std::io::{self, Write};

fn main() {
    println!(" *** Reverse the String ***");
    print!("> Type your String: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Could not read line");
    println!("{}", reverse(input))
}

fn reverse(s: String) -> String {
    s.chars().rev().collect::<String>()
}
