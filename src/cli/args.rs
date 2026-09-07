//! # CLI コマンドライン引数定義サブモジュール (`cli/args.rs`)
//!
//! `clap::Parser` によるコマンドライン構文定義とヘルプ文章を提供します。

use clap::Parser;

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

    /// ポップアップ表示時に通知音・ビープ音を鳴らす
    #[arg(
        long = "sound",
        visible_alias = "beep",
        help = "ポップアップ表示時に通知音を再生",
        long_help = "ポップアップ表示時にシステム通知音（またはビープ音）を鳴らします。\n\
                     ・--icon が指定されている場合はアイコン種別に適した通知音を再生します。\n\
                     ・エイリアス: --beep"
    )]
    pub sound: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_args_parsing_new_features() {
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
            "--sound",
        ])
        .unwrap();

        assert_eq!(args.message_arg.as_deref(), Some("テストメッセージ"));
        assert_eq!(args.timeout, 10);
        assert!(args.show_progress);
        assert_eq!(args.log.as_deref(), Some(std::path::Path::new("app.log")));
        assert!(args.copy);
        assert_eq!(args.action.as_deref(), Some("calc.exe"));
        assert!(args.sound);
    }
}
