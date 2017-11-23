use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;



fn main() {
    let address = "0.0.0.0:5555";
    let listener = TcpListener::bind(address).unwrap();
    println!("Server {} listening. Ready to accept", address);
    for tcp_stream in listener.incoming() {
        match tcp_stream {
            Ok(stream) => {
                thread::spawn(|| communication(stream));
            }
            Err(error) => println!("Error binding connection: {}.", error),
        }

    }
}

fn communication(mut stream: TcpStream) {
    let _ = stream.write(b"Welcome to the TCP server.\r\n");
    loop {
        let mut buffer: [u8; 512] = [0; 512];
        let _ = stream.read(&mut buffer);
        println!("Server received: {:?}", String::from_utf8_lossy(&buffer));
        let _ = stream.write(b"Message was received.\r\n");
    }
}
