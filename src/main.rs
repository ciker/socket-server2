use std::env;
use std::net::TcpListener;
use std::thread;

mod connection_handler;
mod joker;
mod messager;
mod utils;
use connection_handler::connection_handler;

fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap();
    let listener: TcpListener = TcpListener::bind(["0.0.0.0:", &port].join(""))?;

    println!("Server listening on port:{}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    connection_handler(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {:?}", e);
            }
        }
    }

    Ok(())
}
