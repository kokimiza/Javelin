// EventRepository - Event永続抽象
// 必須操作: append / loadStream
// 禁止: 詳細なQuery機能

use crate::{error::DomainResult, event::DomainEvent};

/// EventRepositoryトレイト（async対応）
///
/// ドメイン層が定義するイベント永続化の抽象インターフェース。
/// Infrastructure層でこのトレイトを実装し、DIコンテナから注入される。
#[allow(async_fn_in_trait)]
pub trait EventRepository: Send + Sync {
    type Event: DomainEvent;

    /// イベントを追記
    async fn append(&self, event: Self::Event) -> DomainResult<()>;

    /// 複数イベントを一括追記
    ///
    /// # Arguments
    /// * `aggregate_id` - 集約ID
    /// * `events` - 保存するドメインイベントのリスト
    ///
    /// # Returns
    /// 最後に保存されたイベントのシーケンス番号
    async fn append_events<T>(&self, aggregate_id: &str, events: Vec<T>) -> DomainResult<u64>
    where
        T: serde::Serialize + Send + 'static;

    /// 指定された集約IDのイベントストリームを取得
    ///
    /// # Arguments
    /// * `aggregate_id` - 集約ID
    ///
    /// # Returns
    /// イベントのベクタ（シーケンス順）
    async fn get_events(&self, aggregate_id: &str) -> DomainResult<Vec<serde_json::Value>>;

    /// 指定されたシーケンス番号以降の全イベントを取得
    ///
    /// # Arguments
    /// * `from_sequence` - 開始シーケンス番号
    ///
    /// # Returns
    /// イベントのベクタ（シーケンス順）
    async fn get_all_events(&self, from_sequence: u64) -> DomainResult<Vec<serde_json::Value>>;

    /// 最新シーケンス番号を取得
    async fn get_latest_sequence(&self) -> DomainResult<u64>;
}
