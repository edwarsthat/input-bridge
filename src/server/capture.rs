use crate::error::CaptureError;
use rdev::{Event, EventType, listen};
use tokio::sync::mpsc::Sender;
use super::monitors::{Edge, outer_edge_at};

pub fn start(tx: Sender<Edge>) -> Result<(), CaptureError> {
    println!("Escuchando eventos...");
    listen(move |event| on_event(event, &tx))
        .map_err(|e| CaptureError::ListenerFailed(format!("{:?}", e)))
}

fn on_event(event: Event, tx: &Sender<Edge>) {
    if let EventType::MouseMove { x, y } = event.event_type {
        if let Some(edge) = outer_edge_at(x, y) {
            tx.blocking_send(edge).ok();
        }
    }
}
