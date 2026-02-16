// RepositoryTrait - Event永続抽象
// 必須操作: append / loadStream
// 禁止: Query機能

use crate::error::DomainResult;
use crate::event::DomainEvent;

/// EventRepositoryトレイト（async対応）
#[allow(async_fn_in_trait)]
pub trait EventRepository: Send + Sync {
    type Event: DomainEvent;

    /// イベントを追記
    async fn append(&self, event: Self::Event) -> DomainResult<()>;
}

/// ユーザ操作記録リポジトリ
pub trait UserActionRepository: Send + Sync {
    /// ユーザ操作を保存
    fn save_action(
        &self,
        user: &str,
        location: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<String, Box<dyn std::error::Error + Send + Sync>>> + Send;
}
