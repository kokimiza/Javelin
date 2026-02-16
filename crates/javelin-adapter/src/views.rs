// Views - フロントエンド構造
// layouts: 画面レイアウト構造
// pages: ページ単位のビュー
// components: 再利用可能なUI部品

pub mod components;
pub mod layouts;
pub mod pages;

// Re-export for convenience
pub use components::*;
pub use layouts::*;
pub use pages::*;
