use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;


pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            // or addr: addr
            addr
        }
    }
    //methods take in self as the first argument
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    //For this simmple server 1024 bytes is enough
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
                            // try from is used for converstion of one type to another
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                },
                                Err(e) => {
                                    eprintln!("Failed to parse a request: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read from connection: {}", e);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }

        
    }
}