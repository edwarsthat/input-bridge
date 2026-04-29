
use tokio::net::UdpSocket;
use crate::error::ws::WsError;
use crate::config::sockets::{CLIENT_IP, CLIENT_PORT};

pub async fn run_client() -> Result<UdpSocket, WsError> {
    let bind_addr = format!("{CLIENT_IP}:{CLIENT_PORT}");
    UdpSocket::bind(&bind_addr).await
        .map_err(|e| WsError::BindFailed(e.to_string()))
}

