//! # GUI アプリケーションモジュール (`app.rs`)
//!
//! `eframe::App` トレイトの実装および、メッセージとアイコンの
//! ピクセル完全な上下左右中央配置、スクロール、点滅、キーボードイベント処理を提供します。

use crate::cli::{
    calculate_window_dimensions, parse_icon, parse_theme, CliArgs, IconType, ThemeMode,
};
use crate::color::resolve_theme_palette;
use crate::log::{append_log, LogEvent};
use eframe::egui::{self, Color32, FontFamily, FontId, RichText, ViewportCommand};
use std::path::PathBuf;
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
    /// プログレスバー表示フラグ
    pub show_progress: bool,
    /// ログファイルパス
    pub log_path: Option<PathBuf>,
    /// クリップボードコピーボタン有効フラグ
    pub copy_enabled: bool,
    /// コピー完了通知タイムスタンプ
    pub copied_feedback_until: Option<Instant>,
    /// 外部コマンド実行アクション
    pub action_cmd: Option<String>,
    /// アクション実行結果表示タイムスタンプおよびメッセージ
    pub action_feedback_until: Option<(Instant, bool)>,
    /// アプリケーション起動時刻（点滅周期・タイムアウト計算用）
    pub start_time: Instant,
    /// 定期実行インターバル秒数
    pub interval_secs: Option<u64>,
    /// 複数時刻スケジュールリスト
    pub schedule_times: Vec<chrono::NaiveTime>,
    /// 最大通知回数（0で無制限）
    pub max_count: u64,
    /// 現在の通知回数（1始まり）
    pub current_count: u64,
    /// 完全終了フラグ（Shift+Esc等でセット）
    pub should_exit_all: std::sync::Arc<std::sync::atomic::AtomicBool>,
    /// クローズ処理済みフラグ（二重ログ防止）
    pub is_closing: bool,
}

impl MyMsgApp {
    /// コマンドライン引数からアプリケーション状態を初期化します。
    pub fn new(
        args: CliArgs,
        current_count: u64,
        should_exit_all: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Self {
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
        let interval_secs = args
            .interval
            .as_deref()
            .and_then(crate::cli::parse_interval_to_seconds);
        let schedule_times = crate::cli::parse_at_times(&args.at);

        if let Some(ref path) = args.log {
            append_log(
                path,
                LogEvent::Open {
                    message: &message,
                    count: current_count,
                    timeout_secs: args.timeout,
                },
            );
        }

        if args.sound {
            crate::sound::play_notification_sound(icon);
        }

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
            show_progress: args.show_progress,
            log_path: args.log,
            copy_enabled: args.copy,
            copied_feedback_until: None,
            action_cmd: args.action,
            action_feedback_until: None,
            start_time: Instant::now(),
            interval_secs,
            schedule_times,
            max_count: args.count,
            current_count,
            should_exit_all,
            is_closing: false,
        }
    }

    /// 次のスケジュール（定期実行・予定時刻）が存在するか判定します。
    pub fn has_next_schedule(&self) -> bool {
        if self.max_count > 0 && self.current_count >= self.max_count {
            return false;
        }
        self.interval_secs.is_some() || !self.schedule_times.is_empty()
    }

    /// クローズ時のログ記録
    fn record_close(&mut self, reason: &str) {
        if self.is_closing {
            return;
        }
        self.is_closing = true;
        if let Some(ref path) = self.log_path {
            append_log(
                path,
                LogEvent::Close {
                    reason,
                    elapsed_secs: self.start_time.elapsed().as_secs_f32(),
                },
            );
        }
    }
}

impl eframe::App for MyMsgApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Shift + Esc / Shift + Enter で定期実行を完全に終了
        let is_shift = ctx.input(|i| i.modifiers.shift);
        if is_shift
            && ctx.input(|i| i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Enter))
        {
            self.should_exit_all
                .store(true, std::sync::atomic::Ordering::SeqCst);
            self.record_close("Shift+(Escape/Enter)");
            ctx.send_viewport_cmd(ViewportCommand::Close);
            return;
        }

        // Esc または Enter で今回の通知を閉じる
        if ctx.input(|i| i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Enter)) {
            self.record_close("Escape/Enter");
            ctx.send_viewport_cmd(ViewportCommand::Close);
            return;
        }

        // 自動消去タイマー判定（指定秒数経過で今回通知を自動クローズ）
        let elapsed = self.start_time.elapsed();
        if self.timeout_secs > 0 {
            if elapsed >= Duration::from_secs(self.timeout_secs) {
                self.record_close("Timeout");
                ctx.send_viewport_cmd(ViewportCommand::Close);
                return;
            }
            // プログレスバー表示時は滑らかなアニメーションのために短い再描画間隔を設定
            let remaining = Duration::from_secs(self.timeout_secs).saturating_sub(elapsed);
            let redraw_interval = if self.show_progress {
                Duration::from_millis(50)
            } else {
                remaining.min(Duration::from_millis(200))
            };
            ctx.request_repaint_after(redraw_interval);
        }

        // コピー・アクションフィードバック中の定期再描画
        let now = Instant::now();
        if let Some(until) = self.copied_feedback_until {
            if now < until {
                ctx.request_repaint_after(Duration::from_millis(100));
            } else {
                self.copied_feedback_until = None;
            }
        }
        if let Some((until, _)) = self.action_feedback_until {
            if now < until {
                ctx.request_repaint_after(Duration::from_millis(100));
            } else {
                self.action_feedback_until = None;
            }
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
            let elapsed_sec = elapsed.as_secs_f32();
            let phase = (elapsed_sec % 1.0_f32) < 0.5_f32;
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
                    .inner_margin(egui::Margin::symmetric(16.0_f32, 8.0_f32)),
            )
            .show(ctx, |ui| {
                // タイムアウト・プログレス表示 (--show-progress)
                if self.timeout_secs > 0 && self.show_progress {
                    let total_secs = self.timeout_secs as f32;
                    let remaining_secs = (total_secs - elapsed.as_secs_f32()).max(0.0_f32);
                    let progress_ratio = (remaining_secs / total_secs).clamp(0.0_f32, 1.0_f32);

                    let progress_bar = egui::ProgressBar::new(progress_ratio)
                        .text(format!("{:.1}s / {}s", remaining_secs, self.timeout_secs))
                        .animate(false);

                    ui.add(progress_bar);
                    ui.add_space(4.0_f32);
                }

                // ボタングループ（中央配置）
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        // 横並びセンタリング用のスペーサー計算
                        ui.spacing_mut().item_spacing.x = 6.0_f32;

                        // クリップボードコピーボタン (--copy)
                        if self.copy_enabled {
                            let is_copied = self
                                .copied_feedback_until
                                .map(|u| Instant::now() < u)
                                .unwrap_or(false);
                            let copy_label = if is_copied {
                                "✓ コピー完了"
                            } else {
                                "📋 コピー"
                            };

                            let copy_btn = ui.add(
                                egui::Button::new(
                                    RichText::new(copy_label)
                                        .size(11.0_f32)
                                        .color(palette.button_text),
                                )
                                .fill(palette.button_bg)
                                .stroke(egui::Stroke::new(1.0_f32, palette.button_stroke))
                                .rounding(4.0_f32),
                            );

                            if copy_btn.clicked() {
                                ctx.output_mut(|o| o.copied_text = self.message.clone());
                                self.copied_feedback_until =
                                    Some(Instant::now() + Duration::from_secs(2));
                                if let Some(ref path) = self.log_path {
                                    append_log(path, LogEvent::Copy);
                                }
                            }
                        }

                        // アクションボタン (--action <cmd>)
                        if let Some(ref cmd_str) = self.action_cmd {
                            let action_label =
                                if let Some((u, success)) = self.action_feedback_until {
                                    if Instant::now() < u {
                                        if success {
                                            "✓ 実行完了"
                                        } else {
                                            "✖ 実行失敗"
                                        }
                                    } else {
                                        "⚡ 実行"
                                    }
                                } else {
                                    "⚡ 実行"
                                };

                            let action_btn = ui.add(
                                egui::Button::new(
                                    RichText::new(action_label)
                                        .size(11.0_f32)
                                        .color(palette.button_text),
                                )
                                .fill(palette.button_bg)
                                .stroke(egui::Stroke::new(1.0_f32, palette.button_stroke))
                                .rounding(4.0_f32),
                            );

                            if action_btn.clicked() {
                                let target_cmd = cmd_str.clone();
                                let log_path_clone = self.log_path.clone();

                                // Windows では cmd /C、その他では sh -c で実行
                                #[cfg(windows)]
                                let mut cmd = std::process::Command::new("cmd");
                                #[cfg(windows)]
                                cmd.args(["/C", &target_cmd]);

                                #[cfg(not(windows))]
                                let mut cmd = std::process::Command::new("sh");
                                #[cfg(not(windows))]
                                cmd.args(["-c", &target_cmd]);

                                let spawn_res = cmd.spawn();
                                let is_ok = spawn_res.is_ok();
                                self.action_feedback_until =
                                    Some((Instant::now() + Duration::from_secs(2), is_ok));

                                if let Some(ref path) = log_path_clone {
                                    append_log(
                                        path,
                                        LogEvent::Action {
                                            cmd: &target_cmd,
                                            success: is_ok,
                                        },
                                    );
                                }
                            }
                        }

                        // 閉じるボタン
                        let has_next = self.has_next_schedule();
                        let btn_label = if has_next {
                            if self.max_count > 0 {
                                format!(
                                    "✕ 今回閉じる ({}/{}回) [Esc]",
                                    self.current_count, self.max_count
                                )
                            } else {
                                format!("✕ 今回閉じる ({}回目) [Esc]", self.current_count)
                            }
                        } else {
                            "✕ 閉じる (Esc / Enter)".to_string()
                        };

                        let close_btn = ui.add(
                            egui::Button::new(
                                RichText::new(btn_label)
                                    .size(11.0_f32)
                                    .color(palette.button_text),
                            )
                            .fill(palette.button_bg)
                            .stroke(egui::Stroke::new(1.0_f32, palette.button_stroke))
                            .rounding(4.0_f32),
                        );

                        if close_btn.clicked() {
                            self.record_close("CloseButton");
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }
                    });

                    if self.has_next_schedule() {
                        ui.add_space(2.0_f32);
                        ui.label(
                            RichText::new("完全終了: Shift+Esc")
                                .size(10.0_f32)
                                .color(Color32::from_gray(140)),
                        );
                    }
                });
            });

        // メッセージコンテンツ領域（利用可能領域全体での上下左右完全中央配置＆縦スクロール対応）
        let central_frame = egui::Frame::none()
            .fill(palette.bg_color)
            .inner_margin(egui::Margin::symmetric(16.0_f32, 8.0_f32));

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
                            (avail_w - (self.font_size * 1.3_f32 + 12.0_f32)).max(60.0_f32)
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
                            self.font_size * 1.3_f32
                        } else {
                            0.0_f32
                        };
                        let content_h = text_h.max(icon_h);

                        // 上下の中央に配置するための垂直パディングを計算
                        if avail_h > content_h {
                            let top_padding = (avail_h - content_h) / 2.0_f32;
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
                                                    .size(self.font_size * 1.3_f32)
                                                    .color(icon.default_color())
                                                    .strong(),
                                            );
                                            ui.add_space(8.0_f32);
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
