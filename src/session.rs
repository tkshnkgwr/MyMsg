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

/// タスクスケジューラやエクスプローラーから直接起動された際、
/// OSによって自動生成された単独コンソール（黒いDOS窓）を自動的に解放・消去します。
/// PowerShell や cmd から起動された場合は既存コンソールを維持し、
/// 通常のコンソールCLIとして同期実行・プロンプト即時復帰を実現します。
pub fn auto_detach_console_if_standalone() {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Console::{FreeConsole, GetConsoleProcessList};

        unsafe {
            let mut process_list = [0u32; 2];
            let count = GetConsoleProcessList(process_list.as_mut_ptr(), 2);
            // コンソールに属するプロセスが自分自身のみ（1つだけ）の場合は単独起動されたDOS窓なので閉じる
            if count == 1 {
                FreeConsole();
            }
        }
    }
}

/// 現在のプロセスがユーザーと対話可能なセッション（対話型デスクトップ）で実行されているかを判定します。
pub fn is_interactive_session() -> bool {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::StationsAndDesktops::{
            GetProcessWindowStation, GetThreadDesktop, GetUserObjectInformationW, UOI_FLAGS,
            UOI_NAME, USEROBJECTFLAGS,
        };
        use windows_sys::Win32::System::Threading::GetCurrentThreadId;

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
            if (flags.dwFlags & WSF_VISIBLE) == 0 {
                return false;
            }

            // 現在のスレッドがアタッチされているデスクトップ名を取得
            let hdesk = GetThreadDesktop(GetCurrentThreadId());
            if hdesk == 0 {
                return false;
            }

            let mut desktop_name = [0u16; 256];
            let mut dlen = 0u32;
            if GetUserObjectInformationW(
                hdesk,
                UOI_NAME,
                desktop_name.as_mut_ptr() as *mut _,
                (desktop_name.len() * 2) as u32,
                &mut dlen,
            ) == 0
            {
                return false;
            }

            let name = String::from_utf16_lossy(&desktop_name[..dlen as usize / 2])
                .trim_matches('\0')
                .to_string();

            // ユーザーが見ているアクティブなデスクトップ ("Default") の場合のみ対話型と判定
            name.eq_ignore_ascii_case("default")
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
