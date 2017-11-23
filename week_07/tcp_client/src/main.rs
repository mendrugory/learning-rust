use std::io::{Write, Read};
use std::net::TcpStream;
use std::time;
use std::thread;

fn main() {
    let address = "0.0.0.0:5555";
    match TcpStream::connect(address) {
        Ok(stream) => communication(stream),
        Err(error) => println!("Error connecting to {}. Error: {}.", address, error),
    }
}

fn communication(mut stream: TcpStream) {
    for _ in 1..10 {
        let mut buffer: [u8; 512] = [0; 512];
        let _ = stream.read(&mut buffer);
        println!("I received: {:?}", String::from_utf8_lossy(&buffer));
        let _ = stream.write(b"Hola");
        let sleep_time = time::Duration::from_millis(100000);
        thread::sleep(sleep_time);
    }
}
