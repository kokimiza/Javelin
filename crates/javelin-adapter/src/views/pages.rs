// Pages - ページ単位のビュー
// 責務: 各画面の実装

pub mod account_adjustment_page;
pub mod closing_lock_page;
pub mod closing_page;
pub mod closing_preparation_page;
pub mod financial_statement_page;
pub mod home_page;
pub mod ifrs_valuation_page;
pub mod journal_entry_form_page;
pub mod ledger_page;
pub mod ledger_view_page;
pub mod note_draft_page;

pub use account_adjustment_page::*;
pub use closing_lock_page::*;
pub use closing_page::*;
pub use closing_preparation_page::*;
pub use financial_statement_page::*;
pub use home_page::*;
pub use ifrs_valuation_page::*;
pub use journal_entry_form_page::*;
pub use ledger_page::*;
pub use ledger_view_page::*;
pub use note_draft_page::*;
