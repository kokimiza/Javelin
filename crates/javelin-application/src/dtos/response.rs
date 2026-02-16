// Response DTOs - Interactor → OutputPort → Presenter
// Query結果およびCommand実行結果のデータ転送オブジェクト

use chrono::{DateTime, Utc};
use javelin_domain::financial_close::journal_entry::JournalEntryId;
use javelin_domain::financial_close::{AccountCode, Amount};

// ============================================================================
// ユーザ操作記録
// ============================================================================

#[derive(Debug, Clone)]
pub struct RecordUserActionResponse {
    pub action_id: String,
    pub recorded_at: DateTime<Utc>,
}

// ============================================================================
// 4.1 原始記録登録処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct RegisterJournalEntryResponse {
    pub journal_entry_id: JournalEntryId,
    pub registered_at: DateTime<Utc>,
}

// 簡易版（テスト・デモ用）
impl RegisterJournalEntryResponse {
    pub fn simple(
        journal_entry_id: impl Into<String>,
        _success: bool,
        _message: impl Into<String>,
    ) -> Self {
        Self {
            journal_entry_id: JournalEntryId::new(journal_entry_id.into()),
            registered_at: Utc::now(),
        }
    }
}

// ============================================================================
// 4.2 元帳集約処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct ConsolidateLedgerResponse {
    pub processed_entries_count: usize,
    pub updated_accounts_count: usize,
    pub discrepancies: Vec<LedgerDiscrepancyDto>,
}

#[derive(Debug, Clone)]
pub struct LedgerDiscrepancyDto {
    pub account_code: AccountCode,
    pub general_ledger_balance: Amount,
    pub subsidiary_ledger_balance: Amount,
    pub difference: Amount,
}

// ============================================================================
// 4.3 締準備処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct PrepareClosingResponse {
    pub unregistered_transactions_count: usize,
    pub bank_reconciliation_differences: Vec<BankReconciliationDifferenceDto>,
    pub accrual_entries_created: usize,
    pub provisional_financial_statements_generated: bool,
}

#[derive(Debug, Clone)]
pub struct BankReconciliationDifferenceDto {
    pub bank_account: String,
    pub bank_balance: Amount,
    pub cash_log_balance: Amount,
    pub difference: Amount,
}

// ============================================================================
// 4.4 締日固定処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct LockClosingPeriodResponse {
    pub locked_entries_count: usize,
    pub locked_at: DateTime<Utc>,
    pub audit_log_id: String,
}

// ============================================================================
// 4.5 試算表生成処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct GenerateTrialBalanceResponse {
    pub total_debit: Amount,
    pub total_credit: Amount,
    pub is_balanced: bool,
    pub account_balances: Vec<AccountBalanceDto>,
    pub temporary_account_balances: Vec<AccountBalanceDto>,
    pub foreign_exchange_differences: Vec<ForeignExchangeDifferenceDto>,
}

#[derive(Debug, Clone)]
pub struct AccountBalanceDto {
    pub account_code: AccountCode,
    pub debit_balance: Amount,
    pub credit_balance: Amount,
    pub net_balance: Amount,
}

#[derive(Debug, Clone)]
pub struct ForeignExchangeDifferenceDto {
    pub account_code: AccountCode,
    pub original_amount: Amount,
    pub exchange_rate: f64,
    pub converted_amount: Amount,
    pub difference: Amount,
}

// ============================================================================
// 4.6 注記草案生成処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct GenerateNoteDraftResponse {
    pub accounting_policies: Vec<String>,
    pub significant_estimates: Vec<String>,
    pub account_breakdowns: Vec<AccountBreakdownDto>,
    pub note_draft: String,
}

#[derive(Debug, Clone)]
pub struct AccountBreakdownDto {
    pub account_code: AccountCode,
    pub components: Vec<String>,
}

// ============================================================================
// 4.7 勘定補正処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct AdjustAccountsResponse {
    pub adjustment_entries_created: usize,
    pub reclassified_accounts: Vec<AccountReclassificationDto>,
    pub tax_effect_adjustments: Vec<TaxEffectAdjustmentDto>,
}

#[derive(Debug, Clone)]
pub struct AccountReclassificationDto {
    pub from_account: AccountCode,
    pub to_account: AccountCode,
    pub amount: Amount,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct TaxEffectAdjustmentDto {
    pub temporary_difference: Amount,
    pub tax_rate: f64,
    pub deferred_tax_amount: Amount,
}

// ============================================================================
// 4.8 IFRS評価処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct ApplyIfrsValuationResponse {
    pub expected_credit_loss: Amount,
    pub contingent_liabilities: Vec<ContingentLiabilityDto>,
    pub inventory_write_downs: Vec<InventoryWriteDownDto>,
    pub impairment_losses: Vec<ImpairmentLossDto>,
    pub fair_value_adjustments: Vec<FairValueAdjustmentDto>,
    pub lease_measurements: Vec<LeaseMeasurementDto>,
}

#[derive(Debug, Clone)]
pub struct ContingentLiabilityDto {
    pub description: String,
    pub probability: f64,
    pub estimated_amount: Amount,
}

#[derive(Debug, Clone)]
pub struct InventoryWriteDownDto {
    pub item: String,
    pub cost: Amount,
    pub net_realizable_value: Amount,
    pub write_down_amount: Amount,
}

#[derive(Debug, Clone)]
pub struct ImpairmentLossDto {
    pub asset: String,
    pub carrying_amount: Amount,
    pub recoverable_amount: Amount,
    pub impairment_loss: Amount,
}

#[derive(Debug, Clone)]
pub struct FairValueAdjustmentDto {
    pub financial_asset: String,
    pub book_value: Amount,
    pub fair_value: Amount,
    pub adjustment: Amount,
}

#[derive(Debug, Clone)]
pub struct LeaseMeasurementDto {
    pub lease_contract: String,
    pub right_of_use_asset: Amount,
    pub lease_liability: Amount,
}

// ============================================================================
// 4.9 財務諸表生成処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct GenerateFinancialStatementsResponse {
    pub statement_of_financial_position: StatementOfFinancialPositionDto,
    pub statement_of_profit_or_loss: StatementOfProfitOrLossDto,
    pub statement_of_changes_in_equity: StatementOfChangesInEquityDto,
    pub statement_of_cash_flows: StatementOfCashFlowsDto,
    pub financial_indicators: FinancialIndicatorsDto,
    pub cross_check_passed: bool,
}

#[derive(Debug, Clone)]
pub struct StatementOfFinancialPositionDto {
    pub current_assets: Amount,
    pub non_current_assets: Amount,
    pub current_liabilities: Amount,
    pub non_current_liabilities: Amount,
    pub equity: Amount,
}

#[derive(Debug, Clone)]
pub struct StatementOfProfitOrLossDto {
    pub revenue: Amount,
    pub cost_of_sales: Amount,
    pub gross_profit: Amount,
    pub operating_expenses: Amount,
    pub operating_profit: Amount,
    pub net_profit: Amount,
}

#[derive(Debug, Clone)]
pub struct StatementOfChangesInEquityDto {
    pub opening_balance: Amount,
    pub net_profit: Amount,
    pub dividends: Amount,
    pub closing_balance: Amount,
}

#[derive(Debug, Clone)]
pub struct StatementOfCashFlowsDto {
    pub operating_activities: Amount,
    pub investing_activities: Amount,
    pub financing_activities: Amount,
    pub net_change_in_cash: Amount,
}

#[derive(Debug, Clone)]
pub struct FinancialIndicatorsDto {
    pub roe: f64, // Return on Equity
    pub roa: f64, // Return on Assets
    pub current_ratio: f64,
    pub debt_to_equity_ratio: f64,
}
