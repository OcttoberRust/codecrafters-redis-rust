#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};
use std::thread;

use crate::resp::RespParser;
mod resp;
mod respdispatcher;

fn main() {    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        thread::spawn(move || 
        {
            let mut parser = resp::RespParser {buf: [0;512], pos: 0, bytes_read: 0};
            match stream { 
                Ok(mut _stream) => {
                    println!("accepted new connection");

                    loop {
                        let bytes_read = _stream.read(&mut parser.buf).unwrap();
                        eprintln!("read {} bytes: {:?}", bytes_read, &parser.buf[..bytes_read]);                        
                        if bytes_read == 0 {
                            break;
                        }

                        if parser.buf[0] != b'*' {
                            //invalid array
                        }

                        parser.bytes_read = bytes_read;

                        match parser.parse_input() {
                            Ok(value) => {
                                //parser.dispatch();
                            }
                            Err(e) => {
                                eprintln!("parse error: {:?}", e);
                                parser.pos = 0;
                                continue;
                            }
                        };
                        parser.pos = 0;
                    }
                    
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
         });
    }

}
