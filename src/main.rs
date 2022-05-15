use server::Server;
use http::Request;
use http::Method;

mod http;
mod server;


fn main() {
    // let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..]; // give me everything after 10 bytes and not CHAR
    // let string_literal = "1234"; // string slice and is immutable and known at compile time
    // dbg!(string_slice);
    // dbg!(string);
    // println!("Hello, world!");
    let get = Method::GET;
    let delete = Method::DELETE;
    let put = Method::PUT;
    let post = Method::POST;
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

