// Request DTOs - InputPort → Interactor
// Command側のデータ転送オブジェクト

use chrono::NaiveDate;
use javelin_domain::financial_close::journal_entry::{JudgmentType, RiskClassification};
use javelin_domain::financial_close::{AccountCode, AccountingPeriod, Amount};

// ============================================================================
// ユーザ操作記録
// ============================================================================

#[derive(Debug, Clone)]
pub struct RecordUserActionRequest {
    pub user: String,
    pub location: String,
    pub action: String,
}

// ============================================================================
// 4.1 原始記録登録処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct RegisterJournalEntryRequest {
    pub transaction_date: NaiveDate,
    pub accounting_period: AccountingPeriod,
    pub debit_account: AccountCode,
    pub credit_account: AccountCode,
    pub amount: Amount,
    pub evidence_reference: String,
    pub risk_classification: RiskClassification,
    pub judgment_type: JudgmentType,
    pub description: String,
}

// 簡易版（テスト・デモ用）
impl RegisterJournalEntryRequest {
    pub fn simple(
        _date: impl Into<String>,
        description: impl Into<String>,
        debit_account: impl Into<String>,
        debit_amount: i64,
        credit_account: impl Into<String>,
        _credit_amount: i64,
    ) -> Self {
        Self {
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            accounting_period: AccountingPeriod::new(2024, 1).expect("Invalid accounting period"),
            debit_account: AccountCode::new(debit_account.into())
                .expect("Invalid debit account code"),
            credit_account: AccountCode::new(credit_account.into())
                .expect("Invalid credit account code"),
            amount: Amount::new(debit_amount).expect("Invalid amount"),
            evidence_reference: "DEMO-001".to_string(),
            risk_classification: RiskClassification::Low,
            judgment_type: JudgmentType::Routine,
            description: description.into(),
        }
    }
}

// ============================================================================
// 4.2 元帳集約処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct ConsolidateLedgerRequest {
    pub accounting_period: AccountingPeriod,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}

// ============================================================================
// 4.3 締準備処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct PrepareClosingRequest {
    pub accounting_period: AccountingPeriod,
}

// ============================================================================
// 4.4 締日固定処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct LockClosingPeriodRequest {
    pub accounting_period: AccountingPeriod,
    pub locked_by: String,
}

// ============================================================================
// 4.5 試算表生成処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct GenerateTrialBalanceRequest {
    pub accounting_period: AccountingPeriod,
}

// ============================================================================
// 4.6 注記草案生成処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct GenerateNoteDraftRequest {
    pub accounting_period: AccountingPeriod,
}

// ============================================================================
// 4.7 勘定補正処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct AdjustAccountsRequest {
    pub accounting_period: AccountingPeriod,
}

// ============================================================================
// 4.8 IFRS評価処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct ApplyIfrsValuationRequest {
    pub accounting_period: AccountingPeriod,
}

// ============================================================================
// 4.9 財務諸表生成処理
// ============================================================================

#[derive(Debug, Clone)]
pub struct GenerateFinancialStatementsRequest {
    pub accounting_period: AccountingPeriod,
}
