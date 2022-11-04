use std::{
    io::{BufRead, Read, Write},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

/* test for FLengthFieldBasedFrameDecoder */
fn main() {
    let server_addr = "127.0.0.1:8080";
    println!("listening ... ...");
    if let Ok(listener) = TcpListener::bind(server_addr) {
        for stream in listener.incoming() {
            println!("get a client!");

            // handle_client(stream.unwrap());
            // 1, 半包
            // half_package(stream.unwrap());
            // 2, 粘包
            // one_half_package(stream.unwrap());
            // 3, 大包
            // oversize_package(stream.unwrap());
            // 4, 错包,少发
            // uncompleted_package(stream.unwrap());
            // 5, 错包,多发
            // extra_unknow_data_package(stream.unwrap());
            // 6, 测试 offset
            with_offset_package(stream.unwrap());
        }
    }
}

// 1, 半包
// FLengthFieldBasedFrameDecoder(128, 0, 2,0,2);
fn half_package(mut stream: TcpStream) {

    let mut buff = [0u8; 4];
    buff[0] = 0;
    buff[1] = 5;
    buff[2] = 105;
    buff[3] = 101;
    let _ = stream.write(&buff).unwrap();
    println!("write half_package!");
    
    sleep(Duration::from_millis(2000));

    let mut buff = [0u8; 3];
    buff[0] = 105;
    buff[1] = 105;
    buff[2] = 105;
    let _ = stream.write(&buff).unwrap();
    println!("write half_package!");

}
// 2, 粘包
// FLengthFieldBasedFrameDecoder(128, 0, 2,0,2);
fn one_half_package(mut stream: TcpStream) {

    let mut buff = [0u8; 4];
    buff[0] = 0;
    buff[1] = 5;
    buff[2] = 105;
    buff[3] = 101;
    let _ = stream.write(&buff).unwrap();
    println!("write half_package!");
    
    sleep(Duration::from_millis(2000));

    let mut buff = [0u8; 10];
    buff[0] = 105;
    buff[1] = 105;
    buff[2] = 105;

    
    buff[3] = 0;
    buff[4] = 5;
    buff[5] = 105;
    buff[6] = 101;
    buff[7] = 105;
    buff[8] = 105;
    buff[9] = 105;

    let _ = stream.write(&buff).unwrap();
    println!("write one_half_package!");



}

// 3, 大包
// FLengthFieldBasedFrameDecoder(128, 0, 2,0,2);
fn oversize_package(mut stream: TcpStream) {

    let mut buff = [0u8; 4];
    buff[0] = 0;
    buff[1] = 5;
    buff[2] = 105;
    buff[3] = 101;
    let _ = stream.write(&buff).unwrap();
    println!("write half_package!");
    
    sleep(Duration::from_millis(2000));

    let mut buff = [0u8; 1024];
    buff[0] = 105;
    buff[1] = 105;
    buff[2] = 105;

    
    buff[3] = 0;
    buff[4] = 5;
    buff[5] = 105;
    buff[6] = 101;
    buff[7] = 105;
    buff[8] = 105;
    buff[9] = 105;

    let _ = stream.write(&buff).unwrap();
    println!("write one_half_big_package!");

}


// 4, 错包,少发
// FLengthFieldBasedFrameDecoder(128, 0, 2,0,2);
fn uncompleted_package(mut stream: TcpStream) {

    let mut buff = [0u8; 4];
    buff[0] = 0;
    buff[1] = 5;
    buff[2] = 105;
    buff[3] = 101;
    let _ = stream.write(&buff).unwrap();
    println!("write uncompleted_package!");

    sleep(Duration::from_millis(2000));

    let mut buff = [0u8; 7];
    
    buff[0] = 0;
    buff[1] = 5;
    buff[2] = 105;
    buff[3] = 101;
    buff[4] = 105;
    buff[5] = 105;
    buff[6] = 105;

    let _ = stream.write(&buff).unwrap();
    println!("write one_package!");


}

// 5, 错包,多发
// FLengthFieldBasedFrameDecoder(128, 0, 2,0,2);
fn extra_unknow_data_package(mut stream: TcpStream) {

    let mut buff = [0u8; 10];
    
    buff[0] = 0;
    buff[1] = 5;
    buff[2] = 105;
    buff[3] = 101;
    buff[4] = 105;
    buff[5] = 105;
    buff[6] = 105;
    buff[7] = 105;
    buff[8] = 105;

    let _ = stream.write(&buff).unwrap();
    println!("write one_package!");
}

// 6, 测试 offset
// FLengthFieldBasedFrameDecoder(128, 2, 2,0,4);
fn with_offset_package(mut stream: TcpStream) {

    let mut buff = [0u8; 9];
    
    buff[0] = 0;
    buff[1] = 5;
    buff[2] = 0;
    buff[3] = 5;
    buff[4] = 105;
    buff[5] = 101;
    buff[6] = 105;
    buff[7] = 105;
    buff[8] = 105;

    let _ = stream.write(&buff).unwrap();
    println!("write one_package!");

}


// 7, 加上 strip
// FLengthFieldBasedFrameDecoder(128, 2, 2,0,2);