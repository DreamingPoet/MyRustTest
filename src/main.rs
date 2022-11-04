

mod connection;
mod UserModels;
/// Error returned by most functions.
///
/// When writing a real application, one might want to consider a specialized
/// error handling crate or defining an error type as an `enum` of causes.
/// However, for our example, using a boxed `std::error::Error` is sufficient.
///
/// For performance reasons, boxing is avoided in any hot path. For example, in
/// `parse`, a custom error `enum` is defined. This is because the error is hit
/// and handled during normal execution when a partial frame is received on a
/// socket. `std::error::Error` is implemented for `parse::Error` which allows
/// it to be converted to `Box<dyn std::error::Error>`.
pub type Error = Box<dyn std::error::Error + Send + Sync>;
/// A specialized `Result` type for mini-redis operations.
///
/// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, Error>;



use std::{
    io::{BufRead, Read, Write},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

use protobuf::Message;
use UserModels::LoginReq;

#[tokio::main]
async fn main() {
    println!("start run");
    loop {
        
    }
    // let server_addr = "127.0.0.1:8080";
    // if let Ok(listener) = TcpListener::bind(server_addr) {
    //     // accept connections and process them serially
    //     for stream in listener.incoming() {
    //         handle_client(stream.unwrap());
    //     }
    // }
}