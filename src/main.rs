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
        thread::spawn( move || {
            match stream {
                Ok(mut stream) => { 
                    let mut map : HashMap<String , String> = HashMap::new();
                    loop {
                        let mut buff = [0u8 ; 1024];
                        let bytes_read = stream.read(&mut buff).unwrap();

                        if bytes_read == 0 {
                            break;
                        }
                        let input = String::from_utf8_lossy(&buff[..bytes_read]);
                        let response = handle_command(&input , &mut map);

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
fn handle_command(input : &str ,  map : &mut HashMap<String , String>) -> String{
    let lines : Vec<&str>  = input.split("\r\n").collect();

    if !input.contains("\r\n") {
         let result = raw_server_input(input , map);
         return result
    }

    if lines.len() < 2 {
        println!("Received non-RESP input or partial data: {:?}", lines);
        return "".to_string(); // Ignore it or return a RESP error
    }


    if lines[2].to_lowercase() == "get" {
        let key = lines[4];
        if let Some(res) = map.get(key) {
            let length_of_string : usize = res.len();
            let second = res.to_string();
            let result = format!("${length_of_string}\r\n{second}\r\n");
            return result
        }
        return "$-1\r\n".to_string(); 
    }

    if lines[2].to_lowercase() == "set" { 
        let key = lines[4].to_string();
        let result = lines[6].to_string();

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

    return "".to_string()
}

fn raw_server_input(input: &str, map: &mut HashMap<String, String>) -> String {
    let clean_input = input.trim();
    
    let lines: Vec<&str> = clean_input.split_whitespace().collect();
    
    println!("{:?} result of raw server input", lines);

    if lines.is_empty() {
        return "".to_string();
    }

    let command = lines[0].to_lowercase();

    if command == "get" {
        if lines.len() > 1 {
            let key = lines[1];
            if let Some(res) = map.get(key) {
                return format!("{}\n", res); 
            }
        }
        return "(nil)\n".to_string(); 
    }

    if command == "set" { 
        if lines.len() > 2 {
            let key = lines[1].to_string();
            let result = lines[2].to_string();

            map.insert(key, result);
            return "OK\n".to_string();
        }
        return "ERR\n".to_string();
    }

    if command == "ping" {
        return "PONG\n".to_string();
    }

    if command == "echo" {
        let result = lines[1..].join(" ");
        return format!("{}\n", result);
    }

    "".to_string()
}

