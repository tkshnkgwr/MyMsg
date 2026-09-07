//! # セッション検出モジュール (`session.rs`)
//!
//! Windows Session 0（サービス・非対話セッション）やヘッドレス環境を検出し、
//! GUI が描画できない環境での適切な警告出力およびフォールバック判定を提供します。
//!
//! ## 概要
//!
//! Windows では、サービスやバックグラウンドタスクスケジューラ（「ユーザーがログオンしているかどうかにかかわらず実行する」設定）
//! から起動されたプロセスは **Session 0** と呼ばれる非対話セッションで動作します。
//! この環境下ではウィンドウステーションに表示可能なデスクトップ（`WinSta0`）が存在しないため、
//! GUI ウィンドウ（`eframe` / `egui`）を生成しても画面に描画されません。
//!
//! 本モジュールは、現在のプロセスが対話型セッションで動作しているかを安全に判定し、
//! アプリケーションが自動的にトースト通知モード（`--toast`）へフォールバックできるように支援します。

/// 現在のプロセスがユーザーと対話可能なセッション（対話型デスクトップ）で実行されているかを判定します。
///
/// # プラットフォーム固有の動作
/// - **Windows**:
///   `GetProcessWindowStation` で現在のウィンドウステーションハンドルを取得し、
///   `GetUserObjectInformationW` で `UOI_FLAGS`（`USEROBJECTFLAGS`）を問い合わせます。
///   `dwFlags` に `WSF_VISIBLE` (0x0001) が含まれている場合に対話型（表示可能）と判定します。
/// - **Linux / macOS**:
///   環境変数 `DISPLAY` または `WAYLAND_DISPLAY` が設定されている場合に対話型と判定します。
///
/// # 戻り値
/// - `true`: 対話型デスクトップが利用可能（GUI ポップアップ表示可能）。
/// - `false`: 非対話セッション・ヘッドレス環境（GUI 表示不可、トースト等へのフォールバック推奨）。
pub fn is_interactive_session() -> bool {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::StationsAndDesktops::{
            GetProcessWindowStation, GetUserObjectInformationW, UOI_FLAGS, USEROBJECTFLAGS,
        };

        const WSF_VISIBLE: u32 = 0x0001;

        unsafe {
            let hwinsta = GetProcessWindowStation();
            if hwinsta == 0 {
                return false;
            }

            let mut flags = USEROBJECTFLAGS {
                fInherit: 0,
                fReserved: 0,
                dwFlags: 0,
            };
            let mut needed = 0u32;
            let ok = GetUserObjectInformationW(
                hwinsta,
                UOI_FLAGS,
                &mut flags as *mut _ as *mut _,
                std::mem::size_of::<USEROBJECTFLAGS>() as u32,
                &mut needed,
            );

            if ok == 0 {
                return false;
            }

            // WSF_VISIBLE フラグが立っていれば対話型ウィンドウステーション（WinSta0）
            (flags.dwFlags & WSF_VISIBLE) != 0
        }
    }

    #[cfg(not(windows))]
    {
        // Linux/macOS: DISPLAY または WAYLAND_DISPLAY 環境変数の有無で判定
        std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_interactive_session_callable() {
        // 通常のテスト実行環境（対話型デスクトップ）では true が返ることを確認（またはパニックしないこと）
        let _ = is_interactive_session();
    }
}
