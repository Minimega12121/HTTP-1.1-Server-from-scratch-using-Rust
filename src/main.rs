fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Server {
            addr
            // or addr: addr
        }
    }
    //methods take in self as the first argument
    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}