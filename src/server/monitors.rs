use std::sync::OnceLock;
use windows::Win32::Foundation::{BOOL, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO,
};

#[derive(Debug, Clone)]
pub struct MonitorRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}

static MONITORS: OnceLock<Vec<MonitorRect>> = OnceLock::new();

pub fn get_monitors() -> &'static Vec<MonitorRect> {
    MONITORS.get_or_init(enumerate_monitors)
}

fn enumerate_monitors() -> Vec<MonitorRect> {
    let mut monitors: Vec<MonitorRect> = Vec::new();

    unsafe extern "system" fn callback(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _rect: *mut RECT,
        lparam: LPARAM,
    ) -> BOOL {
        unsafe {
            let ptr = lparam.0 as *mut Vec<MonitorRect>;
            if ptr.is_null() {
                return BOOL(0); // Detener enumeración
            }
            let list = &mut *ptr;
            let mut info = MONITORINFO {
                cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                ..Default::default()
            };
            if GetMonitorInfoW(hmonitor, &mut info).as_bool() {
                list.push(MonitorRect {
                    left: info.rcMonitor.left,
                    top: info.rcMonitor.top,
                    right: info.rcMonitor.right,
                    bottom: info.rcMonitor.bottom,
                });
            }
        }
        BOOL(1)
    }

    unsafe {
        let _ = EnumDisplayMonitors(
            HDC::default(),
            None,
            Some(callback),
            LPARAM(&mut monitors as *mut _ as isize),
        );
    }

    monitors
}

// Devuelve el borde externo que el cursor está tocando, si aplica.
// "Externo" significa que no hay otro monitor del otro lado.
pub fn outer_edge_at(x: f64, y: f64) -> Option<Edge> {
    let monitors = get_monitors();
    let xi = x as i32;
    let yi = y as i32;

    // Encuentra el monitor donde está el cursor
    let current = monitors
        .iter()
        .find(|m| xi >= m.left && xi < m.right && yi >= m.top && yi < m.bottom)?;

    let candidates = [
        (xi <= current.left, Edge::Left, xi - 1, yi),
        (xi >= current.right - 1, Edge::Right, xi + 1, yi),
        (yi <= current.top, Edge::Top, xi, yi - 1),
        (yi >= current.bottom - 1, Edge::Bottom, xi, yi + 1),
    ];

    for (at_edge, edge, nx, ny) in candidates {
        if at_edge {
            // Verifica que ningún otro monitor cubra el punto adyacente
            let blocked = monitors
                .iter()
                .any(|m| nx >= m.left && nx < m.right && ny >= m.top && ny < m.bottom);
            if !blocked {
                return Some(edge);
            }
        }
    }

    None
}
