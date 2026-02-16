// Application Layer - ユースケース / Query / Projection制御
// 依存方向: → Domain

pub mod error;
pub mod interactor;
pub mod output_port;
pub mod projection_builder;
pub mod query_service;

// DTOs - Request/Response data transfer objects
pub mod dtos {
    pub mod request;
    pub mod response;

    // Re-export for convenience
    pub use request::*;
    pub use response::*;
}

// Input Ports - Use case trait definitions
pub mod input_ports {
    pub mod adjust_accounts;
    pub mod apply_ifrs_valuation;
    pub mod consolidate_ledger;
    pub mod generate_financial_statements;
    pub mod generate_note_draft;
    pub mod generate_trial_balance;
    pub mod lock_closing_period;
    pub mod prepare_closing;
    pub mod record_user_action;
    pub mod register_journal_entry;

    // Re-export for convenience
    pub use adjust_accounts::*;
    pub use apply_ifrs_valuation::*;
    pub use consolidate_ledger::*;
    pub use generate_financial_statements::*;
    pub use generate_note_draft::*;
    pub use generate_trial_balance::*;
    pub use lock_closing_period::*;
    pub use prepare_closing::*;
    pub use record_user_action::*;
    pub use register_journal_entry::*;
}
