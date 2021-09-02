use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:7000").await.unwrap();

    let (tx, _rx) = broadcast::channel::<String>(10);

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

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
                tokio::select! {
                         result = reader.read_line(&mut line) => {
                            if result.unwrap() == 0 {
                                break
                            }

                            tx.send(line.clone());
                            line.clear();
                        }
                        result = rx.recv() => {
                            let msg = result.unwrap();
                            writer.write_all(msg.as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
}
