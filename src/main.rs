use std::{sync::Arc, time::Duration};
use tokio::time::interval;
mod client;
mod config;
mod error;
mod server;

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

    let (tx, mut rx) = tokio::sync::mpsc::channel::<server::monitors::Edge>(32);

    std::thread::spawn(move || {
        if let Err(e) = server::capture::start(tx) {
            eprintln!("Error en capture: {e}");
        }
    });

    let mut client_addr = None;
    let mut buf = vec![0u8; 1024];

    println!("Esperando que el cliente se conecte...");

    loop {
        tokio::select! {
            Some(edge) = rx.recv() => {
                if let Some(addr) = client_addr {
                    let msg = format!("{edge:?}");
                    socket.send_to(msg.as_bytes(), addr).await?;
                    println!("Borde {edge:?} → enviado a {addr}");
                }
            }
            Ok((len, addr)) = socket.recv_from(&mut buf) => {
                let msg = String::from_utf8_lossy(&buf[..len]);
                println!("Cliente conectado desde {addr}: {msg}");
                client_addr = Some(addr);
            }
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
