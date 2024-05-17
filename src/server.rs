use std::net::TcpListener;
use std::io::{Read,Write};
use crate::http::{Request,Response,StatusCode,ParseError};
use std::convert::TryFrom;

pub trait Handler {
    fn handle_request (&mut self, request: &Request) -> Response;

    fn handle_bad_request (&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    } 
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr
        }
    }
    //methods take in self as the first argument
    pub fn run(self, mut handler: impl Handler) {
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
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                },
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                eprintln!("Failed to send response: {}", e);
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