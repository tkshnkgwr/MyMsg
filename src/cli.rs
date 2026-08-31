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

    /// 表示までの遅延時間・時刻指定（秒数: 60, 単位: 10m/1h/10分, 時刻: 12:00、最大24時間）
    #[arg(
        short = 'd',
        long = "delay",
        default_value = "0",
        help = "指定時間・時刻後にポップアップを表示 [例: 60, 10m, 1h, 10分, 12:00]",
        long_help = "ポップアップ表示までの待機時間または指定時刻。\n\
                     ・秒数指定: 60, 300, 3600\n\
                     ・単位指定: 10s(秒), 10m(分), 1h(時間), 10分, 1時間\n\
                     ・時刻指定: 12:00, 17:30:00 (過去の時刻は翌日として計算)\n\
                     ※最大待機時間は24時間 (86400秒) です。"
    )]
    pub delay: String,

    /// 表示先モニターの指定 (cursor, primary, またはモニター番号 0, 1, 2...)
    #[arg(
        long = "monitor",
        default_value = "cursor",
        help = "表示先モニター [cursor, primary, 0, 1, 2...]"
    )]
    pub monitor: String,

    /// 自動消去タイマー（秒単位、0は無効で手動終了まで待機）
    #[arg(
        long = "timeout",
        default_value_t = 0,
        help = "指定秒数経過後に自動でウィンドウを閉じる（0で無効）"
    )]
    pub timeout: u64,

    /// OS標準のトースト通知モード（GUIウィンドウを表示せず通知センター経由で表示）
    #[arg(
        short = 'T',
        long = "toast",
        help = "OS標準のトースト通知として表示（GUI非生成・即時終了）"
    )]
    pub toast: bool,
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
        "small" | "s" => (20.0_f32, (300.0_f32, 150.0_f32)),
        "large" | "l" => (36.0_f32, (650.0_f32, 350.0_f32)),
        _ => (26.0_f32, (450.0_f32, 220.0_f32)), // medium (既定値)
    };

    let final_font_size = custom_font_size.unwrap_or(default_font_size);
    (final_font_size, dims)
}

use chrono::{Local, NaiveTime, Timelike};

/// 遅延指定文字列（秒数, 単位付き, または HH:MM / HH:MM:SS 時刻）を解析し、待機秒数を返します。
pub fn parse_delay_to_seconds(input: &str) -> u64 {
    let now = Local::now().time();
    parse_delay_with_reference(input, now)
}

/// 基準時刻（now）を用いて遅延秒数を算出する内部関数（テスト・検証用）
pub fn parse_delay_with_reference(input: &str, now: NaiveTime) -> u64 {
    let clean = input.trim().to_lowercase();
    if clean.is_empty() || clean == "0" {
        return 0;
    }

    // 1. 純粋な秒数（数値のみ）
    if let Ok(secs) = clean.parse::<u64>() {
        return clamp_delay_seconds(secs);
    }

    // 2. 単位付き指定 (s, m, h, 秒, 分, 時間)
    let unit_multipliers = [
        ("s", 1u64),
        ("m", 60),
        ("h", 3600),
        ("秒", 1),
        ("分", 60),
        ("時間", 3600),
    ];
    for (suffix, mult) in unit_multipliers {
        let parsed = clean
            .strip_suffix(suffix)
            .and_then(|num_str| num_str.trim().parse::<u64>().ok());
        if let Some(num) = parsed {
            return clamp_delay_seconds(num * mult);
        }
    }

    // 3. 時刻指定 (HH:MM または HH:MM:SS)
    if clean.contains(':') {
        let parts: Vec<&str> = clean.split(':').collect();
        if parts.len() == 2 || parts.len() == 3 {
            let h_opt = parts[0].trim().parse::<u32>().ok();
            let m_opt = parts[1].trim().parse::<u32>().ok();
            let s_opt = if parts.len() == 3 {
                parts[2].trim().parse::<u32>().ok()
            } else {
                Some(0)
            };

            let parsed_time = match (h_opt, m_opt, s_opt) {
                (Some(h), Some(m), Some(s)) if h < 24 && m < 60 && s < 60 => {
                    NaiveTime::from_hms_opt(h, m, s)
                }
                _ => None,
            };

            if let Some(target_time) = parsed_time {
                let now_secs = now.num_seconds_from_midnight() as i64;
                let target_secs = target_time.num_seconds_from_midnight() as i64;
                let diff = if target_secs >= now_secs {
                    target_secs - now_secs
                } else {
                    (86400 + target_secs) - now_secs
                };
                return clamp_delay_seconds(diff as u64);
            }
        }
    }

    0
}

/// 遅延秒数を安全な範囲（0〜86400秒 = 最大24時間）にクランプします。
pub fn clamp_delay_seconds(delay: u64) -> u64 {
    delay.min(86400)
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
        assert_eq!(clamp_delay_seconds(86400), 86400);
        assert_eq!(clamp_delay_seconds(99999), 86400); // 24時間に制限
    }

    #[test]
    fn test_parse_delay_with_reference() {
        let now = NaiveTime::from_hms_opt(10, 50, 0).unwrap();

        // 1. 秒数指定
        assert_eq!(parse_delay_with_reference("0", now), 0);
        assert_eq!(parse_delay_with_reference("60", now), 60);
        assert_eq!(parse_delay_with_reference("300", now), 300);

        // 2. 単位指定
        assert_eq!(parse_delay_with_reference("10s", now), 10);
        assert_eq!(parse_delay_with_reference("5m", now), 300);
        assert_eq!(parse_delay_with_reference("2h", now), 7200);
        assert_eq!(parse_delay_with_reference("30秒", now), 30);
        assert_eq!(parse_delay_with_reference("10分", now), 600);
        assert_eq!(parse_delay_with_reference("1時間", now), 3600);

        // 3. 当日の後刻指定 (10:50 -> 11:00 = 10分 = 600秒)
        assert_eq!(parse_delay_with_reference("11:00", now), 600);
        // 秒付き指定 (10:50:00 -> 10:50:30 = 30秒)
        assert_eq!(parse_delay_with_reference("10:50:30", now), 30);

        // 4. 翌日の同時刻指定 (10:50 -> 10:00 = 23時間10分 = 83400秒)
        assert_eq!(parse_delay_with_reference("10:00", now), 83400);
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
