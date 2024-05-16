use std::net::TcpListener;
use std::io::Read;

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
                    stream.read(&mut buffer);
                },
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }

        
    }
}