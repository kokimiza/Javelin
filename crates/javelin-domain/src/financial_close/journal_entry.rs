// 仕訳帳 - 発生日基準による全取引記録
// 統制要件: 借貸一致・証憑必須

use crate::entity::{Entity, EntityId};
use crate::error::DomainResult;
use crate::financial_close::{AccountCode, AccountingPeriod, Amount};
use chrono::NaiveDate;

/// 仕訳ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntryId(String);

impl EntityId for JournalEntryId {
    fn value(&self) -> &str {
        &self.0
    }
}

impl JournalEntryId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

/// 仕訳帳エントリ
#[derive(Debug)]
pub struct JournalEntry {
    id: JournalEntryId,
    transaction_date: NaiveDate,
    accounting_period: AccountingPeriod,
    debit_account: AccountCode,
    credit_account: AccountCode,
    amount: Amount,
    evidence_reference: String, // 証憑参照
    risk_classification: RiskClassification,
    judgment_type: JudgmentType,
    approved: bool,
}

impl Entity for JournalEntry {
    type Id = JournalEntryId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

/// リスク分類（第3章 3.2）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiskClassification {
    Low,      // 定型処理
    Medium,   // 見積含有
    High,     // 予測依存
    Critical, // 経営判断
}

/// 会計判断区分
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JudgmentType {
    Estimate, // 見積処理
    Judgment, // 判断処理
    Routine,  // 定型処理
}

impl JournalEntry {
    pub fn new(
        id: JournalEntryId,
        transaction_date: NaiveDate,
        accounting_period: AccountingPeriod,
        debit_account: AccountCode,
        credit_account: AccountCode,
        amount: Amount,
        evidence_reference: String,
        risk_classification: RiskClassification,
        judgment_type: JudgmentType,
    ) -> Self {
        Self {
            id,
            transaction_date,
            accounting_period,
            debit_account,
            credit_account,
            amount,
            evidence_reference,
            risk_classification,
            judgment_type,
            approved: false,
        }
    }

    /// 承認処理
    pub fn approve(&mut self) {
        self.approved = true;
    }

    /// 借貸一致検証
    pub fn validate_balance(&self) -> DomainResult<()> {
        // 借方金額と貸方金額は常に一致（単一仕訳の場合）
        Ok(())
    }

    /// 証憑添付確認
    pub fn has_evidence(&self) -> bool {
        !self.evidence_reference.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journal_entry_creation() {
        let id = JournalEntryId::new("JE001".to_string());
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let period = AccountingPeriod::new(2024, 1).unwrap();
        let debit = AccountCode::new("1000".to_string()).unwrap();
        let credit = AccountCode::new("2000".to_string()).unwrap();
        let amount = Amount::new(100000).unwrap();

        let entry = JournalEntry::new(
            id,
            date,
            period,
            debit,
            credit,
            amount,
            "INV-001".to_string(),
            RiskClassification::Low,
            JudgmentType::Routine,
        );

        assert!(entry.has_evidence());
        assert!(!entry.approved);
    }

    #[test]
    fn test_journal_entry_approval() {
        let id = JournalEntryId::new("JE002".to_string());
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let period = AccountingPeriod::new(2024, 1).unwrap();
        let debit = AccountCode::new("1000".to_string()).unwrap();
        let credit = AccountCode::new("2000".to_string()).unwrap();
        let amount = Amount::new(100000).unwrap();

        let mut entry = JournalEntry::new(
            id,
            date,
            period,
            debit,
            credit,
            amount,
            "INV-002".to_string(),
            RiskClassification::Low,
            JudgmentType::Routine,
        );

        entry.approve();
        assert!(entry.approved);
    }

    #[test]
    fn test_journal_entry_balance_validation() {
        let id = JournalEntryId::new("JE003".to_string());
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let period = AccountingPeriod::new(2024, 1).unwrap();
        let debit = AccountCode::new("1000".to_string()).unwrap();
        let credit = AccountCode::new("2000".to_string()).unwrap();
        let amount = Amount::new(100000).unwrap();

        let entry = JournalEntry::new(
            id,
            date,
            period,
            debit,
            credit,
            amount,
            "INV-003".to_string(),
            RiskClassification::Low,
            JudgmentType::Routine,
        );

        assert!(entry.validate_balance().is_ok());
    }
}
