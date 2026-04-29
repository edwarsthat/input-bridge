use crate::error::CaptureError;
use rdev::{Event, EventType, listen};
use super::monitors::outer_edge_at;

pub fn start() -> Result<(), CaptureError> {
    println!("Escuchando eventos...");
    listen(on_event).map_err(|e| CaptureError::ListenerFailed(format!("{:?}", e)))
}

fn on_event(event: Event) {
    match event.event_type {
        EventType::MouseMove { x, y } => {
            if let Some(edge) = outer_edge_at(x, y) {
                println!("Cursor en borde externo: {:?} ({:.0}, {:.0})", edge, x, y);
                // aquí disparas la lógica de transición entre máquinas
            }
        }
        EventType::ButtonPress(button) => println!("Click: {:?}", button),
        EventType::ButtonRelease(button) => println!("Soltó: {:?}", button),
        _ => {}
    }
}
