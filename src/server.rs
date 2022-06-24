use std::io::Read;
use std::net::TcpListener;
use crate::http::Request;
use std::convert::TryFrom;

pub struct Server {
    addr: String,
}


impl Server {
    pub fn new(addr: String) -> Self {
        // associated function
        Server {
            addr
        }
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        // method. self takes the ownership of the struct, so the struct will b destroyed after the method execution

        // unwrap returns the value or panics
        // the address could already b in use, so this error is unrecoverable
        let listener = TcpListener::bind(&self.addr).unwrap();
        let mut buffer = [0; 1024]; // creates an array of size 1024 filled with zeros. type u8, so a bit more than 1KB
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // &buffer[..] converts from &[u8; 1024] to &[u8]. Another way to convert: &buffer as &[u8]
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Filed to establish a connection: {}", e),
            }
        }
    }
}
