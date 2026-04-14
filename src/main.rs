#![allow(unused_imports)]
use std::thread;
use std::{io::stdin, net::TcpListener};
use std::io::{self, Read, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment the code below to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        thread::spawn( || {
            match stream {
                Ok(mut stream) => { 
                    loop {
                        let mut buff = [0u8 ; 1024];
                        let bytes_read = stream.read(&mut buff).unwrap();
                        if bytes_read == 0 {
                            break;
                        }

                        let input = String::from_utf8_lossy(&buff[..bytes_read]);
                        let response = handle_command(&input);

                        stream.write_all(response.as_bytes()).unwrap()
                    }
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
    }
}

fn handle_command(input : &str) -> String{
   let input =  input.trim();

   if !input.starts_with('*') {
        return "+PONG\r\n".to_string();
    }

}
