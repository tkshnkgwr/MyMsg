// UPDATE 2026-08-28: [GUIレイアウト高度化（複数行折返し最適化・アイコン）およびテーマオプションの追加]
// Why: 長文/改行メッセージの美しく安全な自動折返し表示、通知種別アイコン(info/warn/error/ok)、Dark/Light/System連動テーマを実現するため
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
//! - **複数行・自動折返し**: 長文や改行コード（`\n`）を含むテキストを最適に折り返し描画。
//! - **アイコン表示**: `--icon <info|warn|error|ok>` で通知シンボルを左側に表示。
//! - **テーマ切り替え**: `--theme <system|dark|light>` でシステム自動追従またはダーク/ライト固定。
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

/// MyMsg の GUI アプリケーション状態
pub struct MyMsgApp {
    /// 描画対象のメッセージ文字列
    pub message: String,
    /// ユーザー指定文字色（文字列のまま保持し、テーマ解決時に適用）
    pub custom_text_color: Option<String>,
    /// ユーザー指定背景色
    pub custom_bg_color: Option<String>,
    /// テーマモード設定
    pub theme_mode: ThemeMode,
    /// アイコン設定
    pub icon: Option<IconType>,
    /// egui用フォント識別情報 (サイズ + ファミリ)
    pub font_id: FontId,
    /// フォントサイズ数値
    pub font_size: f32,
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

        let icon = args.icon.as_deref().and_then(parse_icon);
        let theme_mode = parse_theme(&args.theme);

        Self {
            message,
            custom_text_color: args.color,
            custom_bg_color: args.bg_color,
            theme_mode,
            icon,
            font_id,
            font_size,
            blink: args.blink,
            start_time: Instant::now(),
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

        // システムテーマの判定 (ダークモード判定)
        let is_dark_system = match ctx.system_theme() {
            Some(egui::Theme::Light) => false,
            _ => true, // ダークまたは未設定時はデフォルトでダークモード
        };

        // カラーパレットの解決
        let palette = resolve_theme_palette(
            self.theme_mode,
            is_dark_system,
            self.custom_text_color.as_deref(),
            self.custom_bg_color.as_deref(),
        );

        // 点滅エフェクト計算 (0.5秒周期)
        let mut display_color = palette.text_color;
        if self.blink {
            let elapsed = self.start_time.elapsed().as_secs_f32();
            let phase = (elapsed % 1.0) < 0.5;
            if !phase {
                display_color = Color32::from_rgba_unmultiplied(
                    palette.text_color.r(),
                    palette.text_color.g(),
                    palette.text_color.b(),
                    30,
                );
            }
            ctx.request_repaint_after(Duration::from_millis(250));
        }

        let frame = egui::Frame::none()
            .fill(palette.bg_color)
            .inner_margin(egui::Margin::symmetric(16.0, 10.0));

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            // 下部アクションバーを最下部に固定配置
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(4.0);
                let close_btn = ui.add(
                    egui::Button::new(
                        RichText::new("✕ 閉じる (Esc / Enter)")
                            .size(12.0)
                            .color(palette.button_text),
                    )
                    .fill(palette.button_bg)
                    .stroke(egui::Stroke::new(1.0, palette.button_stroke))
                    .rounding(4.0),
                );

                if close_btn.clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }

                ui.add_space(6.0);

                // メッセージコンテンツ領域（縦スクロール＆自動折り返し＆正確な上下左右中央配置）
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.with_layout(
                            egui::Layout::top_down(egui::Align::Center)
                                .with_cross_align(egui::Align::Center)
                                .with_main_align(egui::Align::Center),
                            |ui| {
                                if let Some(icon) = self.icon {
                                    ui.horizontal(|ui| {
                                        ui.with_layout(
                                            egui::Layout::left_to_right(egui::Align::Center)
                                                .with_main_align(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    RichText::new(icon.symbol())
                                                        .size(self.font_size * 1.3)
                                                        .color(icon.default_color())
                                                        .strong(),
                                                );
                                                ui.add_space(8.0);
                                                ui.label(
                                                    RichText::new(&self.message)
                                                        .font(self.font_id.clone())
                                                        .color(display_color)
                                                        .strong(),
                                                );
                                            },
                                        );
                                    });
                                } else {
                                    ui.label(
                                        RichText::new(&self.message)
                                            .font(self.font_id.clone())
                                            .color(display_color)
                                            .strong(),
                                    );
                                }
                            },
                        );
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

// UPDATE 2026-08-28: [マルチモニター対応：アクティブディスプレイ物理ピクセル中央座標の算出]
// Why: eframe/winit の with_position() は仮想デスクトップの物理ピクセル座標をそのまま受け付ける。
//      centered: true はウィンドウ生成後にプライマリモニター中央へ set_outer_position() を再呼び出しするため
//      with_position() を上書きしてしまう。centered: false にして自前の座標を使うことで修正する。
/// 現在のマウスカーソルが存在するモニターの作業領域（タスクバーを除いた領域）の
/// 物理ピクセル中央座標を返します。winit の with_position() に直接渡せる単位です。
#[cfg(windows)]
pub fn get_active_monitor_center_position(
    window_width: f32,
    window_height: f32,
) -> Option<[f32; 2]> {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;

    unsafe {
        let mut cursor_pos = POINT { x: 0, y: 0 };
        if GetCursorPos(&mut cursor_pos) == 0 {
            return None;
        }

        let h_monitor = MonitorFromPoint(cursor_pos, MONITOR_DEFAULTTONEAREST);
        if h_monitor == 0 {
            return None;
        }

        let mut monitor_info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: std::mem::zeroed(),
            rcWork: std::mem::zeroed(),
            dwFlags: 0,
        };
        if GetMonitorInfoW(h_monitor, &mut monitor_info) == 0 {
            return None;
        }

        // rcWork は仮想デスクトップ上の物理ピクセル座標
        // winit の with_position([x, y]) もこれと同じ物理ピクセル単位を受け付ける
        let left = monitor_info.rcWork.left as f32;
        let top = monitor_info.rcWork.top as f32;
        let work_w = (monitor_info.rcWork.right - monitor_info.rcWork.left) as f32;
        let work_h = (monitor_info.rcWork.bottom - monitor_info.rcWork.top) as f32;

        let pos_x = left + (work_w - window_width) / 2.0;
        let pos_y = top + (work_h - window_height) / 2.0;

        Some([pos_x, pos_y])
    }
}

/// 非Windows環境用フォールバック
#[cfg(not(windows))]
pub fn get_active_monitor_center_position(
    _window_width: f32,
    _window_height: f32,
) -> Option<[f32; 2]> {
    None
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

    let mut viewport = ViewportBuilder::default()
        .with_title("MyMsg")
        .with_inner_size([width, height])
        .with_always_on_top()
        .with_resizable(false)
        .with_active(true)
        .with_decorations(true);

    // マルチモニター環境で現在操作中のディスプレイ中央に配置
    if let Some(pos) = get_active_monitor_center_position(width, height) {
        viewport = viewport.with_position(pos);
    }

    // eframeのネイティブオプション
    // IMPORTANT: centered: false にしないと eframe がウィンドウ生成後に
    //            プライマリモニター中央へ set_outer_position() を再呼び出しし、
    //            with_position() で設定したアクティブモニター座標が上書きされる
    let native_options = eframe::NativeOptions {
        centered: false,
        viewport,
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
