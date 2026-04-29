
use tokio::net::UdpSocket;
use crate::error::ws::WsError;
use crate::config::sockets::{HOST_IP, HOST_PORT};

pub async fn run_server() -> Result<UdpSocket, WsError> {
    let bind_addr = format!("{HOST_IP}:{HOST_PORT}");
    match UdpSocket::bind(&bind_addr).await {
        Ok(listener) => {
            println!("Servidor escuchando en {bind_addr}");
            Ok(listener)
        }
        Err(err) => return Err(WsError::BindFailed(err.to_string()))
    };
}
