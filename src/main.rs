#![allow(unused_imports)]
use std::collections::HashMap;
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
    let mut map : HashMap<&str , String> = HashMap::new();
    let lines : Vec<&str>  = input.split("\r\n").collect();

    println!("{:?} after split" , lines);

    if lines[2].to_lowercase() == "get" {
        let key = lines[4];

        if let Some(res) = map.get(key){
            return res.to_string()
        }
        return "$-1\r\n".to_string(); 
    }

    if lines[2].to_lowercase() == "set" { 
        let key = lines[4];
       let length_of_string = lines[6].len();
       let second = lines[6];
       let result = format!("${length_of_string}\r\n{second}\r\n");

       map.insert(key , result);
       return "+OK\r\n".to_string()
    }

   //let string_iter : Vec<&str> = input.split(' ').collect();

   if lines[2].to_lowercase() == "ping" {
        return "+PONG\r\n".to_string();
   }

   if lines[2].to_lowercase() == "echo" {
       let length_of_string = lines[4].len();
       let second = lines[4];
       let result = format!("${length_of_string}\r\n{second}\r\n");
       return result
   }

   "".to_string()
}

