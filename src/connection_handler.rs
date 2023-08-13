#![allow(unused_mut)]
use crate::joker::joker;
use crate::messager::Messager;
use std::net::{SocketAddr, TcpStream};

pub fn connection_handler(mut stream: TcpStream) {
    let adress: SocketAddr = stream.peer_addr().unwrap();
    println!("New connection: {}", &adress);

    let mut buffer: [u8; 1024] = [0; 1024];

    let mut messager = Messager::new(stream, buffer);

    joker(messager);
}
