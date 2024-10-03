mod poker;
mod server;
mod client;

use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "server" {
        println!("Starting server...");
        server::run_server().await.unwrap();
    } else {
        let player_name = if args.len() > 1 { &args[1] } else { "Player" };
        println!("Starting client as {}...", player_name);
        client::run_client(player_name).await.unwrap();
    }
}
