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
    version,
    about = "最前面メッセージポップアップCLIツール",
    long_about = "低リソース環境向けに最適化された、最前面固定のメッセージポップアップ通知CLIです。EscまたはEnterで即座に閉じられます。"
)]
pub struct CliArgs {
    /// 表示するメッセージ（位置引数）
    #[arg(
        value_name = "MESSAGE",
        index = 1,
        help = "表示するメッセージ文字列",
        long_help = "ポップアップまたはトーストに表示するメッセージ文字列。\n\
                     改行コード（\\n または \\r\\n）を含めると複数行テキストとして描画されます。\n\
                     省略時は「MyMsg: 通知が届きました」が表示されます。"
    )]
    pub message_arg: Option<String>,

    /// 表示するメッセージ（オプション引数 -m / --message）
    #[arg(
        short = 'm',
        long = "message",
        help = "表示するメッセージ文字列",
        long_help = "表示するメッセージ文字列（-m / --message）。\n\
                     位置引数と同時に指定された場合は位置引数が優先されます。"
    )]
    pub message_opt: Option<String>,

    /// ウィンドウサイズ (small: 300x150, medium: 450x220, large: 650x350)
    #[arg(
        short = 's',
        long = "size",
        default_value = "medium",
        help = "ウィンドウサイズ [small, medium, large]",
        long_help = "ウィンドウ寸法とフォントサイズのプリセット。\n\
                     ・small (s): 幅300x高150 px, 文字サイズ 20pt (控えめな通知)\n\
                     ・medium (m): 幅450x高220 px, 文字サイズ 26pt (標準・既定値)\n\
                     ・large (l): 幅650x高350 px, 文字サイズ 36pt (大画面・重要アラート)"
    )]
    pub size: String,

    /// フォントサイズ (pt単位、省略時はウィンドウサイズから自動算出)
    #[arg(
        long = "font-size",
        help = "文字サイズ（pt）",
        long_help = "メッセージ文字のフォントサイズをポイント（pt）単位で直接指定。\n\
                     指定した場合は --size プリセットの既定文字サイズを上書きします。"
    )]
    pub font_size: Option<f32>,

    /// メッセージ文字色 (名前・1文字略称・#HEX、省略時はテーマ標準色)
    #[arg(
        short = 'c',
        long = "color",
        help = "文字色 (例: Red, blue, bule, g, #00FFCC)",
        long_help = "メッセージ文字色を指定。\n\
                     ・基本色名: Red, Green, Blue, Yellow, Cyan, Magenta, Pink, White, Black\n\
                     ・1文字略称: r (赤), g (緑), b (青), y (黄), c (シアン), m (マゼンタ), w (白), k (黒)\n\
                     ・日本語和名: 赤, 緑, 青, 黄, 白, 黒\n\
                     ・タイポ補正: bule (→ blue), grean (→ green), yello (→ yellow) 等\n\
                     ・HEXカラー: #RGB, #RRGGBB, #RRGGBBAA (例: #00FFCC, #FF4500)\n\
                     ・拡張パレット色: Lime, Gold, Amber, Emerald, Teal, Sky, Indigo, Violet, Rose, Crimson, Navy, Gray\n\
                     ・省略時はテーマの標準テキスト色が適用されます。"
    )]
    pub color: Option<String>,

    /// ウィンドウ背景色 (省略時はテーマ標準色)
    #[arg(
        long = "bg-color",
        help = "ウィンドウ背景色 (例: #111111, black, #002244)",
        long_help = "ウィンドウ全体の背景色を指定（HEXコードまたは色名）。\n\
                     例: #111111, black, #002244, #1A1B26\n\
                     ・省略時はテーマの標準背景色が適用されます。"
    )]
    pub bg_color: Option<String>,

    /// 文字の点滅表示を有効化（約0.5秒周期で明滅）
    #[arg(
        short = 'b',
        long = "blink",
        help = "メッセージ文字を点滅させる",
        long_help = "メッセージ文字を約0.5秒周期で明滅（点滅）させます。\n\
                     緊急のアラートや目立たせたい通知に最適です。"
    )]
    pub blink: bool,

    /// フォント種別 (1/default/sans, 2/mono, 3/serif, 4/impact)
    #[arg(
        short = 'f',
        long = "font",
        default_value = "default",
        help = "フォントタイプ (default, sans, mono, serif, impact)",
        long_help = "描画に使用するフォントファミリを指定。\n\
                     ・default / 1 / sans: 標準プロポーショナルゴシック体（Meiryo等、既定値）\n\
                     ・mono / 2 / monospace: 等幅フォント（ソースコード・ログ・パス表示向け）\n\
                     ・serif / 3: 明朝体・セリフ体\n\
                     ・impact / 4: 強調フォント"
    )]
    pub font: String,

    /// アイコン表示種別 (info, warn, error, ok)
    #[arg(
        short = 'i',
        long = "icon",
        help = "アイコン表示 [info, warn, error, ok]",
        long_help = "メッセージ左側に表示するステータスシンボルアイコンを指定。\n\
                     ・info (i, information): ℹ スカイブルー（情報・案内）\n\
                     ・warn (w, warning, alert): ⚠ アンバーイエロー（注意・警告）\n\
                     ・error (e, err, danger, ng): ✖ レッド（重大エラー・異常）\n\
                     ・ok (s, k, success, check): ✔ グリーン（成功・処理完了）"
    )]
    pub icon: Option<String>,

    /// テーマ設定 (system, dark, light)
    #[arg(
        short = 't',
        long = "theme",
        default_value = "system",
        help = "テーマ設定 [system, dark, light]",
        long_help = "ウィンドウ全体のカラーテーマを指定。\n\
                     ・system (sys, auto): OSのダーク/ライトモード設定を自動検出して追従 (既定値)\n\
                     ・dark (d, black): ダークモード固定 (#1A1B26 背景 / 明るい文字)\n\
                     ・light (l, white): ライトモード固定 (#F8FAFC 背景 / 濃い文字)"
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
                     ・最大待機時間は24時間 (86400秒) です。"
    )]
    pub delay: String,

    /// 表示先モニターの指定 (cursor, primary, またはモニター番号 0, 1, 2...)
    #[arg(
        long = "monitor",
        default_value = "cursor",
        help = "表示先モニター [cursor, primary, 0, 1, 2...]",
        long_help = "表示先モニター（ディスプレイ）を指定。\n\
                     ・cursor (c, mouse, active): マウスカーソルが存在する画面中央 (既定値)\n\
                     ・primary (p, main): メインディスプレイの作業領域中央\n\
                     ・0, 1, 2...: 指定したモニター番号の中央 (範囲外時はプライマリ)"
    )]
    pub monitor: String,

    /// 自動消去タイマー（秒単位、0は無効で手動終了まで待機）
    #[arg(
        long = "timeout",
        default_value_t = 0,
        help = "指定秒数経過後に自動でウィンドウを閉じる（0で無効）",
        long_help = "指定秒数経過後に自動でウィンドウを閉じるタイマー（秒単位）。\n\
                     0 を指定すると無効化され、キー入力（Esc/Enter）まで待機します。\n\
                     定期実行時は自動で閉じて次回待機へ移行します。"
    )]
    pub timeout: u64,

    /// OS標準のトースト通知モード（GUIウィンドウを表示せず通知センター経由で表示）
    #[arg(
        short = 'T',
        long = "toast",
        help = "OS標準のトースト通知として表示（GUI非生成・即時終了）",
        long_help = "OS標準のネイティブトースト通知（通知センター）として表示。\n\
                     GUIウィンドウを立ち上げずに即時送信・終了（低リソース）。\n\
                     --interval や --at と併用した場合はバックグラウンドで定期通知を発行します。"
    )]
    pub toast: bool,

    /// 定期実行・インターバル通知の間隔（秒数、または 30m, 1h, 30分 などの単位指定）
    #[arg(
        long = "interval",
        visible_alias = "every",
        help = "指定間隔ごとに繰り返し通知を表示 [例: 30m, 1h, 300, 30分]",
        long_help = "指定間隔ごとに繰り返し通知を表示する定期実行タイマー。\n\
                     ・秒数指定: 300, 1800, 3600\n\
                     ・単位指定: 30s (秒), 30m (分), 1h (時間), 30分, 1時間\n\
                     ・終了操作: Esc / Enter / 閉じるボタンで「今回閉じて次回待機」、Shift+Esc で「完全終了」。\n\
                     ・--immediate を併用すると初回待機をスキップして即時1回目を表示します。"
    )]
    pub interval: Option<String>,

    /// 複数時刻・スケジュール指定（カンマ区切りまたは複数指定、例: 09:00,12:00,15:00）
    #[arg(
        long = "at",
        visible_alias = "schedule",
        value_delimiter = ',',
        help = "指定時刻（複数可）に通知を表示 [例: 09:00,12:00,15:00]",
        long_help = "1日の中の特定時刻（複数指定可）に通知を表示するスケジュール機能。\n\
                     ・指定例: --at 09:00,12:00,15:00 または --at 09:00 --at 12:00\n\
                     ・書式: HH:MM または HH:MM:SS (24時間制)\n\
                     本日の未来の時刻を順次待機・表示し、当日分が終わると翌日先頭時刻へ自動ループします。"
    )]
    pub at: Vec<String>,

    /// 最大通知回数（0で無制限ループ）
    #[arg(
        long = "count",
        visible_alias = "times",
        default_value_t = 0,
        help = "定期実行時の最大通知回数（0で無制限）",
        long_help = "定期実行（--interval / --at）時の最大通知回数。\n\
                     0 を指定すると無制限にループします（既定値: 0）。\n\
                     指定回数通知するとプロセスが自動的に完全終了します。"
    )]
    pub count: u64,

    /// 定期実行時、初回待機をスキップして起動直後に1回目の通知を表示
    #[arg(
        long = "immediate",
        help = "インターバル指定時、初回待機を行わず即座に1回目を表示",
        long_help = "インターバル通知（--interval）指定時、初回待機を行わずに起動直後まず1回目の通知を表示します。"
    )]
    pub immediate: bool,

    /// タイムアウト時の残り時間・プログレスバーを画面下部に表示
    #[arg(
        long = "show-progress",
        visible_alias = "progress",
        help = "タイムアウト時の残り時間・プログレスバーを表示",
        long_help = "タイムアウト（--timeout）指定時に、残り時間またはカウントダウンを視覚的に示すプログレスバーを表示します。\n\
                     ・--timeout が指定されている場合のみ有効です。\n\
                     ・プログレスバー内に残り秒数がアニメーション表示されます。"
    )]
    pub show_progress: bool,

    /// ログファイル出力先パス（ポップアップ表示・操作・終了履歴を記録）
    #[arg(
        long = "log",
        value_name = "FILE",
        help = "ポップアップ履歴やアクションを指定ログファイルに追記",
        long_help = "ポップアップの起動時刻、表示回数、終了時刻・終了理由、アクション実行履歴を指定ファイルに追記します。\n\
                     ・指定パスが存在しない場合は自動新規作成されます。\n\
                     ・定期実行の稼働監視やバッチ処理の実行証跡記録に最適です。"
    )]
    pub log: Option<std::path::PathBuf>,

    /// メッセージ文字列をクリップボードにコピーするボタンを表示
    #[arg(
        long = "copy",
        help = "クリップボードコピー用ボタンを表示",
        long_help = "メッセージ文字列を1クリックでOSのクリップボードへコピーできるボタンを画面下部に表示します。\n\
                     ・クリックすると「✓ コピー完了」にフィードバック表示されます。"
    )]
    pub copy: bool,

    /// 外部コマンドを実行するアクションボタンを表示
    #[arg(
        long = "action",
        value_name = "CMD",
        help = "外部コマンドを実行するアクションボタンを表示",
        long_help = "指定した外部コマンド（シェルまたは実行可能ファイル）をワンクリックで実行できる「⚡ 実行」ボタンを表示します。\n\
                     ・例: --action \"notepad.exe C:\\path\\to\\file.txt\"\n\
                     ・クリック時にバックグラウンドでプロセスを起動します。"
    )]
    pub action: Option<String>,
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

/// 単位付き時間（10s, 30m, 1h, 10秒, 30分, 1時間 等）または純粋な秒数を秒数に変換します。
pub fn parse_duration_to_seconds(input: &str) -> Option<u64> {
    let clean = input.trim().to_lowercase();
    if clean.is_empty() {
        return None;
    }
    if let Ok(secs) = clean.parse::<u64>() {
        return Some(secs);
    }
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
            return Some(num * mult);
        }
    }
    None
}

/// インターバル指定文字列（30m, 1h, 300, 30分 など）を解析し、秒数を返します（1秒以上有効）。
pub fn parse_interval_to_seconds(input: &str) -> Option<u64> {
    parse_duration_to_seconds(input).filter(|&s| s > 0)
}

/// 時刻文字列（HH:MM または HH:MM:SS）を解析して NaiveTime を返します。
pub fn parse_time_str(input: &str) -> Option<NaiveTime> {
    let clean = input.trim();
    if !clean.contains(':') {
        return None;
    }
    let parts: Vec<&str> = clean.split(':').collect();
    if parts.len() == 2 || parts.len() == 3 {
        let h_opt = parts[0].trim().parse::<u32>().ok();
        let m_opt = parts[1].trim().parse::<u32>().ok();
        let s_opt = if parts.len() == 3 {
            parts[2].trim().parse::<u32>().ok()
        } else {
            Some(0)
        };

        match (h_opt, m_opt, s_opt) {
            (Some(h), Some(m), Some(s)) if h < 24 && m < 60 && s < 60 => {
                NaiveTime::from_hms_opt(h, m, s)
            }
            _ => None,
        }
    } else {
        None
    }
}

/// 複数時刻指定引数（カンマ区切りまたは配列）を解析し、ソート・重複排除された NaiveTime リストを返します。
pub fn parse_at_times(inputs: &[String]) -> Vec<NaiveTime> {
    let mut times: Vec<NaiveTime> = Vec::new();
    for item in inputs {
        for token in item.split(',') {
            if let Some(t) = parse_time_str(token) {
                times.push(t);
            }
        }
    }
    times.sort();
    times.dedup();
    times
}

/// 基準時刻（now）から、スケジュール時刻リストの中で最も近い次の待機秒数を算出します。
/// 本日の未来に該当時刻があればその差分秒数、無ければ翌日最初の時刻までの差分秒数を返します。
pub fn calculate_next_schedule_wait(now: NaiveTime, times: &[NaiveTime]) -> Option<u64> {
    if times.is_empty() {
        return None;
    }
    let now_secs = now.num_seconds_from_midnight() as i64;
    // 本日の未来の時刻（1秒以上先）
    for t in times {
        let t_secs = t.num_seconds_from_midnight() as i64;
        if t_secs > now_secs {
            return Some((t_secs - now_secs) as u64);
        }
    }
    // 本日分が終了している場合は翌日の先頭時刻
    let first_secs = times[0].num_seconds_from_midnight() as i64;
    let diff = (86400 + first_secs) - now_secs;
    Some(diff as u64)
}

/// 遅延指定文字列（秒数, 単位付き, または HH:MM / HH:MM:SS 時刻）を解析し、待機秒数を返します。
pub fn parse_delay_to_seconds(input: &str) -> u64 {
    let now = Local::now().time();
    parse_delay_with_reference(input, now)
}

/// 基準時刻（now）を用いて遅延秒数を算出する内部関数（テスト・検証用）
pub fn parse_delay_with_reference(input: &str, now: NaiveTime) -> u64 {
    let clean = input.trim();
    if clean.is_empty() || clean == "0" {
        return 0;
    }

    // 1. 単位付きまたは純粋な秒数
    if let Some(secs) = parse_duration_to_seconds(clean) {
        return clamp_delay_seconds(secs);
    }

    // 2. 時刻指定 (HH:MM または HH:MM:SS)
    if let Some(target_time) = parse_time_str(clean) {
        let now_secs = now.num_seconds_from_midnight() as i64;
        let target_secs = target_time.num_seconds_from_midnight() as i64;
        let diff = if target_secs >= now_secs {
            target_secs - now_secs
        } else {
            (86400 + target_secs) - now_secs
        };
        return clamp_delay_seconds(diff as u64);
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

    #[test]
    fn test_parse_duration_and_interval() {
        assert_eq!(parse_duration_to_seconds("30"), Some(30));
        assert_eq!(parse_duration_to_seconds("10s"), Some(10));
        assert_eq!(parse_duration_to_seconds("30m"), Some(1800));
        assert_eq!(parse_duration_to_seconds("1h"), Some(3600));
        assert_eq!(parse_duration_to_seconds("10秒"), Some(10));
        assert_eq!(parse_duration_to_seconds("30分"), Some(1800));
        assert_eq!(parse_duration_to_seconds("1時間"), Some(3600));
        assert_eq!(parse_duration_to_seconds("invalid"), None);

        assert_eq!(parse_interval_to_seconds("30m"), Some(1800));
        assert_eq!(parse_interval_to_seconds("0"), None);
        assert_eq!(parse_interval_to_seconds("0s"), None);
    }

    #[test]
    fn test_parse_at_times() {
        let inputs = vec![
            "12:00,09:00".to_string(),
            "15:30".to_string(),
            "invalid".to_string(),
            "09:00".to_string(), // 重複
        ];
        let times = parse_at_times(&inputs);
        assert_eq!(times.len(), 3);
        assert_eq!(times[0], NaiveTime::from_hms_opt(9, 0, 0).unwrap());
        assert_eq!(times[1], NaiveTime::from_hms_opt(12, 0, 0).unwrap());
        assert_eq!(times[2], NaiveTime::from_hms_opt(15, 30, 0).unwrap());
    }

    #[test]
    fn test_calculate_next_schedule_wait() {
        let times = vec![
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
        ];

        // 1. 朝8:00 -> 次は 9:00 (1時間 = 3600秒)
        let now1 = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        assert_eq!(calculate_next_schedule_wait(now1, &times), Some(3600));

        // 2. 昼10:30 -> 次は 12:00 (1時間30分 = 5400秒)
        let now2 = NaiveTime::from_hms_opt(10, 30, 0).unwrap();
        assert_eq!(calculate_next_schedule_wait(now2, &times), Some(5400));

        // 3. 夜20:00 -> 本日の予定終了、次は翌朝 9:00 (4時間 + 9時間 = 13時間 = 46800秒)
        let now3 = NaiveTime::from_hms_opt(20, 0, 0).unwrap();
        assert_eq!(calculate_next_schedule_wait(now3, &times), Some(46800));

        // 4. 空のリスト
        assert_eq!(calculate_next_schedule_wait(now1, &[]), None);
    }

    #[test]
    fn test_cli_args_parsing_new_features() {
        use clap::Parser;

        // 新機能オプションのパース検証
        let args = CliArgs::try_parse_from([
            "MyMsg",
            "テストメッセージ",
            "--timeout",
            "10",
            "--show-progress",
            "--log",
            "app.log",
            "--copy",
            "--action",
            "calc.exe",
        ])
        .unwrap();

        assert_eq!(args.message_arg.as_deref(), Some("テストメッセージ"));
        assert_eq!(args.timeout, 10);
        assert!(args.show_progress);
        assert_eq!(args.log.as_deref(), Some(std::path::Path::new("app.log")));
        assert!(args.copy);
        assert_eq!(args.action.as_deref(), Some("calc.exe"));
    }
}
