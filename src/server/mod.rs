use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    player: String,
    content: String,
}

pub async fn run_server() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (socket, _) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            handle_connection(socket, tx, rx).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, tx: broadcast::Sender<Message>, mut rx: broadcast::Receiver<Message>) {
    let mut buffer = [0; 1024];

    loop {
        tokio::select! {
            result = socket.read(&mut buffer) => {
                if let Ok(n) = result {
                    if n == 0 { break; }
                    let msg: Message = serde_json::from_slice(&buffer[..n]).unwrap();
                    tx.send(msg).unwrap();
                }
            }
            result = rx.recv() => {
                if let Ok(msg) = result {
                    let msg = serde_json::to_string(&msg).unwrap();
                    socket.write_all(msg.as_bytes()).await.unwrap();
                }
            }
        }
    }
}
