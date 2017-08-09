extern crate rand;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time;
use rand::Rng;
use std::fmt::Display;

fn main() {

    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        loop{
            match rx.recv() {
                Ok(msg) => consume(msg),
                Err(error) => println!("Error: {}", error),
            }
        }
    });

    let mut message: i32 = 1;
    loop{
        message = produce(&tx, message + 1);
    }
    
}

fn consume<T: Display>(message: T) {
    println!("--- Received Message: {}", message);
    println!("--- Consuming Message: {}", message);
    random_sleep_time();
    println!("--- Message {} consumed", message);
    println!(" ");
}

fn produce<T: Display + Copy>(tx: &Sender<T>, message: T) -> T {
    println!("+++ Sending Message: {}", message);
    let _ = tx.send(message);
    random_sleep_time();
    println!(" ");
    message
}

fn random_sleep_time(){
    let mut rng = rand::thread_rng();
    let millis: u64 = rng.gen_range(1000, 2000);
    let sleep_time = time::Duration::from_millis(millis);
    thread::sleep(sleep_time);
}