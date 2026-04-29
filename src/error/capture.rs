use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaptureError {
    #[error("Error al iniciar el listener de eventos: {0}")]
    ListenerFailed(String),

    #[error("Permisos insuficientes. En Linux ejecuta: sudo usermod -aG input $USER")]
    PermissionDenied,

    #[error("Wayland no está soportado directamente. Usa X11 o habilita XWayland")]
    WaylandNotSupported,

    #[error("El canal de eventos se cerró inesperadamente")]
    ChannelClosed,
}
