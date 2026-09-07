//! # サウンド通知モジュール (`sound.rs`)
//!
//! Windows API (`MessageBeep`) または端末ベル制御文字を用いた通知音・ビープ音の再生を提供します。
//!
//! ## 概要
//!
//! コマンドライン引数 `--sound`（または `--beep`）が指定された際に、
//! ポップアップ表示時やトースト通知送信時に OS のシステム通知音を再生します。
//!
//! アイコンオプション（`--icon` / `-i`）が指定されている場合、アイコン種別（情報・警告・エラー・成功）
//! に対応したシステムサウンド（Windows では `MB_ICON*` 定数）を鳴らし分けます。

use crate::cli::IconType;

/// アイコン種別や状況に応じたシステム通知音を再生します。
///
/// # 引数
/// - `icon`: アイコン種別（`Some(IconType)` または `None`）。
///   - `Some(IconType::Info)`: 情報音 (`MB_ICONASTERISK` / アスタリスクチャイム)
///   - `Some(IconType::Warn)`: 警告音 (`MB_ICONEXCLAMATION` / 感嘆符警告音)
///   - `Some(IconType::Error)`: エラー音 (`MB_ICONHAND` / ハンドストップ音)
///   - `Some(IconType::Ok)` または `None`: 標準通知音 (`MB_OK`)
///
/// # プラットフォーム固有の動作
/// - **Windows**: `windows_sys::Win32::System::Diagnostics::Debug::MessageBeep` を呼び出します。
/// - **Linux / macOS**: 標準出力に ASCII 端末ベル文字（`\x07`）を出力します。
pub fn play_notification_sound(icon: Option<IconType>) {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Diagnostics::Debug::MessageBeep;
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            MB_ICONASTERISK, MB_ICONEXCLAMATION, MB_ICONHAND, MB_OK,
        };

        let u_type = match icon {
            Some(IconType::Info) => MB_ICONASTERISK,
            Some(IconType::Warn) => MB_ICONEXCLAMATION,
            Some(IconType::Error) => MB_ICONHAND,
            Some(IconType::Ok) => MB_OK,
            None => MB_OK,
        };

        unsafe {
            MessageBeep(u_type);
        }
    }

    #[cfg(not(windows))]
    {
        let _ = icon;
        print!("\x07");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_sound_does_not_panic() {
        // 音声再生呼び出しがパニックしないことを検証
        play_notification_sound(Some(IconType::Info));
        play_notification_sound(Some(IconType::Warn));
        play_notification_sound(Some(IconType::Error));
        play_notification_sound(Some(IconType::Ok));
        play_notification_sound(None);
    }
}
