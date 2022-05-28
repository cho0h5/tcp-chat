use std::io::prelude::*;
use std::net::TcpListener;
use std::str;
use std::env;

fn main() -> std::io::Result<()> {
    let host = match env::args().nth(1) {
        Some(host) => host,
        None => String::from("0.0.0.0:8080"),
    };

    let listener = TcpListener::bind(host)?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        
        println!("[CONNECTED] {}", stream.peer_addr().unwrap());
        loop {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(n) => if n != 0 {
                    println!("[ECHO] {}", str::from_utf8(&buffer).unwrap());
                },
                Err(_) => {
                    break
                },
            }
            match stream.write(&buffer) {
                Ok(_) => (),
                Err(_) => {
                    break
                },
            }
        }
        println!("[DISCONNECTED]\n");
    }

    Ok(())
}