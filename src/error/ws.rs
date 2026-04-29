use thiserror::Error;

#[derive(Debug, Error)]
pub enum WsError {
    #[error("Error al enlazar el socket UDP: {0}")]
    BindFailed(String),

    #[error("Error al enviar datos por UDP: {0}")]
    SendFailed(String),

    #[error("Error al recibir datos por UDP: {0}")]
    RecvFailed(String),
}
