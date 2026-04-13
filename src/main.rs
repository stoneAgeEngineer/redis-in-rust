#![allow(unused_imports)]
use std::{io::stdin, net::TcpListener};
use std::io::{self, Read, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment the code below to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                loop {
                    let mut buff = [0u8 , 64];
                    let bytes_read = stream.read(&mut buff).unwrap();
                    if bytes_read == 0 {
                        break;
                    }

                    stream.write_all(b"+PONG\r\n").unwrap()
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    /*
       let mut buffer = String::new();
       io::stdin().read_line(&mut buffer);
       io::stdout().write_all("PONG");
       */
}
