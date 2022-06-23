use std::io::Read;
use std::net::TcpListener;

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

        //unwrap returns the value or panics
        //the address could already b in use, so this error is unrecoverable
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept(){
                Ok((stream, addr)) => {
                    // stream.read();
                }
                Err(e) => println!("Filed to establish a connection: {}", e)
            }

        }
    }
}
