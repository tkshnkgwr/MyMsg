//! # マルチモニター管理モジュール (`monitor.rs`)
//!
//! Windows Native API (`GetCursorPos`, `MonitorFromPoint`, `GetMonitorInfoW`) を使用して
//! 現在マウスカーソルが存在するモニターの作業領域（タスクバーを除外した実効領域）の中央座標を算出します。

/// 現在のマウスカーソルが存在するモニターの作業領域（タスクバーを除いた領域）の
/// 物理ピクセル中央座標を返します。winit の with_position() に直接渡せる単位です。
#[cfg(windows)]
pub fn get_active_monitor_center_position(
    window_width: f32,
    window_height: f32,
) -> Option<[f32; 2]> {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;

    unsafe {
        let mut cursor_pos = POINT { x: 0, y: 0 };
        if GetCursorPos(&mut cursor_pos) == 0 {
            return None;
        }

        let h_monitor = MonitorFromPoint(cursor_pos, MONITOR_DEFAULTTONEAREST);
        if h_monitor == 0 {
            return None;
        }

        let mut monitor_info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: std::mem::zeroed(),
            rcWork: std::mem::zeroed(),
            dwFlags: 0,
        };
        if GetMonitorInfoW(h_monitor, &mut monitor_info) == 0 {
            return None;
        }

        // rcWork は仮想デスクトップ上の物理ピクセル座標
        // winit の with_position([x, y]) もこれと同じ物理ピクセル単位を受け付ける
        let left = monitor_info.rcWork.left as f32;
        let top = monitor_info.rcWork.top as f32;
        let work_w = (monitor_info.rcWork.right - monitor_info.rcWork.left) as f32;
        let work_h = (monitor_info.rcWork.bottom - monitor_info.rcWork.top) as f32;

        let pos_x = left + (work_w - window_width) / 2.0;
        let pos_y = top + (work_h - window_height) / 2.0;

        Some([pos_x, pos_y])
    }
}

/// 非Windows環境用フォールバック
#[cfg(not(windows))]
pub fn get_active_monitor_center_position(
    _window_width: f32,
    _window_height: f32,
) -> Option<[f32; 2]> {
    None
}
