// UPDATE 2026-08-28: [モジュール分割リファクタリングの実施]
// Why: 約880行の単一ソースコードを責務別（cli, color, font, monitor, app）に分割し、保守性と拡張性を大幅に向上させるため

//! # MyMsg (マイ・メッセージ)
//!
//! 低スペック・低リソース環境に最適化された、超軽量・最前面固定（Always on Top）メッセージポップアップ通知CLIツール。
//!
//! ## 主な特徴
//! - **Always on Top**: 画面最前面に固定され、全画面アプリや作業中でも確実に通知。
//! - **マルチモニター自動追従**: マウスカーソルのあるアクティブ画面の中央にポップアップ。
//! - **低CPU負荷**: イベント駆動描画（静止時はCPU 0%）とRustネイティブバイナリによる軽快動作。
//! - **キーボード即時終了**: `Esc` または `Enter` キーを押すだけで瞬時に終了。
//! - **複数行・自動折返し & 上下左右完全中央配置**: 長文や改行コード（`\n`）を含むテキストを最適に折り返し描画。
//! - **アイコン表示**: `--icon <info|warn|error|ok>` で通知シンボルを左側に表示。
//! - **テーマ切り替え**: `--theme <system|dark|light>` でシステム自動追従またはダーク/ライト固定。
//! - **柔軟なカラー指定**: `Red`, `bule`(typo補正), `g`, `#00E5FF` など直感的なカラー名や略称に対応。
//! - **タイマー/遅延通知**: `--delay <秒>` で指定秒数待機後に最前面表示（待機中はGUI非生成で負荷ゼロ）。
//! - **定期実行 & スケジュール**: `--interval <間隔>` や `--at <時刻,...>` による繰り返し通知と指定時刻スケジュール。
//! - **自動消去 & プログレスバー**: `--timeout <秒>` による自動終了と `--show-progress` による残り時間カウントダウン。
//! - **クリップボードコピー & アクション**: `--copy` によるテキストコピーと `--action <cmd>` による外部コマンド実行。
//! - **ロギング**: `--log <file>` による表示・終了・操作履歴のファイル追記。

pub mod app;
pub mod cli;
pub mod color;
pub mod font;
pub mod log;
pub mod monitor;
pub mod session;
pub mod sound;
pub mod toast;

use app::MyMsgApp;
use clap::Parser;
use cli::{
    calculate_next_schedule_wait, calculate_window_dimensions, parse_at_times,
    parse_delay_to_seconds, parse_interval_to_seconds, parse_monitor_target, CliArgs,
};
use eframe::egui::ViewportBuilder;
use font::setup_japanese_fonts;
use monitor::get_monitor_center_position;
use std::thread;
use std::time::Duration;

fn main() -> eframe::Result<()> {
    // タスクスケジューラやエクスプローラーから単独起動された場合は自動的に黒いコンソール窓を解放・消去
    session::auto_detach_console_if_standalone();

    let mut args = CliArgs::parse();

    // ヘッドレス / 非対話セッション (Session 0) の検出と自動フォールバック
    if !args.toast && !session::is_interactive_session() {
        eprintln!(
            "MyMsg [警告]: 非対話セッション（Session 0 / ヘッドレス環境）を検出しました。\n\
             GUI ウィンドウを表示できないため、OSトースト通知モードへ自動フォールバックします。"
        );
        if let Some(ref path) = args.log {
            log::append_log(
                path,
                log::LogEvent::Action {
                    cmd: "Session0Detection: Fallback to Toast",
                    success: true,
                },
            );
        }
        args.toast = true;
    }

    let delay_secs = parse_delay_to_seconds(&args.delay);
    let interval_secs = args.interval.as_deref().and_then(parse_interval_to_seconds);
    let schedule_times = parse_at_times(&args.at);

    // OS標準トースト通知モード（GUIウィンドウを立ち上げずに定期/即時送信）
    if args.toast {
        let mut current_count = 0u64;
        loop {
            // 初回および次回待機時間の算出
            let wait_secs = if current_count == 0 {
                if delay_secs > 0 {
                    delay_secs
                } else if !schedule_times.is_empty() {
                    let now = chrono::Local::now().time();
                    calculate_next_schedule_wait(now, &schedule_times).unwrap_or(0)
                } else if let Some(int_secs) = interval_secs {
                    if args.immediate {
                        0
                    } else {
                        int_secs
                    }
                } else {
                    0
                }
            } else if let Some(int_secs) = interval_secs {
                int_secs
            } else if !schedule_times.is_empty() {
                let now = chrono::Local::now().time();
                calculate_next_schedule_wait(now, &schedule_times).unwrap_or(0)
            } else {
                break;
            };

            if wait_secs > 0 {
                thread::sleep(Duration::from_secs(wait_secs));
            }

            if let Some(ref path) = args.log {
                let msg = cli::resolve_message(args.message_arg.clone(), args.message_opt.clone());
                log::append_log(
                    path,
                    log::LogEvent::Open {
                        message: &msg,
                        count: current_count + 1,
                        timeout_secs: args.timeout,
                    },
                );
            }

            if let Err(err) = toast::send_toast_notification(&args) {
                eprintln!("MyMsg: トースト通知の送信に失敗しました: {err}");
            }

            if let Some(ref path) = args.log {
                log::append_log(
                    path,
                    log::LogEvent::Close {
                        reason: "ToastSent",
                        elapsed_secs: 0.0,
                    },
                );
            }

            current_count += 1;
            if args.count > 0 && current_count >= args.count {
                break;
            }

            // スケジュール設定がない場合は1回で終了
            if interval_secs.is_none() && schedule_times.is_empty() {
                break;
            }
        }
        return Ok(());
    }

    // GUIモードの表示ループ（初回遅延・インターバル・時刻スケジュール・最大回数）
    let should_exit_all = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let mut current_count = 0u64;

    loop {
        let wait_secs = if current_count == 0 {
            if delay_secs > 0 {
                delay_secs
            } else if !schedule_times.is_empty() {
                let now = chrono::Local::now().time();
                calculate_next_schedule_wait(now, &schedule_times).unwrap_or(0)
            } else if let Some(int_secs) = interval_secs {
                if args.immediate {
                    0
                } else {
                    int_secs
                }
            } else {
                0
            }
        } else if let Some(int_secs) = interval_secs {
            int_secs
        } else if !schedule_times.is_empty() {
            let now = chrono::Local::now().time();
            calculate_next_schedule_wait(now, &schedule_times).unwrap_or(0)
        } else {
            break;
        };

        if wait_secs > 0 {
            thread::sleep(Duration::from_secs(wait_secs));
        }

        let (_, (width, height)) = calculate_window_dimensions(&args.size, args.font_size);

        let mut viewport = ViewportBuilder::default()
            .with_title("MyMsg")
            .with_inner_size([width, height])
            .with_always_on_top()
            .with_resizable(false)
            .with_active(true)
            .with_decorations(true);

        // 指定されたモニター（既定: マウスカーソル位置）の中央に配置
        let monitor_target = parse_monitor_target(&args.monitor);
        if let Some(pos) = get_monitor_center_position(monitor_target, width, height) {
            viewport = viewport.with_position(pos);
        }

        let native_options = eframe::NativeOptions {
            centered: false,
            viewport,
            ..Default::default()
        };

        let app_args = args.clone();
        let app_should_exit = std::sync::Arc::clone(&should_exit_all);
        let count_for_app = current_count + 1;

        let res = eframe::run_native(
            "MyMsg",
            native_options,
            Box::new(move |cc| {
                setup_japanese_fonts(&cc.egui_ctx);
                Ok(Box::new(MyMsgApp::new(
                    app_args,
                    count_for_app,
                    app_should_exit,
                )))
            }),
        );

        if let Err(err) = res {
            eprintln!("MyMsg: ウィンドウの実行に失敗しました: {err}");
            eprintln!("MyMsg: OSトースト通知へ自動フォールバックします。");
            let _ = toast::send_toast_notification(&args);
            if let Some(ref path) = args.log {
                log::append_log(
                    path,
                    log::LogEvent::Action {
                        cmd: "WindowLaunchFailed: Fallback to Toast",
                        success: true,
                    },
                );
            }
            break;
        }

        if should_exit_all.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }

        current_count += 1;
        if args.count > 0 && current_count >= args.count {
            break;
        }

        if interval_secs.is_none() && schedule_times.is_empty() {
            break;
        }
    }

    Ok(())
}
