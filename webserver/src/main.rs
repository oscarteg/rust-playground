use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

use std::io;

async fn process_socket(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0; 1024];
    socket.write_all(b"hello world").await?;
    Ok(())
}

const IP: &str = "127.0.0.1:8081";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(IP).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}
