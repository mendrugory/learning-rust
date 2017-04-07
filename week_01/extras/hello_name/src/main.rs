use std::env;

fn main() {
    if let Some(name) = env::args().nth(1){
        println!("Hello {}!", name);
    }
}
