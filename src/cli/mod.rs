//! # CLI モジュール (`cli/mod.rs`)
//!
//! コマンドライン引数のパース、アイコン/テーマの種別定義、寸法・遅延の計算処理を提供します。
//! サブモジュール（`args`, `types`, `time`, `layout`）の各定義を再エクスポートします。

pub mod args;
pub mod layout;
pub mod time;
pub mod types;

// 後方互換性維持のためのフラットな再エクスポート
pub use args::*;
pub use layout::*;
pub use time::*;
pub use types::*;
