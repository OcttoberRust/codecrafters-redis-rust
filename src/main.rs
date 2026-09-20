use std::io::Read;
use std::net::TcpListener;
use std::thread;

mod resp;
mod respdispatcher;

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
                            Ok(value) => dispatcher.convert_to_command(value),
                            Err(e) => {
                                eprintln!("parse error: {:?}", e);
                                parser.pos = 0;
                                continue;
                            }
                        };

                        // TODO: dispatch(command) -> RespProtocolDataType,
                        // encode into bytes, then socket.write_all them.

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
