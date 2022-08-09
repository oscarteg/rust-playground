#![allow(dead_code)]
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

enum Method {
    GET,
    HEAD,
    POST,
    DELETE,
    TRACE,
    CONNECT,
}

struct Request {
    method: Method,
}

async fn process_socket(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];

    // stream.(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).await?;

    stream.flush().await?;
    Ok(())
}

const IP: &str = "127.0.0.1:8081";

/// Method Request-URI HTTP-Version CRLF
/// headers CRLF
/// message-body
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(IP).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}
