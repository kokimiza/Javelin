// LedgerController実装
// 元帳・試算表照会に関する外部入力を受け付ける

use std::sync::Arc;

use javelin_application::query_service::{
    GetLedgerQuery, GetTrialBalanceQuery, LedgerQueryService,
};

/// 元帳コントローラ
///
/// 元帳・試算表照会に関するすべての操作を受け付ける。
/// QueryServiceへの委譲のみを行い、ビジネスロジックは含まない。
pub struct LedgerController<L>
where
    L: LedgerQueryService,
{
    ledger_query_service: Arc<L>,
}

impl<L> LedgerController<L>
where
    L: LedgerQueryService,
{
    /// 新しいコントローラインスタンスを作成
    pub fn new(ledger_query_service: Arc<L>) -> Self {
        Self { ledger_query_service }
    }

    /// 元帳を取得
    pub async fn get_ledger(&self, query: GetLedgerQuery) -> Result<(), String> {
        self.ledger_query_service
            .get_ledger(query)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    /// 試算表を取得
    pub async fn get_trial_balance(&self, query: GetTrialBalanceQuery) -> Result<(), String> {
        self.ledger_query_service
            .get_trial_balance(query)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}
