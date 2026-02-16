// ProjectionBuilder - ReadModel生成
// 責務: Event → Projection変換
// 再構築: 全イベント再生で生成可能

use crate::error::ApplicationResult;

pub struct ProjectionBuilder;

impl ProjectionBuilder {
    pub fn new() -> Self {
        Self
    }

    /// イベントストリームからProjectionを再構築（async）
    pub async fn rebuild_from_events(&self) -> ApplicationResult<()> {
        // TODO: Event Stream → Projection DB
        Ok(())
    }

    /// 単一イベントからProjectionを更新（async）
    pub async fn update_from_event(&self, _event_data: &[u8]) -> ApplicationResult<()> {
        // TODO: Event → Projection更新
        Ok(())
    }
}

impl Default for ProjectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}
