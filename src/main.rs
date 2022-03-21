use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    // Initiate listener
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    //Connect Multiple Clients
    let (tx, _rx) = broadcast::channel::<String>(10);
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        // async block
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result  = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send(line.clone()).unwrap();
                        line.clear();
                    }
                    result = rx.recv() =>{
                        let msg = result.unwrap();
                        writer.write_all(&msg.as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
}
