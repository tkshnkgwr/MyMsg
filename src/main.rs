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

pub mod app;
pub mod cli;
pub mod color;
pub mod font;
pub mod monitor;

use app::MyMsgApp;
use clap::Parser;
use cli::{CliArgs, calculate_window_dimensions, clamp_delay_seconds};
use eframe::egui::ViewportBuilder;
use font::setup_japanese_fonts;
use monitor::get_active_monitor_center_position;
use std::thread;
use std::time::Duration;

fn main() -> eframe::Result<()> {
    let args = CliArgs::parse();

    // 遅延表示処理（指定秒数スリープしてからウィンドウを作成）
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

    // eframeのネイティブオプション（centered: false で独自算出のマルチモニター座標を維持）
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
