use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response{
        println!("Failed to parse request {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    // main constructor, Self and Server is the same as it ref to the struct
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }
    pub fn run(self, mut handler: impl Handler) {
        println!("listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; //initialize memory
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Request::try_from(&buffer as &[u8]);
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                }
                                Err(e) => handler.handle_bad_request(&e)
                            };
                            if let Err(e) = response.send(&mut stream) { // if fail to send response, print error message
                                println!("Failed to send response: {}", e);
                            }
                            // let rest: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read connection, {}", e),
                    }
                }
                Err(e) => println!("Failed connection: {}", e),
            }

            // let res = listener.accept();
            // if res.is_err() {
            //     continue
            // }
            // let stream = res.unwrap();
        }
    }
}
