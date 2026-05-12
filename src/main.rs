#![allow(unused_imports)]
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{io::stdin, net::TcpListener};
use std::io::{self, Read, Write};

struct HashMapValues {
    time_to_ex : String,
    result : String
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    let user_db : Arc<Mutex<HashMap<String , String>>> =  Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        thread::spawn( move || {
            match stream {
                Ok(mut stream) => { 
                    let mut map : HashMap<String , HashMapValues> = HashMap::new();
                    let mut user_map : HashMap<String , String> = HashMap::new();
                    let mut is_authenticated :Option<bool> = None ;
                    loop {
                        let mut buff = [0u8 ; 1024];
                        let bytes_read = stream.read(&mut buff).unwrap();

                        if bytes_read == 0 {
                            break;
                        }
                        let input = String::from_utf8_lossy(&buff[..bytes_read]);

                        let response = handle_command(&input , &mut map , &mut user_map , &mut is_authenticated);

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
fn handle_command(input : &str ,  map : &mut HashMap<String , HashMapValues> , user_map : &mut HashMap<String , String> , is_authenticated : &mut Option<bool>) -> String{
    let lines : Vec<&str>  = input.split("\r\n").collect();
    println!("{:?} result of lines" , lines);

    /*
       if !input.contains("\r\n") {
       let result = raw_server_input(input , map);
       return result
       }
       */


    // for auth they send like this auth <username> <password> - authenticates current connection
    // with a specified username
    let auth_response = handle_auth(lines.clone(), is_authenticated , user_map);

    if !auth_response.is_empty() {
        return auth_response
    }

    if let Some(false) = is_authenticated{
        return "-NOAUTH Authentication required.\r\n".to_string()
    };


    if lines.len() < 3 {
        println!("Received non-RESP input or partial data: {:?}", lines);
        return "".to_string(); // Ignore it or return a RESP error
    }

    if lines[2].to_lowercase() == "acl" && lines[4].to_lowercase() == "whoami" {
        return format!("$7\r\ndefault\r\n");
    }

    if lines[2].to_lowercase() == "acl" && lines[4].to_lowercase() == "getuser" {
        if let Some(res) =  user_map.get(lines[6]){ // return nopass in array because user is
                                                    // authenticated
            return format!("*4\r\n$5\r\nflags\r\n*0\r\n$9\r\npasswords\r\n*1\r\n${}\r\n{}\r\n" , res.len() , res)
        }
        return format!("*4\r\n$5\r\nflags\r\n*1\r\n$6\r\nnopass\r\n$9\r\npasswords\r\n*0\r\n")
    }

    if lines[2].to_lowercase() == "get" {
        let key = lines[4];
        if let Some(res) = map.get(key) {

            if !res.time_to_ex.is_empty() {
                let time_val : Vec<&str> = res.time_to_ex.split(":").collect();

                println!("{:?}  time_val" , time_val);

                let ttl_type = time_val[0];
                let ttl_value : u128 = time_val[1].parse().unwrap();
                let created_at : u128 = time_val[2].parse().unwrap();

                println!("{:?} ,  {:?} , {:?}  ttl , created_at , ttl_type" , ttl_value , created_at , ttl_type);

                let now  = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                let elasped = now - created_at;

                if ttl_type.to_uppercase() == "PX" {
                    println!("Inside px");

                    if elasped <= ttl_value {
                        println!("elased greater than ttl");
                        let length_of_string : usize = res.result.len();
                        let second = res.result.to_string();
                        let result = format!("${length_of_string}\r\n{second}\r\n");
                        return result
                    }else if elasped > ttl_value {
                        println!("going inside elapsed less than ttl_value");
                        map.remove(key);
                        return "$-1\r\n".to_string(); 
                    }
                }

                if ttl_type.to_uppercase() == "EX" {
                    if elasped <= (ttl_value * 1000) {
                        let length_of_string : usize = res.result.len();
                        let second = res.result.to_string();
                        let result = format!("${length_of_string}\r\n{second}\r\n");
                        return result;
                    }else if elasped < (ttl_value * 1000) {
                        map.remove(key);
                        return "$-1\r\n".to_string(); 
                    }
                }
            }
            let length_of_string : usize = res.result.len();
            let second = res.result.to_string();
            let result = format!("${length_of_string}\r\n{second}\r\n");
            return result;
        }
        return "$-1\r\n".to_string(); 
    }

    if lines[2].to_lowercase() == "set" { 
        let mut time_to_ex = String::new() ;

        if lines.len() >= 10 {
            let now_mill_sec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            time_to_ex = format!("{}:{}:{}" , lines[8] , lines[10] , now_mill_sec);
        }

        let key =  lines[4].to_string();
        let final_result = HashMapValues{
            time_to_ex : time_to_ex,
            result : lines[6].to_string()
        };

        map.insert(key , final_result);
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

fn handle_auth(lines : Vec<&str> , is_authenticated : &mut Option<bool> , user_map : &mut HashMap<String , String>) -> String{
    if lines[2].to_lowercase() == "auth" {
         if let Some(res) = user_map.get(lines[4]){
           let pass_provided = sha256::digest(lines[6]) ;
           if *res == pass_provided {
               *is_authenticated = Some(true);
               return "+OK\r\n".to_string();
           }
           return "-WRONGPASS invalid username-password pair or user is disabled\r\n".to_string()
         }
    }

    if lines[2].to_lowercase() == "acl" && lines[4].to_lowercase() == "setuser" && lines[8].contains(">"){
        println!("inside user_map setuser");
        lines[8].to_string();
        let mut final_val = String::new();
        for (_i , char) in lines[8].char_indices() {
            if char == '>' {
                continue
            }
            final_val.push(char)
        }

        let result = sha256::digest(final_val);
        user_map.insert(lines[6].to_string() , result);
        *is_authenticated = Some(true);
        return "+OK\r\n".to_string()
    }

    return "".to_string()
}

/*
   fn raw_server_input(input: &str, map: &mut HashMap<String, HashMapValues>) -> String {
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
   */

