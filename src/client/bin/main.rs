use std::net::TcpStream;
use std::io::prelude::*;
use std::str;
use std::thread;
use std::env;

fn main() -> std::io::Result<()> {
    let host = match env::args().nth(1) {
        Some(host) => host,
        None => String::from("0.0.0.0:8080"),
    };

    let mut stream = TcpStream::connect(host)?;
    let mut stream_recv = stream.try_clone().unwrap();
    println!("[INFO] Connected");

    thread::spawn(move || {
        loop {
            let mut buffer = [0; 1024];
            match stream_recv.read(&mut buffer) {
                Ok(n) => if n != 0 {
                    println!("[RECV] {}", str::from_utf8(&buffer).unwrap());
                },
                Err(_) => {
                    break
                },
            }
        }
    });
    
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;

        stream.write(buffer.trim().as_bytes()).unwrap();
    }
}