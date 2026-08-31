//! # トースト通知モジュール (`toast.rs`)
//!
//! `notify-rust` クレートを使用して、OSネイティブ（Windows トースト通知 / macOS / Linux）の
//! デスクトップ通知センター経由でメッセージを送信します。

use crate::cli::{CliArgs, parse_icon, resolve_message};
use notify_rust::{Notification, Timeout};

/// OS標準のトースト通知を送信します。
pub fn send_toast_notification(args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    let message = resolve_message(args.message_arg.clone(), args.message_opt.clone());
    let icon_opt = args.icon.as_deref().and_then(parse_icon);

    let summary = if let Some(icon) = icon_opt {
        format!("MyMsg {} 通知", icon.symbol())
    } else {
        "MyMsg 通知".to_string()
    };

    let mut notification = Notification::new();
    notification
        .appname("MyMsg")
        .summary(&summary)
        .body(&message);

    if args.timeout > 0 {
        notification.timeout(Timeout::Milliseconds((args.timeout * 1000) as u32));
    }

    notification.show()?;
    Ok(())
}
