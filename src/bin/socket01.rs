use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
fn main() {
    let server_addr = "www.baidu.com:80";
    if let Ok(mut stream) = TcpStream::connect(server_addr) {
        stream.write(b"GET / HTTP/1.0\n\n").unwrap();
        let mut buff = [0u8; 4096];
        loop {
            if let Ok(read_size) = stream.read(&mut buff){
                if read_size > 0 {
                    println!("get {:?}", std::str::from_utf8(&buff));
                }
            }
        }
    }
}
