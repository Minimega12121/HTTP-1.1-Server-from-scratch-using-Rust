#![allow(dead_code)]

use std::env;
use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let default_path = format!("{}/public",env!("CARGO_MANIFEST_DIR"));
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(default_path));
}


// example structure of http1.1 request is
// GET path ? query_string HTTP/1.1