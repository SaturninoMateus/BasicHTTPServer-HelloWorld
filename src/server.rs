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

        let listener = TcpListener::bind(&self.addr).unwrap(); //unwrap returns the value or panics

    }
}
