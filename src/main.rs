#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};
use std::thread;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    // let handle = thread::spawn(move ||
    // {
    for stream in listener.incoming() {
        thread::spawn(move || 
        {
            match stream {
                Ok(mut _stream) => {
                    println!("accepted new connection");
                    let mut buf = [0; 512];

                    loop {
                        let bytes_read = _stream.read(&mut buf).unwrap();
                        if bytes_read == 0 {
                            break;
                        }
                        _stream.write_all(b"+PONG\r\n");
                    }
                    
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
         });
    }
    // });
    // handle.join().unwrap();

}
