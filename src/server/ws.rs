
use tokio::net::UdpSocket;
use crate::error::ws::WsError;

pub async fn run_server() -> Result<UdpSocket, WsError> {
    let bind_addr = "0.0.0.0:5000";
    match UdpSocket::bind(bind_addr).await {
        Ok(listener) => {
            println!("Servidor escuchando en {bind_addr}");
            return Ok(listener)
        }
        Err(err) => return Err(WsError::BindFailed(err.to_string()))
    };
}
