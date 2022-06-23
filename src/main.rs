use server::Server;
use http::Request;
use http::Method;

//every file in rust is treated as a module
mod http;
mod server;

fn main() {
    // &str = string slice = immutable reference to part of a string
    // String
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
