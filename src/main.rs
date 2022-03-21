use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    // Initiate listener
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // Create a channel
    let (tx, _rx) = broadcast::channel(10);
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        // Sender, Cloned cause of moved error
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        // async block
        tokio::spawn(async move {
            //Split the socket to get reader and writer
            let (reader, mut writer) = socket.split();
            // creating a new bufReader
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result  = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, oth_addr) = result.unwrap();
                        if addr != oth_addr {
                            writer.write_all(&msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
