use crate::joker::joker;
use crate::messager::Messager;
use std::net::{SocketAddr, TcpStream};

pub fn connection_handler(stream: TcpStream) {
    let adress: SocketAddr = stream.peer_addr().unwrap();
    println!("New connection: {}", &adress);

    let buffer: [u8; 1024] = [0; 1024];

    let messager = Messager::new(stream, buffer);

    joker(messager);
}
