use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    // main constructor, Self and Server is the same as it ref to the struct
    pub fn new(addr: String) -> Self{
        Self {
            addr: addr
        }
    }
    pub fn run(self){
        println!("listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        
        loop {
            match listener.accept() {
                Ok((stream, _)) => {


                }
                Err(e) => println!("Failed connection: {}", e)
            }

            let res = listener.accept();
            if res.is_err() {
                continue
            }
            let stream = res.unwrap();
        }
    }
}