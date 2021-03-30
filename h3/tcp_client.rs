use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:3333") { // 匹配模式
        Ok(mut stream) => { // 连接成功
            println!("Successfully connected to server in port 3333");
            // 消息内容
            let msg = b"Hello";
            // 发送消息
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 5]; // 6字节buffer
            match stream.read_exact(&mut data) {
                Ok(_) => { // 收到回信
                    let text = from_utf8(&data).unwrap();
                    // 判断回信与消息内容是否一致
                    if &data == msg {
                        println!("Reply is: {}", text);
                    } else {
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => { // 未收到回信
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => { // 连接失败
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}