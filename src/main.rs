// UPDATE 2026-08-28: [RustDocおよび単体テストコード (#[cfg(test)]) の追加]
// Why: Rust版としての保守性・仕様明確化を高め、カラーパース、タイマークランプ、サイズ計算などのロジック品質を保証するため
// UPDATE 2026-08-28: [シミュレーターおよびGUIのタイトルバー・マージン狭小化]
// Why: メッセージ表示領域の視認性を最大化し、極細タイトルバー／フレームマージンのスリム化を実現するため
// UPDATE 2026-08-28: [遅延表示オプション (--delay / -d) の追加]
// Why: 「30秒後に通知」「10分後にリマインド」などタイマー・遅延ポップアップ実行を可能にするため（最大3600秒/1時間ガード付き）
// UPDATE 2026-08-28: [カラー簡易指定の拡張・強化]
// Why: 「Red」「Bule」(typo補正)「GREEN」などの大文字小文字不問な簡易カラー名、1文字略称(r, g, b, y, w, k等)、Web標準色に対応し、CLI入力の手軽さを向上させるため
// UPDATE 2026-08-28: [MyMsg 初回実装]
// Why: Windows/クロスプラットフォームで低CPU負荷かつ高視認性を持つCLIメッセージ通知ツールを実現するため
// 変更点: clapによる引数パース、eframe(egui)による最前面ウィンドウ生成、Esc/Enterキー即時終了、点滅エフェクト対応

//! # MyMsg (マイ・メッセージ)
//!
//! 低スペック・低リソース環境に最適化された、超軽量・最前面固定（Always on Top）メッセージポップアップ通知CLIツール。
//!
//! ## 主な特徴
//! - **Always on Top**: 画面最前面に固定され、全画面アプリや作業中でも確実に通知。
//! - **低CPU負荷**: イベント駆動描画（静止時はCPU 0%）とRustネイティブバイナリによる軽快動作。
//! - **キーボード即時終了**: `Esc` または `Enter` キーを押すだけで瞬時に終了。
//! - **柔軟なカラー指定**: `Red`, `bule`(typo補正), `g`, `#00E5FF` など直感的なカラー名や略称に対応。
//! - **タイマー/遅延通知**: `--delay <秒>` で指定秒数待機後に最前面表示（待機中はGUI非生成で負荷ゼロ）。

use clap::Parser;
use eframe::egui::{self, Color32, FontFamily, FontId, RichText, ViewportBuilder, ViewportCommand};
use std::thread;
use std::time::{Duration, Instant};

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
    #[arg(short = 's', long = "size", default_value = "medium", help = "ウィンドウサイズ [small, medium, large]")]
    pub size: String,

    /// フォントサイズ (pt単位、省略時はウィンドウサイズから自動算出)
    #[arg(long = "font-size", help = "文字サイズ（pt）")]
    pub font_size: Option<f32>,

    /// メッセージ文字色 (名前・1文字略称・#HEX)
    #[arg(short = 'c', long = "color", default_value = "white", help = "文字色 (例: Red, blue, bule, g, #00FFCC)")]
    pub color: String,

    /// ウィンドウ背景色 (省略時は標準ダーク #1a1b26)
    #[arg(long = "bg-color", help = "ウィンドウ背景色 (例: #111111, black, #002244)")]
    pub bg_color: Option<String>,

    /// 文字の点滅表示を有効化（約0.5秒周期で明滅）
    #[arg(short = 'b', long = "blink", help = "メッセージ文字を点滅させる")]
    pub blink: bool,

    /// フォント種別 (1/default/sans, 2/mono, 3/serif, 4/impact)
    #[arg(short = 'f', long = "font", default_value = "default", help = "フォントタイプ (default, sans, mono, serif, impact)")]
    pub font: String,

    /// 表示までの遅延時間（秒単位、0〜3600秒/最大1時間）
    #[arg(short = 'd', long = "delay", default_value_t = 0, help = "指定秒数後にポップアップを表示（最大3600秒）")]
    pub delay: u64,
}

/// MyMsg の GUI アプリケーション状態
pub struct MyMsgApp {
    /// 描画対象のメッセージ文字列
    pub message: String,
    /// メッセージの基本文字色
    pub text_color: Color32,
    /// ウィンドウ背景色
    pub bg_color: Color32,
    /// egui用フォント識別情報 (サイズ + ファミリ)
    pub font_id: FontId,
    /// 点滅エフェクトフラグ
    pub blink: bool,
    /// アプリケーション起動時刻（点滅周期計算用）
    pub start_time: Instant,
}

impl MyMsgApp {
    /// コマンドライン引数からアプリケーション状態を初期化します。
    pub fn new(args: CliArgs) -> Self {
        let message = resolve_message(args.message_arg, args.message_opt);
        let (font_size, _) = calculate_window_dimensions(&args.size, args.font_size);

        let font_family = match args.font.to_lowercase().as_str() {
            "2" | "mono" | "monospace" => FontFamily::Monospace,
            "3" | "serif" => FontFamily::Name("serif".into()),
            _ => FontFamily::Proportional,
        };
        let font_id = FontId::new(font_size, font_family);

        let text_color = parse_color(&args.color).unwrap_or(Color32::from_rgb(240, 240, 240));
        let bg_color = args
            .bg_color
            .as_deref()
            .and_then(parse_color)
            .unwrap_or(Color32::from_rgb(26, 27, 38));

        Self {
            message,
            text_color,
            bg_color,
            font_id,
            blink: args.blink,
            start_time: Instant::now(),
        }
    }
}

/// メッセージ文字列の優先度解決を行います。
///
/// 1. 位置引数 (`message_arg`) があれば最優先
/// 2. `-m / --message` (`message_opt`) があれば採用
/// 3. いずれもなければデフォルトの通知テキストを返します。
pub fn resolve_message(arg: Option<String>, opt: Option<String>) -> String {
    arg.or(opt)
        .unwrap_or_else(|| "MyMsg: 通知が届きました".to_string())
}

/// サイズ指定文字列およびフォントサイズ指定から、適切な文字サイズとウィンドウ寸法を算出します。
///
/// # 戻り値
/// `(font_size, (width, height))`
pub fn calculate_window_dimensions(size_str: &str, custom_font_size: Option<f32>) -> (f32, (f32, f32)) {
    let (default_font_size, dims) = match size_str.trim().to_lowercase().as_str() {
        "small" | "s" => (20.0, (300.0, 150.0)),
        "large" | "l" => (36.0, (650.0, 350.0)),
        _ => (26.0, (450.0, 220.0)), // medium
    };

    let final_font_size = custom_font_size.unwrap_or(default_font_size);
    (final_font_size, dims)
}

/// 遅延秒数を安全な範囲（0〜3600秒 = 最大1時間）にクランプします。
pub fn clamp_delay_seconds(delay: u64) -> u64 {
    delay.min(3600)
}

impl eframe::App for MyMsgApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // キーボード即時終了判定 (Esc または Enter)
        if ctx.input(|i| i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Enter)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
            return;
        }

        // 点滅エフェクト計算 (0.5秒周期)
        let mut display_color = self.text_color;
        if self.blink {
            let elapsed = self.start_time.elapsed().as_secs_f32();
            let phase = (elapsed % 1.0) < 0.5;
            if !phase {
                display_color = Color32::from_rgba_unmultiplied(
                    self.text_color.r(),
                    self.text_color.g(),
                    self.text_color.b(),
                    30,
                );
            }
            ctx.request_repaint_after(Duration::from_millis(250));
        }

        // UPDATE 2026-08-28: [狭小スリムマージンの適用]
        // Why: ウィンドウ全体で無駄な余白を削減し、メッセージ本文を狭い画面でも大きく力強く描画するため
        let frame = egui::Frame::none()
            .fill(self.bg_color)
            .inner_margin(egui::Margin::symmetric(16.0, 10.0));

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);

                // 中央メッセージ表示
                ui.label(
                    RichText::new(&self.message)
                        .font(self.font_id.clone())
                        .color(display_color)
                        .strong(),
                );

                ui.add_space(8.0);

                // 下部アクションバー
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(2.0);
                    let close_btn = ui.add(
                        egui::Button::new(
                            RichText::new("✕ 閉じる (Esc / Enter)")
                                .size(12.0)
                                .color(Color32::from_rgb(200, 200, 200)),
                        )
                        .fill(Color32::from_rgb(45, 48, 65))
                        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(80, 85, 110)))
                        .rounding(4.0),
                    );

                    if close_btn.clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
            });
        });
    }
}

// UPDATE 2026-08-28: [カラー簡易指定の拡張・強化]
// Why: 「Red」「Bule」(typo補正)「GREEN」などの大文字小文字不問な簡易カラー名、1文字略称(r, g, b, y, w, k等)、Web標準色に対応し、CLI入力の手軽さを向上させるため
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

// UPDATE 2026-08-28: [日本語フォントの自動検出とeguiへの登録]
// Why: デフォルトのeguiフォントに含まれない日本語（CJK文字）の豆腐化・文字化けを防止し、OSの日本語フォントを自動ロードするため
/// システムにインストールされた日本語フォントを検出し、eguiのフォント定義に登録します。
pub fn setup_japanese_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());

    // 日本語ゴシック系フォント候補 (Windows / macOS / Linux)
    let font_candidates = [
        format!("{}\\Fonts\\meiryo.ttc", windir),
        format!("{}\\Fonts\\YuGothM.ttc", windir),
        format!("{}\\Fonts\\msgothic.ttc", windir),
        format!("{}\\Fonts\\msyh.ttc", windir),
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
        format!("{}\\Fonts\\msmincho.ttc", windir),
        format!("{}\\Fonts\\yumin.ttf", windir),
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

fn main() -> eframe::Result<()> {
    let args = CliArgs::parse();

    // UPDATE 2026-08-28: [遅延表示処理の実行]
    // Why: 指定された秒数（0〜3600秒）スリープしてからウィンドウを作成することで、タイマー/遅延通知を実現しつつ無駄なGUIリソース消費を防止するため
    if args.delay > 0 {
        let clamped_delay = clamp_delay_seconds(args.delay);
        thread::sleep(Duration::from_secs(clamped_delay));
    }

    let (_, (width, height)) = calculate_window_dimensions(&args.size, args.font_size);

    // eframeのネイティブオプション（最前面・中央・リサイズ不可・タイトルバー設定）
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("MyMsg")
            .with_inner_size([width, height])
            .with_always_on_top()
            .with_resizable(false)
            .with_active(true)
            .with_decorations(true),
        ..Default::default()
    };

    eframe::run_native(
        "MyMsg",
        native_options,
        Box::new(|cc| {
            setup_japanese_fonts(&cc.egui_ctx);
            Ok(Box::new(MyMsgApp::new(args)))
        }),
    )
}

// UPDATE 2026-08-28: [単体テストモジュールの追加]
// Why: カラーパース、引数解決、遅延秒数クランプ、サイズ計算などの単体機能をCIやローカルテストで即座に検証できるようにするため
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

        // medium (default)
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
        assert_eq!(parse_color("#FFFFFF"), Some(Color32::from_rgb(255, 255, 255)));
        assert_eq!(parse_color("00E5FF"), Some(Color32::from_rgb(0, 229, 255)));

        // 3桁 HEX
        assert_eq!(parse_color("#F00"), Some(Color32::from_rgb(255, 0, 0)));
        assert_eq!(parse_color("0F0"), Some(Color32::from_rgb(0, 255, 0)));

        // 8桁 HEX (RGBA)
        assert_eq!(parse_color("#FFFFFF80"), Some(Color32::from_rgba_unmultiplied(255, 255, 255, 128)));

        // 不正値
        assert_eq!(parse_color("invalid_color_xyz"), None);
    }
}
