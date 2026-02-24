// ClosingController - 月次決算処理コントローラ
// 責務: 月次決算関連のユースケースを呼び出す

use std::sync::Arc;

use javelin_application::{
    dtos::{
        AdjustAccountsRequest, AdjustAccountsResponse, ApplyIfrsValuationRequest,
        ApplyIfrsValuationResponse, ConsolidateLedgerRequest, ConsolidateLedgerResponse,
        GenerateFinancialStatementsRequest, GenerateFinancialStatementsResponse,
        GenerateNoteDraftRequest, GenerateNoteDraftResponse, GenerateTrialBalanceRequest,
        GenerateTrialBalanceResponse, LockClosingPeriodRequest, LockClosingPeriodResponse,
        PrepareClosingRequest, PrepareClosingResponse,
    },
    input_ports::{
        AdjustAccountsUseCase, ApplyIfrsValuationUseCase, ConsolidateLedgerUseCase,
        GenerateFinancialStatementsUseCase, GenerateNoteDraftUseCase, GenerateTrialBalanceUseCase,
        LockClosingPeriodUseCase, PrepareClosingUseCase,
    },
};

use crate::error::AdapterResult;

pub struct ClosingController<
    Consolidate,
    Prepare,
    Lock,
    TrialBalance,
    NoteDraft,
    Adjust,
    Ifrs,
    Financial,
> where
    Consolidate: ConsolidateLedgerUseCase,
    Prepare: PrepareClosingUseCase,
    Lock: LockClosingPeriodUseCase,
    TrialBalance: GenerateTrialBalanceUseCase,
    NoteDraft: GenerateNoteDraftUseCase,
    Adjust: AdjustAccountsUseCase,
    Ifrs: ApplyIfrsValuationUseCase,
    Financial: GenerateFinancialStatementsUseCase,
{
    consolidate_ledger: Arc<Consolidate>,
    prepare_closing: Arc<Prepare>,
    lock_closing_period: Arc<Lock>,
    generate_trial_balance: Arc<TrialBalance>,
    generate_note_draft: Arc<NoteDraft>,
    adjust_accounts: Arc<Adjust>,
    apply_ifrs_valuation: Arc<Ifrs>,
    generate_financial_statements: Arc<Financial>,
}

impl<Consolidate, Prepare, Lock, TrialBalance, NoteDraft, Adjust, Ifrs, Financial>
    ClosingController<Consolidate, Prepare, Lock, TrialBalance, NoteDraft, Adjust, Ifrs, Financial>
where
    Consolidate: ConsolidateLedgerUseCase,
    Prepare: PrepareClosingUseCase,
    Lock: LockClosingPeriodUseCase,
    TrialBalance: GenerateTrialBalanceUseCase,
    NoteDraft: GenerateNoteDraftUseCase,
    Adjust: AdjustAccountsUseCase,
    Ifrs: ApplyIfrsValuationUseCase,
    Financial: GenerateFinancialStatementsUseCase,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        consolidate_ledger: Arc<Consolidate>,
        prepare_closing: Arc<Prepare>,
        lock_closing_period: Arc<Lock>,
        generate_trial_balance: Arc<TrialBalance>,
        generate_note_draft: Arc<NoteDraft>,
        adjust_accounts: Arc<Adjust>,
        apply_ifrs_valuation: Arc<Ifrs>,
        generate_financial_statements: Arc<Financial>,
    ) -> Self {
        Self {
            consolidate_ledger,
            prepare_closing,
            lock_closing_period,
            generate_trial_balance,
            generate_note_draft,
            adjust_accounts,
            apply_ifrs_valuation,
            generate_financial_statements,
        }
    }

    /// 元帳集約処理
    pub async fn consolidate_ledger(
        &self,
        request: ConsolidateLedgerRequest,
    ) -> AdapterResult<ConsolidateLedgerResponse> {
        self.consolidate_ledger
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 締準備処理
    pub async fn prepare_closing(
        &self,
        request: PrepareClosingRequest,
    ) -> AdapterResult<PrepareClosingResponse> {
        self.prepare_closing
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 締日固定処理
    pub async fn lock_closing_period(
        &self,
        request: LockClosingPeriodRequest,
    ) -> AdapterResult<LockClosingPeriodResponse> {
        self.lock_closing_period
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 試算表生成処理
    pub async fn generate_trial_balance(
        &self,
        request: GenerateTrialBalanceRequest,
    ) -> AdapterResult<GenerateTrialBalanceResponse> {
        self.generate_trial_balance
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 注記草案生成処理
    pub async fn generate_note_draft(
        &self,
        request: GenerateNoteDraftRequest,
    ) -> AdapterResult<GenerateNoteDraftResponse> {
        self.generate_note_draft
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 勘定補正処理
    pub async fn adjust_accounts(
        &self,
        request: AdjustAccountsRequest,
    ) -> AdapterResult<AdjustAccountsResponse> {
        self.adjust_accounts
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// IFRS評価処理
    pub async fn apply_ifrs_valuation(
        &self,
        request: ApplyIfrsValuationRequest,
    ) -> AdapterResult<ApplyIfrsValuationResponse> {
        self.apply_ifrs_valuation
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }

    /// 財務諸表生成処理
    pub async fn generate_financial_statements(
        &self,
        request: GenerateFinancialStatementsRequest,
    ) -> AdapterResult<GenerateFinancialStatementsResponse> {
        self.generate_financial_statements
            .execute(request)
            .await
            .map_err(crate::error::AdapterError::ApplicationError)
    }
}
