// Route - Screen identifier enum
// Identifies which screen to display (separate from page state)

/// Screen identifier for navigation
///
/// Identifies which screen to display without containing any state.
/// Each Route corresponds to a PageState implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Route {
    /// Home screen - Main menu
    Home,

    /// 101 - Journal entry form
    JournalEntry,

    /// 102 - Journal entry search
    Search,

    /// 401 - Ledger view
    Ledger,

    /// Ledger detail view (drill-down from Ledger)
    LedgerDetail,

    /// 201 - Ledger consolidation
    LedgerConsolidation,

    /// 201E - Ledger consolidation execution
    LedgerConsolidationExecution,

    /// 301 - Closing preparation
    ClosingPreparation,

    /// 301E - Closing preparation execution
    ClosingPreparationExecution,

    /// 302 - Closing lock
    ClosingLock,

    /// 303 - Trial balance
    TrialBalance,

    /// 304 - Note draft generation
    NoteDraft,

    /// 305 - Account adjustment
    AccountAdjustment,

    /// 305E - Account adjustment execution
    AccountAdjustmentExecution,

    /// 306 - IFRS valuation
    IfrsValuation,

    /// 306E - IFRS valuation execution
    IfrsValuationExecution,

    /// 307 - Financial statement generation
    FinancialStatement,

    /// 307E - Financial statement generation execution
    FinancialStatementExecution,

    /// 901 - Account master management
    AccountMaster,

    /// 902 - Subsidiary account master management
    SubsidiaryAccountMaster,

    /// 903 - Application settings management
    ApplicationSettings,

    /// 904 - Data import
    DataImport,

    /// 905 - Data export
    DataExport,
}
