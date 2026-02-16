// Infrastructure Layer - 永続化 / 外部技術実装
// 依存方向: → Domain

pub mod commands;
pub mod error;
pub mod event_store;
pub mod projection_db;
pub mod queries;

// Re-export for convenience
pub use commands::{EventRepositoryImpl, UserActionRepositoryImpl};
pub use queries::master_data_loader_impl;
