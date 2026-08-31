//! # フォント管理モジュール (`font.rs`)
//!
//! OS環境（Windows / macOS / Linux）に合わせたシステム日本語フォントの
//! 自動検出および `egui::Context` へのフォント定義登録処理を提供します。

use eframe::egui;

/// システムにインストールされた日本語フォントを検出し、eguiのフォント定義に登録します。
pub fn setup_japanese_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());

    // 日本語ゴシック系フォント候補 (Windows / macOS / Linux)
    let font_candidates = [
        format!("{windir}\\Fonts\\meiryo.ttc"),
        format!("{windir}\\Fonts\\YuGothM.ttc"),
        format!("{windir}\\Fonts\\msgothic.ttc"),
        format!("{windir}\\Fonts\\msyh.ttc"),
        "/System/Library/Fonts/Hiragino Sans GB.ttc".to_string(),
        "/System/Library/Fonts/PingFang.ttc".to_string(),
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc".to_string(),
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc".to_string(),
    ];

    for path in &font_candidates {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "japanese_font".to_owned(),
                egui::FontData::from_owned(font_data),
            );

            // Proportionalフォントの最優先として登録
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "japanese_font".to_owned());

            // Monospaceフォントのフォールバックとしても追加
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("japanese_font".to_owned());

            break;
        }
    }

    // 明朝系（Serif）フォント候補
    let serif_candidates = [
        format!("{windir}\\Fonts\\msmincho.ttc"),
        format!("{windir}\\Fonts\\yumin.ttf"),
    ];
    let mut serif_loaded = false;
    for path in &serif_candidates {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "japanese_serif".to_owned(),
                egui::FontData::from_owned(font_data),
            );
            fonts.families.insert(
                egui::FontFamily::Name("serif".into()),
                vec!["japanese_serif".to_owned(), "japanese_font".to_owned()],
            );
            serif_loaded = true;
            break;
        }
    }
    if !serif_loaded {
        fonts.families.insert(
            egui::FontFamily::Name("serif".into()),
            vec!["japanese_font".to_owned()],
        );
    }

    ctx.set_fonts(fonts);
}
