// 4.1 原始記録登録処理（毎日）
// 目的: 発生した経済取引を発生主義に基づき適切な会計期間へ帰属させる

use crate::dtos::{RegisterJournalEntryRequest, RegisterJournalEntryResponse};
use crate::error::ApplicationResult;

/// 仕訳登録ユースケース
#[allow(async_fn_in_trait)]
pub trait RegisterJournalEntryUseCase: Send + Sync {
    async fn execute(
        &self,
        request: RegisterJournalEntryRequest,
    ) -> ApplicationResult<RegisterJournalEntryResponse>;
}
