#![allow(dead_code)]

use server::Server;
use http::{Request,Method};

mod server;
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}


// example structure of http1.1 request is
// GET path ? query_string HTTP/1.1