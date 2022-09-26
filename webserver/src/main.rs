#![allow(dead_code)]
use std::{collections::HashMap, str::FromStr};

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    TRACE,
    CONNECT,
}

enum Version {
    HTTP1,
    HTTP2,
}

impl Version {
    fn as_str(&self) -> &'static str {
        match self {
            Version::HTTP1 => "HTTP1/1",
            Version::HTTP2 => "HTTP2",
        }
    }
}

impl FromStr for Version {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "HTTP/1.1" => Ok(Version::HTTP1),
            "HTTP/2" => Ok(Version::HTTP2),
            _ => Err(()),
        }
    }
}

struct Request {
    url: String,
    method: Method,
    version: Version,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    path_params: HashMap<String, String>,
    reader: TcpStream,
}

async fn process_socket(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];

    // stream.(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).await?;

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
