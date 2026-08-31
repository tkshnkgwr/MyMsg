//! # GUI アプリケーションモジュール (`app.rs`)
//!
//! `eframe::App` トレイトの実装および、メッセージとアイコンの
//! ピクセル完全な上下左右中央配置、スクロール、点滅、キーボードイベント処理を提供します。

use crate::cli::{
    calculate_window_dimensions, parse_icon, parse_theme, CliArgs, IconType, ThemeMode,
};
use crate::color::resolve_theme_palette;
use eframe::egui::{self, Color32, FontFamily, FontId, RichText, ViewportCommand};
use std::time::{Duration, Instant};

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
    /// 自動消去タイマー（秒単位、0は無効）
    pub timeout_secs: u64,
    /// アプリケーション起動時刻（点滅周期・タイムアウト計算用）
    pub start_time: Instant,
}

impl MyMsgApp {
    /// コマンドライン引数からアプリケーション状態を初期化します。
    pub fn new(args: CliArgs) -> Self {
        let message = crate::cli::resolve_message(args.message_arg, args.message_opt);
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
            timeout_secs: args.timeout,
            start_time: Instant::now(),
        }
    }
}

impl eframe::App for MyMsgApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // キーボード即時終了判定 (Esc または Enter)
        if ctx.input(|i| i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Enter)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
            return;
        }

        // 自動消去タイマー判定（指定秒数経過で自動終了）
        if self.timeout_secs > 0 {
            let elapsed = self.start_time.elapsed();
            if elapsed >= Duration::from_secs(self.timeout_secs) {
                ctx.send_viewport_cmd(ViewportCommand::Close);
                return;
            }
            // タイムアウト検知のための再描画要求
            let remaining = Duration::from_secs(self.timeout_secs).saturating_sub(elapsed);
            ctx.request_repaint_after(remaining.min(Duration::from_millis(200)));
        }

        // システムテーマの判定 (ダークモード判定)
        let is_dark_system = !matches!(ctx.system_theme(), Some(egui::Theme::Light));

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

        // 下部アクションバー（最下部に独立固定配置）
        egui::TopBottomPanel::bottom("bottom_bar")
            .frame(
                egui::Frame::none()
                    .fill(palette.bg_color)
                    .inner_margin(egui::Margin::symmetric(16.0, 8.0)),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
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
                });
            });

        // メッセージコンテンツ領域（利用可能領域全体での上下左右完全中央配置＆縦スクロール対応）
        let central_frame = egui::Frame::none()
            .fill(palette.bg_color)
            .inner_margin(egui::Margin::symmetric(16.0, 8.0));

        egui::CentralPanel::default()
            .frame(central_frame)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let avail_w = ui.available_width();
                        let avail_h = ui.available_height();

                        // アイコン幅と余白を考慮したテキストの最大折り返し幅
                        let text_wrap_width = if self.icon.is_some() {
                            (avail_w - (self.font_size * 1.3 + 12.0)).max(60.0)
                        } else {
                            avail_w
                        };

                        // テキストの描画サイズ（Galley）を正確に事前計算
                        let galley = ui.fonts(|f| {
                            f.layout(
                                self.message.clone(),
                                self.font_id.clone(),
                                display_color,
                                text_wrap_width,
                            )
                        });

                        let text_h = galley.size().y;
                        let icon_h = if self.icon.is_some() {
                            self.font_size * 1.3
                        } else {
                            0.0
                        };
                        let content_h = text_h.max(icon_h);

                        // 上下の中央に配置するための垂直パディングを計算
                        if avail_h > content_h {
                            let top_padding = (avail_h - content_h) / 2.0;
                            ui.add_space(top_padding);
                        }

                        // 水平中央揃えでコンテンツを描画
                        ui.vertical_centered(|ui| {
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
                                            ui.label(galley);
                                        },
                                    );
                                });
                            } else {
                                ui.label(galley);
                            }
                        });
                    });
            });
    }
}
