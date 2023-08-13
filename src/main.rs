use std::net::TcpListener;
use std::thread;

mod connection_handler;
pub mod joker;
pub mod messager;
use connection_handler::connection_handler;

fn main() -> std::io::Result<()> {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:8080")?;

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
