use std::io::{BufReader, Write, BufRead, BufWriter};
use std::net::{TcpStream, Shutdown};
use std::time;
use std::thread;

fn main() {
    let address = "0.0.0.0:5555";
    match TcpStream::connect(address) {
        Ok(stream) => communication(stream),
        Err(error) => println!("Error connecting to {}. Error: {}.", address, error),
    }
}

fn communication(stream: TcpStream) {
    for _ in 1..10 {
        let mut reader_buffer = BufReader::new(&stream);
        let mut writer_buffer = BufWriter::new(&stream);
        let mut response = String::new();
        reader_buffer.read_line(&mut response).expect("could not read");
        println!("I received: {:?}", response.trim());
        writer_buffer.write_all("Hola.\r\n".as_bytes()).expect("could not write");
        writer_buffer.flush().expect("could not flush");
        let sleep_time = time::Duration::from_millis(1000);
        thread::sleep(sleep_time);
    }
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
}
