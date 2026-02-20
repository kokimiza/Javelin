// RepositoryTrait - Event永続抽象
// 必須操作: append / loadStream
// 禁止: 詳細なQuery機能
pub mod account_master_repository;
pub mod application_settings_repository;
pub mod company_master_repository;
pub mod event_repository;
pub mod subsidiary_account_master_repository;
pub mod user_action_repository;

pub use account_master_repository::*;
pub use application_settings_repository::*;
pub use company_master_repository::*;
pub use event_repository::*;
pub use subsidiary_account_master_repository::*;
pub use user_action_repository::*;
