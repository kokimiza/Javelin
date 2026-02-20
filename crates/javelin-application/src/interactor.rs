// Interactor - Command実装
// 責務: ドメイン操作調整
// 利用対象: Entity / ValueObject / DomainService / RepositoryTrait

pub mod account_master_interactor;
pub mod application_settings_interactor;
pub mod closing;
pub mod company_master_interactor;
pub mod journal_entry;
pub mod master_data;
pub mod subsidiary_account_master_interactor;

pub use account_master_interactor::{
    AccountMasterInteractor, GetAccountMastersQuery, RegisterAccountMasterRequest,
    UpdateAccountMasterRequest,
};
pub use application_settings_interactor::{
    ApplicationSettingsInteractor, GetApplicationSettingsQuery, UpdateApplicationSettingsRequest,
};
pub use closing::{
    AdjustAccountsInteractor, ApplyIfrsValuationInteractor, ConsolidateLedgerInteractor,
    GenerateFinancialStatementsInteractor, GenerateNoteDraftInteractor,
    GenerateTrialBalanceInteractor, LockClosingPeriodInteractor, PrepareClosingInteractor,
};
pub use company_master_interactor::{
    CompanyMasterInteractor, GetCompanyMastersQuery, RegisterCompanyMasterRequest,
    UpdateCompanyMasterRequest,
};
pub use journal_entry::{
    ApproveJournalEntryInteractor, CancelJournalEntryInteractor, CorrectJournalEntryInteractor,
    CreateAdditionalEntryInteractor, CreateReclassificationEntryInteractor,
    CreateReplacementEntryInteractor, CreateReversalEntryInteractor,
    DeleteDraftJournalEntryInteractor, RegisterJournalEntryInteractor,
    RejectJournalEntryInteractor, ReverseJournalEntryInteractor, SubmitForApprovalInteractor,
    UpdateDraftJournalEntryInteractor,
};
pub use master_data::{LoadAccountMasterInteractor, RecordUserActionInteractor};
pub use subsidiary_account_master_interactor::SubsidiaryAccountMasterInteractor;

#[cfg(test)]
mod interactor_property_tests;

#[cfg(test)]
mod interactor_unit_tests;
