use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

mod resp;
mod respdispatcher;
mod respencoder;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for incoming in listener.incoming() {
        thread::spawn(move || {
            let mut parser = resp::RespParser {
                buf: [0; 512],
                pos: 0,
                bytes_read: 0,
            };
            let mut dispatcher = respdispatcher::RespDispatcher {};
            let mut encoder = respencoder::RespEncoder {};

            match incoming {
                Ok(mut socket) => {
                    println!("accepted new connection");

                    loop {
                        let bytes_read = socket.read(&mut parser.buf).unwrap();
                        eprintln!("read {} bytes: {:?}", bytes_read, &parser.buf[..bytes_read]);

                        if bytes_read == 0 {
                            break;
                        }

                        parser.bytes_read = bytes_read;

                        let command = match parser.parse_input() {
                            Ok(value) => {
                                match dispatcher.convert_to_command(value) {
                                    Ok(value) => value,
                                    Err(e) => {
                                        eprintln!("parse error: {:?}", e);
                                        parser.pos = 0;
                                        continue;
                                    }
                                }
                            },
                            Err(e) => {
                                eprintln!("parse error: {:?}", e);
                                parser.pos = 0;
                                continue;
                            }
                        };

                        let value_to_encode = dispatcher.dispatch(command);
                        
                        let encoded_value = match encoder.encode(value_to_encode) {
                            Ok(value) => value,
                            Err(e) => {
                                eprintln!("encoding error: {:?}", e);
                                parser.pos = 0;
                                continue;
                            }
                        };

                        parser.pos = 0;

                        let result = socket.write_all(&encoded_value);
                        result.expect("TBD")

                    }
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
    }
}
