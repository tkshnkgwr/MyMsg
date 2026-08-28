//! # CLI モジュール (`cli.rs`)
//!
//! コマンドライン引数のパース、アイコン/テーマの種別定義、寸法・遅延の計算処理を提供します。

use clap::Parser;
use eframe::egui::Color32;

/// MyMsg のコマンドライン引数構造体
///
/// `clap` の derive マクロを用いて CLI 引数のパースを行います。
#[derive(Parser, Debug, Clone, PartialEq)]
#[command(
    name = "mymsg",
    author = "MyMsg Developer",
    version = "0.1.0",
    about = "最前面メッセージポップアップCLIツール",
    long_about = "低リソース環境向けに最適化された、最前面固定のメッセージポップアップ通知CLIです。EscまたはEnterで即座に閉じられます。"
)]
pub struct CliArgs {
    /// 表示するメッセージ（位置引数）
    #[arg(value_name = "MESSAGE", index = 1)]
    pub message_arg: Option<String>,

    /// 表示するメッセージ（オプション引数 -m / --message）
    #[arg(short = 'm', long = "message", help = "表示するメッセージ文字列")]
    pub message_opt: Option<String>,

    /// ウィンドウサイズ (small: 300x150, medium: 450x220, large: 650x350)
    #[arg(
        short = 's',
        long = "size",
        default_value = "medium",
        help = "ウィンドウサイズ [small, medium, large]"
    )]
    pub size: String,

    /// フォントサイズ (pt単位、省略時はウィンドウサイズから自動算出)
    #[arg(long = "font-size", help = "文字サイズ（pt）")]
    pub font_size: Option<f32>,

    /// メッセージ文字色 (名前・1文字略称・#HEX、省略時はテーマ標準色)
    #[arg(
        short = 'c',
        long = "color",
        help = "文字色 (例: Red, blue, bule, g, #00FFCC)"
    )]
    pub color: Option<String>,

    /// ウィンドウ背景色 (省略時はテーマ標準色)
    #[arg(
        long = "bg-color",
        help = "ウィンドウ背景色 (例: #111111, black, #002244)"
    )]
    pub bg_color: Option<String>,

    /// 文字の点滅表示を有効化（約0.5秒周期で明滅）
    #[arg(short = 'b', long = "blink", help = "メッセージ文字を点滅させる")]
    pub blink: bool,

    /// フォント種別 (1/default/sans, 2/mono, 3/serif, 4/impact)
    #[arg(
        short = 'f',
        long = "font",
        default_value = "default",
        help = "フォントタイプ (default, sans, mono, serif, impact)"
    )]
    pub font: String,

    /// アイコン表示種別 (info, warn, error, ok)
    #[arg(
        short = 'i',
        long = "icon",
        help = "アイコン表示 [info, warn, error, ok]"
    )]
    pub icon: Option<String>,

    /// テーマ設定 (system, dark, light)
    #[arg(
        short = 't',
        long = "theme",
        default_value = "system",
        help = "テーマ設定 [system, dark, light]"
    )]
    pub theme: String,

    /// 表示までの遅延時間（秒単位、0〜3600秒/最大1時間）
    #[arg(
        short = 'd',
        long = "delay",
        default_value_t = 0,
        help = "指定秒数後にポップアップを表示（最大3600秒）"
    )]
    pub delay: u64,
}

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

/// メッセージ文字列の優先度解決およびエスケープ改行の展開を行います。
///
/// 1. 位置引数 (`message_arg`) があれば最優先
/// 2. `-m / --message` (`message_opt`) があれば採用
/// 3. いずれもなければデフォルトの通知テキストを返します。
/// 4. 文字列内の `\n` や `\r\n` エスケープ文字を実際の改行文字に展開します。
pub fn resolve_message(arg: Option<String>, opt: Option<String>) -> String {
    let raw = arg
        .or(opt)
        .unwrap_or_else(|| "MyMsg: 通知が届きました".to_string());
    raw.replace("\\r\\n", "\n").replace("\\n", "\n")
}

/// サイズ指定文字列およびフォントサイズ指定から、適切な文字サイズとウィンドウ寸法を算出します。
///
/// # 戻り値
/// `(font_size, (width, height))`
pub fn calculate_window_dimensions(
    size_str: &str,
    custom_font_size: Option<f32>,
) -> (f32, (f32, f32)) {
    let (default_font_size, dims) = match size_str.trim().to_lowercase().as_str() {
        "small" | "s" => (20.0, (300.0, 150.0)),
        "large" | "l" => (36.0, (650.0, 350.0)),
        _ => (26.0, (450.0, 220.0)), // medium (既定値)
    };

    let final_font_size = custom_font_size.unwrap_or(default_font_size);
    (final_font_size, dims)
}

/// 遅延秒数を安全な範囲（0〜3600秒 = 最大1時間）にクランプします。
pub fn clamp_delay_seconds(delay: u64) -> u64 {
    delay.min(3600)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_message_priority() {
        // 位置引数優先
        let msg1 = resolve_message(Some("位置メッセージ".into()), Some("オプション".into()));
        assert_eq!(msg1, "位置メッセージ");

        // オプション引数
        let msg2 = resolve_message(None, Some("オプションメッセージ".into()));
        assert_eq!(msg2, "オプションメッセージ");

        // デフォルト値
        let msg3 = resolve_message(None, None);
        assert_eq!(msg3, "MyMsg: 通知が届きました");
    }

    #[test]
    fn test_calculate_window_dimensions() {
        // small
        let (font_s, (w_s, h_s)) = calculate_window_dimensions("small", None);
        assert_eq!(font_s, 20.0);
        assert_eq!((w_s, h_s), (300.0, 150.0));

        // medium (既定値)
        let (font_m, (w_m, h_m)) = calculate_window_dimensions("medium", None);
        assert_eq!(font_m, 26.0);
        assert_eq!((w_m, h_m), (450.0, 220.0));

        // large
        let (font_l, (w_l, h_l)) = calculate_window_dimensions("large", None);
        assert_eq!(font_l, 36.0);
        assert_eq!((w_l, h_l), (650.0, 350.0));

        // カスタムフォントサイズの優先
        let (custom_font, _) = calculate_window_dimensions("small", Some(48.0));
        assert_eq!(custom_font, 48.0);
    }

    #[test]
    fn test_clamp_delay_seconds() {
        assert_eq!(clamp_delay_seconds(0), 0);
        assert_eq!(clamp_delay_seconds(30), 30);
        assert_eq!(clamp_delay_seconds(3600), 3600);
        assert_eq!(clamp_delay_seconds(9999), 3600); // 1時間に制限
    }

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
    fn test_resolve_message_newlines() {
        let msg = resolve_message(Some("1行目\\n2行目\\r\\n3行目".into()), None);
        assert_eq!(msg, "1行目\n2行目\n3行目");
    }
}
