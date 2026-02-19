// RepositoryTrait - Event永続抽象
// 必須操作: append / loadStream
// 禁止: 詳細なQuery機能
pub mod event_repository;
pub mod system_master_repository;
pub mod user_action_repository;

pub use event_repository::*;
pub use system_master_repository::*;
pub use user_action_repository::*;
