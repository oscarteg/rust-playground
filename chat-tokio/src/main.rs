use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:7000").await.unwrap();

    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // To avoid blocking on the "task" level
        // Task are units of async computations
        // Like lightweight threads
        // Tokio schedules the tasks for the most efficient processing
        // Rust has async block instead of async functions like Javascript, could be writen as async function but this is fine.

        //
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            //// Example of using spawn to write message
            // This does not work because the socket does not live long enough to be used
            // Little vague...

            // tokio::spawn(async move {
            //     reader.read_line(&mut line).await.unwrap();
            // });
            //
            // tx.send(("".to_string(), addr)).unwrap();
            //// End example

            loop {
                // Another way to do concurrency
                // Select is that need to operate in the same shared state and have finite number of things
                tokio::select! {
                         result = reader.read_line(&mut line) => {
                            if result.
                                break
                            }


                            if let Err(e) = tx.send((line.clone(), addr)) {
                                panic!("Something whent wrong: {}", e);
                            }
                            line.clear();
                        }
                        result = rx.recv() => {
                            let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
