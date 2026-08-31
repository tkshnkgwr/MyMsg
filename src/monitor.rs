//! # マルチモニター管理モジュール (`monitor.rs`)
//!
//! Windows Native API (`EnumDisplayMonitors`, `GetCursorPos`, `MonitorFromPoint`, `GetMonitorInfoW`) を使用して
//! 指定されたモニター（カーソル位置、プライマリ、指定インデックス）の作業領域（タスクバーを除外した実効領域）の
//! 中央座標を算出します。

use crate::cli::MonitorTarget;

#[cfg(windows)]
use windows_sys::Win32::Foundation::{BOOL, LPARAM, POINT, RECT};
#[cfg(windows)]
use windows_sys::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, GetMonitorInfoW, MonitorFromPoint, HDC, HMONITOR, MONITORINFO,
    MONITOR_DEFAULTTONEAREST, MONITOR_DEFAULTTOPRIMARY,
};
#[cfg(windows)]
use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;

#[cfg(windows)]
const MONITORINFOF_PRIMARY: u32 = 1;

#[cfg(windows)]
unsafe extern "system" fn monitor_enum_proc(
    h_monitor: HMONITOR,
    _hdc: HDC,
    _lprect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    unsafe {
        let list = &mut *(lparam as *mut Vec<MONITORINFO>);
        let mut info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: std::mem::zeroed(),
            rcWork: std::mem::zeroed(),
            dwFlags: 0,
        };
        if GetMonitorInfoW(h_monitor, &mut info) != 0 {
            list.push(info);
        }
    }
    1 // TRUE (列挙継続)
}

#[cfg(windows)]
fn get_all_monitors() -> Vec<MONITORINFO> {
    let mut list: Vec<MONITORINFO> = Vec::new();
    unsafe {
        EnumDisplayMonitors(
            0,
            std::ptr::null(),
            Some(monitor_enum_proc),
            &mut list as *mut _ as LPARAM,
        );
    }
    list
}

#[cfg(windows)]
fn calculate_center(rc_work: &RECT, window_width: f32, window_height: f32) -> [f32; 2] {
    let left = rc_work.left as f32;
    let top = rc_work.top as f32;
    let work_w = (rc_work.right - rc_work.left) as f32;
    let work_h = (rc_work.bottom - rc_work.top) as f32;

    let pos_x = left + (work_w - window_width) / 2.0;
    let pos_y = top + (work_h - window_height) / 2.0;

    [pos_x, pos_y]
}

/// 指定されたモニターターゲットの作業領域（タスクバーを除いた領域）の
/// 物理ピクセル中央座標を返します。winit の with_position() に直接渡せる単位です。
#[cfg(windows)]
pub fn get_monitor_center_position(
    target: MonitorTarget,
    window_width: f32,
    window_height: f32,
) -> Option<[f32; 2]> {
    unsafe {
        match target {
            MonitorTarget::Cursor => {
                let mut cursor_pos = POINT { x: 0, y: 0 };
                if GetCursorPos(&mut cursor_pos) == 0 {
                    return get_monitor_center_position(
                        MonitorTarget::Primary,
                        window_width,
                        window_height,
                    );
                }

                let h_monitor = MonitorFromPoint(cursor_pos, MONITOR_DEFAULTTONEAREST);
                if h_monitor == 0 {
                    return get_monitor_center_position(
                        MonitorTarget::Primary,
                        window_width,
                        window_height,
                    );
                }

                let mut monitor_info = MONITORINFO {
                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                    rcMonitor: std::mem::zeroed(),
                    rcWork: std::mem::zeroed(),
                    dwFlags: 0,
                };
                if GetMonitorInfoW(h_monitor, &mut monitor_info) != 0 {
                    Some(calculate_center(
                        &monitor_info.rcWork,
                        window_width,
                        window_height,
                    ))
                } else {
                    get_monitor_center_position(MonitorTarget::Primary, window_width, window_height)
                }
            }
            MonitorTarget::Primary => {
                let monitors = get_all_monitors();
                // 1. MONITORINFOF_PRIMARY フラグを持つモニターを検索
                if let Some(primary) = monitors
                    .iter()
                    .find(|m| (m.dwFlags & MONITORINFOF_PRIMARY) != 0)
                {
                    return Some(calculate_center(
                        &primary.rcWork,
                        window_width,
                        window_height,
                    ));
                }

                // 2. 原点 (0,0) を含むモニター
                let origin = POINT { x: 0, y: 0 };
                let h_monitor = MonitorFromPoint(origin, MONITOR_DEFAULTTOPRIMARY);
                if h_monitor != 0 {
                    let mut monitor_info = MONITORINFO {
                        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                        rcMonitor: std::mem::zeroed(),
                        rcWork: std::mem::zeroed(),
                        dwFlags: 0,
                    };
                    if GetMonitorInfoW(h_monitor, &mut monitor_info) != 0 {
                        return Some(calculate_center(
                            &monitor_info.rcWork,
                            window_width,
                            window_height,
                        ));
                    }
                }

                // 3. 最初に見つかったモニターでフォールバック
                monitors
                    .first()
                    .map(|m| calculate_center(&m.rcWork, window_width, window_height))
            }
            MonitorTarget::Index(index) => {
                let monitors = get_all_monitors();
                if let Some(m) = monitors.get(index) {
                    Some(calculate_center(&m.rcWork, window_width, window_height))
                } else if !monitors.is_empty() {
                    // インデックスが範囲外の場合はプライマリまたは0番目にフォールバック
                    Some(calculate_center(
                        &monitors[0].rcWork,
                        window_width,
                        window_height,
                    ))
                } else {
                    None
                }
            }
        }
    }
}

/// 既存互換用：カーソル位置のモニター中央座標を取得
#[allow(dead_code)]
pub fn get_active_monitor_center_position(
    window_width: f32,
    window_height: f32,
) -> Option<[f32; 2]> {
    get_monitor_center_position(MonitorTarget::Cursor, window_width, window_height)
}

/// 非Windows環境用フォールバック
#[cfg(not(windows))]
pub fn get_monitor_center_position(
    _target: MonitorTarget,
    _window_width: f32,
    _window_height: f32,
) -> Option<[f32; 2]> {
    None
}
