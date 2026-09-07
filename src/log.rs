//! # ロギングモジュール (`log.rs`)
//!
//! 指定されたログファイルへのイベント追記（起動、終了、クリップボードコピー、アクション実行など）を提供します。

use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// ログイベントの種類
#[derive(Debug, Clone)]
pub enum LogEvent<'a> {
    /// 通知ウィンドウの表示開始
    Open {
        message: &'a str,
        count: u64,
        timeout_secs: u64,
    },
    /// 通知ウィンドウの終了
    Close { reason: &'a str, elapsed_secs: f32 },
    /// クリップボードコピー実行
    Copy,
    /// 外部コマンド実行
    Action { cmd: &'a str, success: bool },
}

/// ログファイルへイベントを追記します。
pub fn append_log(path: &Path, event: LogEvent) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");

    let line = match event {
        LogEvent::Open {
            message,
            count,
            timeout_secs,
        } => {
            // 改行を含むメッセージは1行ログ向けにエスケープ
            let escaped_msg = message.replace('\r', "").replace('\n', "\\n");
            format!(
                "[{now}] [OPEN] count={count} timeout={timeout_secs}s message=\"{escaped_msg}\"\n"
            )
        }
        LogEvent::Close {
            reason,
            elapsed_secs,
        } => {
            format!("[{now}] [CLOSE] reason={reason} elapsed={elapsed_secs:.2}s\n")
        }
        LogEvent::Copy => {
            format!("[{now}] [COPY] Message copied to clipboard\n")
        }
        LogEvent::Action { cmd, success } => {
            let status = if success { "success" } else { "failed" };
            format!("[{now}] [ACTION] cmd=\"{cmd}\" status={status}\n")
        }
    };

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = file.write_all(line.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_append_log_flow() {
        let temp_dir = std::env::temp_dir();
        let log_file = temp_dir.join("mymsg_test_log.log");

        // 既存テストファイルがあれば削除
        if log_file.exists() {
            let _ = fs::remove_file(&log_file);
        }

        // ログイベントを書き込み
        append_log(
            &log_file,
            LogEvent::Open {
                message: "テスト\nメッセージ",
                count: 1,
                timeout_secs: 5,
            },
        );
        append_log(&log_file, LogEvent::Copy);
        append_log(
            &log_file,
            LogEvent::Action {
                cmd: "echo hello",
                success: true,
            },
        );
        append_log(
            &log_file,
            LogEvent::Close {
                reason: "Timeout",
                elapsed_secs: 5.01,
            },
        );

        let content = fs::read_to_string(&log_file).unwrap();
        assert!(content.contains("[OPEN] count=1 timeout=5s message=\"テスト\\nメッセージ\""));
        assert!(content.contains("[COPY] Message copied to clipboard"));
        assert!(content.contains("[ACTION] cmd=\"echo hello\" status=success"));
        assert!(content.contains("[CLOSE] reason=Timeout elapsed=5.01s"));

        // クリーンアップ
        let _ = fs::remove_file(&log_file);
    }
}
