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
        // method. self takes the ownership of the struct
    }
}
