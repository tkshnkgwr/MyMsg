//! # CLI レイアウト・メッセージ解決サブモジュール (`cli/layout.rs`)
//!
//! ウィンドウ寸法・フォントサイズの算出や、メッセージ文字列のエスケープ展開・優先度解決を提供します。

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
    fn test_resolve_message_newlines() {
        let msg = resolve_message(Some("1行目\\n2行目\\r\\n3行目".into()), None);
        assert_eq!(msg, "1行目\n2行目\n3行目");
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
}
