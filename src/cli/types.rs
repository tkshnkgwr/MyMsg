//! # CLI 型定義サブモジュール (`cli/types.rs`)
//!
//! アイコン種別、テーマモード、表示モニターターゲット等の列挙型と
//! コマンドライン引数文字列からのパース関数を提供します。

use eframe::egui::Color32;

/// アイコンの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconType {
    /// 情報アイコン (ℹ)
    Info,
    /// 警告アイコン (⚠)
    Warn,
    /// エラー・危険アイコン (✖)
    Error,
    /// 成功・完了アイコン (✔)
    Ok,
}

impl IconType {
    /// アイコンのテキストシンボル
    pub fn symbol(&self) -> &'static str {
        match self {
            IconType::Info => "ℹ",
            IconType::Warn => "⚠",
            IconType::Error => "✖",
            IconType::Ok => "✔",
        }
    }

    /// アイコン固有のデフォルト強調カラー
    pub fn default_color(&self) -> Color32 {
        match self {
            IconType::Info => Color32::from_rgb(56, 189, 248), // スカイブルー（情報）
            IconType::Warn => Color32::from_rgb(251, 191, 36), // アンバー/イエロー（警告）
            IconType::Error => Color32::from_rgb(248, 113, 113), // レッド（エラー）
            IconType::Ok => Color32::from_rgb(74, 222, 128),   // グリーン（成功）
        }
    }
}

/// アイコン文字列を IconType にパースします。
pub fn parse_icon(input: &str) -> Option<IconType> {
    let clean = input.trim().to_lowercase();
    match clean.as_str() {
        "info" | "i" | "information" => Some(IconType::Info),
        "warn" | "warning" | "w" | "alert" => Some(IconType::Warn),
        "error" | "err" | "e" | "danger" | "ng" => Some(IconType::Error),
        "ok" | "success" | "check" | "s" | "k" => Some(IconType::Ok),
        _ => None,
    }
}

/// テーマモード種別
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    /// システム設定に追従 (既定)
    System,
    /// ダークモード固定
    Dark,
    /// ライトモード固定
    Light,
}

/// テーマ文字列を ThemeMode にパースします。
pub fn parse_theme(input: &str) -> ThemeMode {
    match input.trim().to_lowercase().as_str() {
        "dark" | "d" | "black" => ThemeMode::Dark,
        "light" | "l" | "white" => ThemeMode::Light,
        _ => ThemeMode::System,
    }
}

/// モニター指定種別
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitorTarget {
    /// マウスカーソルが存在するモニター (既定値)
    Cursor,
    /// プライマリモニター
    Primary,
    /// 指定されたインデックスのモニター (0始まり)
    Index(usize),
}

/// モニター指定文字列を MonitorTarget にパースします。
pub fn parse_monitor_target(input: &str) -> MonitorTarget {
    let clean = input.trim().to_lowercase();
    match clean.as_str() {
        "cursor" | "c" | "mouse" | "active" => MonitorTarget::Cursor,
        "primary" | "p" | "main" => MonitorTarget::Primary,
        _ => {
            if let Ok(idx) = clean.parse::<usize>() {
                MonitorTarget::Index(idx)
            } else {
                MonitorTarget::Cursor
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_icon() {
        assert_eq!(parse_icon("info"), Some(IconType::Info));
        assert_eq!(parse_icon("i"), Some(IconType::Info));
        assert_eq!(parse_icon("warn"), Some(IconType::Warn));
        assert_eq!(parse_icon("warning"), Some(IconType::Warn));
        assert_eq!(parse_icon("w"), Some(IconType::Warn));
        assert_eq!(parse_icon("error"), Some(IconType::Error));
        assert_eq!(parse_icon("err"), Some(IconType::Error));
        assert_eq!(parse_icon("e"), Some(IconType::Error));
        assert_eq!(parse_icon("ok"), Some(IconType::Ok));
        assert_eq!(parse_icon("success"), Some(IconType::Ok));
        assert_eq!(parse_icon("check"), Some(IconType::Ok));
        assert_eq!(parse_icon("s"), Some(IconType::Ok));
        assert_eq!(parse_icon("unknown"), None);
    }

    #[test]
    fn test_parse_theme() {
        assert_eq!(parse_theme("dark"), ThemeMode::Dark);
        assert_eq!(parse_theme("d"), ThemeMode::Dark);
        assert_eq!(parse_theme("light"), ThemeMode::Light);
        assert_eq!(parse_theme("l"), ThemeMode::Light);
        assert_eq!(parse_theme("system"), ThemeMode::System);
        assert_eq!(parse_theme("sys"), ThemeMode::System);
        assert_eq!(parse_theme("auto"), ThemeMode::System);
    }

    #[test]
    fn test_parse_monitor_target() {
        assert_eq!(parse_monitor_target("cursor"), MonitorTarget::Cursor);
        assert_eq!(parse_monitor_target("c"), MonitorTarget::Cursor);
        assert_eq!(parse_monitor_target("mouse"), MonitorTarget::Cursor);
        assert_eq!(parse_monitor_target("primary"), MonitorTarget::Primary);
        assert_eq!(parse_monitor_target("p"), MonitorTarget::Primary);
        assert_eq!(parse_monitor_target("main"), MonitorTarget::Primary);
        assert_eq!(parse_monitor_target("0"), MonitorTarget::Index(0));
        assert_eq!(parse_monitor_target("1"), MonitorTarget::Index(1));
        assert_eq!(parse_monitor_target("2"), MonitorTarget::Index(2));
        assert_eq!(parse_monitor_target("unknown"), MonitorTarget::Cursor);
    }
}
