// Adapter Layer - 外部入出力変換
// 依存方向: → Application

pub mod controller;
pub mod error;
pub mod error_handler;
pub mod presenter;
pub mod view_router;
pub mod views;

// Re-export for convenience
pub use error_handler::*;
pub use views::*;
