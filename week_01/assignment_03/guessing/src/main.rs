extern crate rand;

use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("*** Guess the number ***");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        print!("> Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("failed to read line");

        let guess: i64 = match input.trim().parse() {
          Ok(num) => num,
          Err(err) => {
              println!("Error typing input number: {}", err);
              continue;
          },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break
            }
        }
    }
}
