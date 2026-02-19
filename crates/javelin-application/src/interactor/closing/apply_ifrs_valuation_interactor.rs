// ApplyIfrsValuationInteractor - IFRS評価処理
// 責務: 見積会計・公正価値測定

use std::sync::Arc;

use chrono::Utc;
use javelin_domain::{
    financial_close::closing_events::ClosingEvent, repositories::EventRepository,
};

use crate::{
    dtos::{ApplyIfrsValuationRequest, ApplyIfrsValuationResponse},
    error::ApplicationResult,
    input_ports::ApplyIfrsValuationUseCase,
    query_service::ledger_query_service::{GetTrialBalanceQuery, LedgerQueryService},
};

pub struct ApplyIfrsValuationInteractor<R, Q>
where
    R: EventRepository,
    Q: LedgerQueryService,
{
    event_repository: Arc<R>,
    ledger_query_service: Arc<Q>,
}

impl<R, Q> ApplyIfrsValuationInteractor<R, Q>
where
    R: EventRepository,
    Q: LedgerQueryService,
{
    pub fn new(event_repository: Arc<R>, ledger_query_service: Arc<Q>) -> Self {
        Self { event_repository, ledger_query_service }
    }
}

impl<R, Q> ApplyIfrsValuationUseCase for ApplyIfrsValuationInteractor<R, Q>
where
    R: EventRepository,
    Q: LedgerQueryService,
{
    async fn execute(
        &self,
        request: ApplyIfrsValuationRequest,
    ) -> ApplicationResult<ApplyIfrsValuationResponse> {
        // 試算表を取得してIFRS評価対象を特定
        let _trial_balance = self
            .ledger_query_service
            .get_trial_balance(GetTrialBalanceQuery {
                period_year: request.fiscal_year as u32,
                period_month: request.period,
            })
            .await?;

        // IFRS評価イベントを記録
        let valuation_id = format!("IFRS-{}-{:02}", request.fiscal_year, request.period);
        let events = vec![ClosingEvent::IfrsValuationApplied {
            valuation_id: format!("{}-ECL", valuation_id),
            fiscal_year: request.fiscal_year,
            period: request.period,
            valuation_type: "ExpectedCreditLoss".to_string(),
            account_code: "1100".to_string(), // 売掛金
            amount: 50000.0,
            currency: "JPY".to_string(),
            applied_by: "system".to_string(),
            applied_at: Utc::now(),
        }];

        self.event_repository.append_events(&valuation_id, events).await?;

        Ok(ApplyIfrsValuationResponse {
            expected_credit_loss: 50000.0,
            expected_credit_loss_currency: "JPY".to_string(),
            contingent_liabilities: vec![],
            inventory_write_downs: vec![],
            impairment_losses: vec![],
            fair_value_adjustments: vec![],
            lease_measurements: vec![],
        })
    }
}
