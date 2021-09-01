use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:7000").await.unwrap();

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        // To avoid blocking on the "task" level
        // Task are units of async computations
        // Like lightweight threads
        // Tokio schedules the tasks for the most efficient processing
        // Rust has async block instead of async functions like Javascript, could be writen as async function but this is fine.
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap();

                if bytes_read == 0 {
                    break;
                }

                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
