#![allow(unused_imports)]
use std::thread;
use std::{io::stdin, net::TcpListener};
use std::io::{self, Read, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

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

//program sends data in RESP format like this -> *2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n
// *2 indicates an array with 2 elements
// $4 indicates a bulk string of 4 bytes
fn handle_command(input : &str) -> String{
    let lines : Vec<&str>  = input.split("\r\n").collect();

    println!("{:?} after split" , lines);

   //let string_iter : Vec<&str> = input.split(' ').collect();

   if lines[1] == "ping" {
        return  "PONG".to_string();
   }

   if lines[1] == "echo" {
       let length_of_string = lines[2].len();
       let second = lines[1];
       let result = format!("{length_of_string}\r\n{second}\r\n");
       return result
   }


   "".to_string()
}

