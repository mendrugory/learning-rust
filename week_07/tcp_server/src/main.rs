use std::io::{Write, BufReader, BufRead, BufWriter};
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
    let _ = stream.write("Welcome to the TCP server.\r\n".as_bytes());
    let mut reader_buffer = BufReader::new(&stream);
    let mut writer_buffer = BufWriter::new(&stream);
    loop {
        let mut response = String::new();
        reader_buffer.read_line(&mut response).expect("could not read");
        println!("Server received: {}", response.trim());
        writer_buffer.write_all("Message was received.\r\n".as_bytes()).expect("could not write");
        writer_buffer.flush().expect("could not flush");
    }
}
