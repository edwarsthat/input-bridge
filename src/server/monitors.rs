use std::sync::OnceLock;

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

// ── Windows ──────────────────────────────────────────────────────────────────

#[cfg(windows)]
fn enumerate_monitors() -> Vec<MonitorRect> {
    use windows::Win32::Foundation::{BOOL, LPARAM, RECT};
    use windows::Win32::Graphics::Gdi::{
        EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO,
    };

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
                return BOOL(0);
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

// ── Linux (X11 / XWayland) ───────────────────────────────────────────────────

#[cfg(target_os = "linux")]
fn enumerate_monitors() -> Vec<MonitorRect> {
    use x11rb::connection::Connection;
    use x11rb::errors::ReplyOrIdError;
    use x11rb::protocol::randr::ConnectionExt as RandrExt;

    let Ok((conn, screen_num)) = x11rb::connect(None) else {
        return linux_fallback();
    };

    let root = conn.setup().roots[screen_num].root;

    let result = conn
        .randr_get_monitors(root, true)
        .map_err(ReplyOrIdError::from)
        .and_then(|cookie| cookie.reply().map_err(ReplyOrIdError::from));
    match result {
        Ok(reply) => reply
            .monitors
            .iter()
            .map(|m| MonitorRect {
                left: m.x as i32,
                top: m.y as i32,
                right: m.x as i32 + m.width as i32,
                bottom: m.y as i32 + m.height as i32,
            })
            .collect(),
        Err(e) => {
            eprintln!("RandR error: {e}. Usando monitor de respaldo.");
            linux_fallback()
        }
    }
}

#[cfg(target_os = "linux")]
fn linux_fallback() -> Vec<MonitorRect> {
    // Si no hay conexión X11 (p. ej. Wayland puro sin XWayland), asumimos
    // un monitor estándar. Considera ejecutar con DISPLAY=:0 o habilitar XWayland.
    eprintln!("No se pudo conectar a X11. Usando resolución de respaldo 1920×1080.");
    vec![MonitorRect {
        left: 0,
        top: 0,
        right: 1920,
        bottom: 1080,
    }]
}

// ── Lógica compartida ────────────────────────────────────────────────────────

/// Devuelve el borde externo que el cursor está tocando, si aplica.
/// "Externo" significa que no hay otro monitor del otro lado.
pub fn outer_edge_at(x: f64, y: f64) -> Option<Edge> {
    let monitors = get_monitors();
    let xi = x as i32;
    let yi = y as i32;

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
