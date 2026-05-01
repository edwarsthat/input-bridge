use std::{sync::{Arc, OnceLock}, time::Duration};
use tokio::{net::UdpSocket, time::interval};
mod client;
mod config;
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
            if let Err(e) = run_client_service().await {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }

        _ => {
            eprintln!("Uso: cargo run -- server | client");
            std::process::exit(1);
        }
    }
}

async fn run_server_service() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Arc::new(server::ws::run_server().await?);
    SOCKET.set(socket.clone()).ok();

    let mut buf = vec![0u8; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Paquete recibido de {addr}: {} bytes", len);
        
        // opcional: imprimir el contenido como texto
        if let Ok(msg) = std::str::from_utf8(&buf[..len]) {
            println!("Contenido: {msg}");
        }
    }
}

async fn run_client_service() -> Result<(), Box<dyn std::error::Error>> {
    let socket = client::ws_client::run_client().await?;
    let target = config::sockets::SERVER_TARGET;

    println!("Cliente listo. Conectando a {target}...");

    let mut ticker = interval(Duration::from_secs(5));
    loop {
        ticker.tick().await;
        match socket.send_to(b"ping", target).await {
            Ok(_) => println!("Ping enviado a {target}"),
            Err(e) => eprintln!("Error enviando a {target}: {e}"),
        }
    }
}
