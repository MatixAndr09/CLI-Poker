use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    player: String,
    content: String,
}

pub async fn run_client(player_name: &str) -> tokio::io::Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await?;
    let mut buffer = [0; 1024];

    // Przykład: wysyłanie wiadomości
    let msg = Message {
        player: player_name.to_string(),
        content: "Join the game".to_string(),
    };

    let msg = serde_json::to_string(&msg).unwrap();
    socket.write_all(msg.as_bytes()).await?;

    // Odczytywanie wiadomości od serwera
    loop {
        let n = socket.read(&mut buffer).await?;
        if n == 0 {
            break;
        }

        let msg: Message = serde_json::from_slice(&buffer[..n]).unwrap();
        println!("Received: {:?}", msg);
    }

    Ok(())
}
