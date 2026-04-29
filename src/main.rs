use std::sync::{Arc, OnceLock};

use tokio::net::UdpSocket;

mod error;
mod server;

static SOCKET: OnceLock<Arc<UdpSocket>> = OnceLock::new();

#[tokio::main]
async fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();

    match mode.as_str() {
        "server" => {
            if let Err(e) = run_server_service().await {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        "client" => {
            // run_client_service().await
        }
        _ => {
            eprintln!("Uso: cargo run -- server | client");
            std::process::exit(1);
        }
    }
}

async fn run_server_service() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Arc::new(server::ws::run_server().await?);
    SOCKET.set(socket).ok();

    let mut buf = vec![0u8; 1024];

    loop {
        let (len, addr) = SOCKET.get().unwrap().recv_from(&mut buf).await?;
        println!("Paquete recibido de {addr}: {} bytes", len);
    }
}
