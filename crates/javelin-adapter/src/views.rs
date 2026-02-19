// Views - フロントエンド構造
// layouts: 画面レイアウト構造
// pages: ページ単位のビュー
// components: 再利用可能なUI部品
// utils: ユーティリティマクロ
// terminal_manager: ターミナル管理
// page_navigator: ページ遷移管理

pub mod components;
pub mod layouts;
pub mod page_navigator;
pub mod pages;
pub mod terminal_manager;
pub mod utils;

// Re-export for convenience
pub use page_navigator::*;
pub use terminal_manager::*;
