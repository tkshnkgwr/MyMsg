//! # カラー＆テーマパレットモジュール (`color.rs`)
//!
//! Web標準色名、日本語名、タイポ補正、HEXコードのパース処理および
//! ダーク/ライト/システムテーマに基づくカラーパレット解決処理を提供します。

use crate::cli::ThemeMode;
use eframe::egui::Color32;

/// UIカラーパレット
#[derive(Debug, Clone, PartialEq)]
pub struct ThemePalette {
    pub bg_color: Color32,
    pub text_color: Color32,
    pub button_bg: Color32,
    pub button_text: Color32,
    pub button_stroke: Color32,
}

impl ThemePalette {
    /// ダークテーマパレット
    pub fn dark() -> Self {
        Self {
            bg_color: Color32::from_rgb(26, 27, 38),
            text_color: Color32::from_rgb(240, 240, 240),
            button_bg: Color32::from_rgb(45, 48, 65),
            button_text: Color32::from_rgb(200, 200, 200),
            button_stroke: Color32::from_rgb(80, 85, 110),
        }
    }

    /// ライトテーマパレット
    pub fn light() -> Self {
        Self {
            bg_color: Color32::from_rgb(248, 250, 252),
            text_color: Color32::from_rgb(15, 23, 42),
            button_bg: Color32::from_rgb(226, 232, 240),
            button_text: Color32::from_rgb(51, 65, 85),
            button_stroke: Color32::from_rgb(203, 213, 225),
        }
    }
}

/// テーマモードとシステム状態、ユーザー指定色から最終パレットを解決します。
pub fn resolve_theme_palette(
    theme_mode: ThemeMode,
    is_dark_system: bool,
    custom_text_color: Option<&str>,
    custom_bg_color: Option<&str>,
) -> ThemePalette {
    let is_dark = match theme_mode {
        ThemeMode::Dark => true,
        ThemeMode::Light => false,
        ThemeMode::System => is_dark_system,
    };

    let mut palette = if is_dark {
        ThemePalette::dark()
    } else {
        ThemePalette::light()
    };

    // ユーザー指定文字色のオーバーライド
    if let Some(c) = custom_text_color.and_then(parse_color) {
        palette.text_color = c;
    }

    // ユーザー指定背景色のオーバーライド
    if let Some(bg) = custom_bg_color.and_then(parse_color) {
        palette.bg_color = bg;
    }

    palette
}

/// カラー文字列（名前・1文字略称・HEX）をColor32にパースします。
///
/// - 大文字小文字不問 (`"Red"`, `"green"`)
/// - 1文字略称 (`"r"`, `"g"`, `"b"`, `"y"`, `"w"`, `"k"`)
/// - よくあるタイポ補正 (`"bule"` -> 青)
/// - HEXコード (`"#RRGGBB"`, `"#RGB"`, `"#RRGGBBAA"`, `#`なし可)
///
/// # Examples
/// ```
/// use egui::Color32;
/// assert_eq!(parse_color("red"), Some(Color32::from_rgb(239, 68, 68)));
/// assert_eq!(parse_color("Bule"), Some(Color32::from_rgb(59, 130, 246)));
/// assert_eq!(parse_color("#FFFFFF"), Some(Color32::from_rgb(255, 255, 255)));
/// ```
pub fn parse_color(input: &str) -> Option<Color32> {
    let clean = input.trim().to_lowercase();
    match clean.as_str() {
        // 基本色 ＆ 1文字略称
        "red" | "r" | "赤" => Some(Color32::from_rgb(239, 68, 68)),
        "green" | "g" | "緑" => Some(Color32::from_rgb(34, 197, 94)),
        "blue" | "bule" | "b" | "青" => Some(Color32::from_rgb(59, 130, 246)),
        "yellow" | "y" | "黄" => Some(Color32::from_rgb(234, 179, 8)),
        "orange" | "o" => Some(Color32::from_rgb(249, 115, 22)),
        "purple" | "p" => Some(Color32::from_rgb(168, 85, 247)),
        "cyan" | "c" => Some(Color32::from_rgb(6, 182, 212)),
        "pink" | "magenta" | "m" => Some(Color32::from_rgb(236, 72, 153)),
        "white" | "w" | "白" => Some(Color32::from_rgb(255, 255, 255)),
        "black" | "k" | "黒" => Some(Color32::from_rgb(10, 10, 15)),

        // 拡張パレット色
        "lime" => Some(Color32::from_rgb(132, 204, 22)),
        "gold" => Some(Color32::from_rgb(250, 204, 21)),
        "amber" => Some(Color32::from_rgb(245, 158, 11)),
        "emerald" => Some(Color32::from_rgb(16, 185, 129)),
        "teal" => Some(Color32::from_rgb(20, 184, 166)),
        "sky" => Some(Color32::from_rgb(14, 165, 233)),
        "indigo" => Some(Color32::from_rgb(99, 102, 241)),
        "violet" => Some(Color32::from_rgb(139, 92, 246)),
        "rose" => Some(Color32::from_rgb(244, 63, 94)),
        "crimson" => Some(Color32::from_rgb(220, 20, 60)),
        "navy" => Some(Color32::from_rgb(15, 23, 42)),
        "gray" | "grey" | "gray500" => Some(Color32::from_rgb(148, 163, 184)),
        "dark" | "darkgray" => Some(Color32::from_rgb(30, 41, 59)),
        "light" | "lightgray" => Some(Color32::from_rgb(226, 232, 240)),
        _ => {
            // #RRGGBB または #RGB または RRGGBB をパース
            let hex = clean.strip_prefix('#').unwrap_or(&clean);
            if hex.len() == 6 {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Color32::from_rgb(r, g, b))
            } else if hex.len() == 3 {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                Some(Color32::from_rgb(r, g, b))
            } else if hex.len() == 8 {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Color32::from_rgba_unmultiplied(r, g, b, a))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color_named_and_typo() {
        // 基本名
        assert_eq!(parse_color("red"), Some(Color32::from_rgb(239, 68, 68)));
        assert_eq!(parse_color("RED"), Some(Color32::from_rgb(239, 68, 68)));
        assert_eq!(parse_color("green"), Some(Color32::from_rgb(34, 197, 94)));

        // typo補正 (bule -> blue)
        assert_eq!(parse_color("bule"), Some(Color32::from_rgb(59, 130, 246)));
        assert_eq!(parse_color("Bule"), Some(Color32::from_rgb(59, 130, 246)));

        // 1文字略称
        assert_eq!(parse_color("r"), Some(Color32::from_rgb(239, 68, 68)));
        assert_eq!(parse_color("g"), Some(Color32::from_rgb(34, 197, 94)));
        assert_eq!(parse_color("b"), Some(Color32::from_rgb(59, 130, 246)));
        assert_eq!(parse_color("y"), Some(Color32::from_rgb(234, 179, 8)));
        assert_eq!(parse_color("w"), Some(Color32::from_rgb(255, 255, 255)));
        assert_eq!(parse_color("k"), Some(Color32::from_rgb(10, 10, 15)));
    }

    #[test]
    fn test_parse_color_hex() {
        // 6桁 HEX
        assert_eq!(
            parse_color("#FFFFFF"),
            Some(Color32::from_rgb(255, 255, 255))
        );
        assert_eq!(parse_color("00E5FF"), Some(Color32::from_rgb(0, 229, 255)));

        // 3桁 HEX
        assert_eq!(parse_color("#F00"), Some(Color32::from_rgb(255, 0, 0)));
        assert_eq!(parse_color("0F0"), Some(Color32::from_rgb(0, 255, 0)));

        // 8桁 HEX (RGBA)
        assert_eq!(
            parse_color("#FFFFFF80"),
            Some(Color32::from_rgba_unmultiplied(255, 255, 255, 128))
        );

        // 不正値
        assert_eq!(parse_color("invalid_color_xyz"), None);
    }

    #[test]
    fn test_resolve_theme_palette() {
        // ダークテーマの明示的指定
        let p_dark = resolve_theme_palette(ThemeMode::Dark, false, None, None);
        assert_eq!(p_dark.bg_color, Color32::from_rgb(26, 27, 38));
        assert_eq!(p_dark.text_color, Color32::from_rgb(240, 240, 240));

        // ライトテーマの明示的指定
        let p_light = resolve_theme_palette(ThemeMode::Light, true, None, None);
        assert_eq!(p_light.bg_color, Color32::from_rgb(248, 250, 252));
        assert_eq!(p_light.text_color, Color32::from_rgb(15, 23, 42));

        // システムテーマ設定への自動追従
        let p_sys_dark = resolve_theme_palette(ThemeMode::System, true, None, None);
        assert_eq!(p_sys_dark.bg_color, Color32::from_rgb(26, 27, 38));

        let p_sys_light = resolve_theme_palette(ThemeMode::System, false, None, None);
        assert_eq!(p_sys_light.bg_color, Color32::from_rgb(248, 250, 252));

        // ユーザー指定色の最優先オーバーライド
        let p_custom = resolve_theme_palette(ThemeMode::Dark, true, Some("red"), Some("#000000"));
        assert_eq!(p_custom.text_color, Color32::from_rgb(239, 68, 68));
        assert_eq!(p_custom.bg_color, Color32::from_rgb(0, 0, 0));
    }
}
