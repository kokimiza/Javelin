// PageStateResolver - ルートからPageStateへの解決
// 責務: Route → PageState のマッピング

use std::sync::Arc;

use javelin_adapter::{HomePageState, PageState, PresenterRegistry, Route, SearchPageState};

use crate::app_error::{AppError, AppResult};

/// PageStateの解決を担当
pub struct PageStateResolver {
    presenter_registry: Arc<PresenterRegistry>,
}

impl PageStateResolver {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { presenter_registry }
    }

    /// ルートからPageStateを解決
    pub fn resolve(&self, route: Route) -> AppResult<Box<dyn PageState>> {
        match route {
            Route::Home => Ok(Box::new(HomePageState::new())),
            Route::Search => {
                Ok(Box::new(SearchPageState::new(Arc::clone(&self.presenter_registry))))
            }
            Route::JournalEntry => Ok(Box::new(javelin_adapter::JournalEntryPageState::new(
                Arc::clone(&self.presenter_registry),
            ))),
            Route::Ledger => Ok(Box::new(javelin_adapter::LedgerPageState::new())),
            Route::LedgerDetail => Ok(Box::new(javelin_adapter::LedgerDetailPageState::new())),
            Route::LedgerConsolidation => {
                Ok(Box::new(javelin_adapter::LedgerConsolidationPageState::new()))
            }
            Route::ClosingPreparation => {
                Ok(Box::new(javelin_adapter::ClosingPreparationPageState::new()))
            }
            Route::ClosingLock => Ok(Box::new(javelin_adapter::ClosingLockPageState::new())),
            Route::TrialBalance => Ok(Box::new(javelin_adapter::TrialBalancePageState::new())),
            Route::AccountAdjustment => {
                Ok(Box::new(javelin_adapter::AccountAdjustmentPageState::new()))
            }
            Route::NoteDraft => Ok(Box::new(javelin_adapter::NoteDraftPageState::new())),
            Route::IfrsValuation => Ok(Box::new(javelin_adapter::IfrsValuationPageState::new())),
            Route::FinancialStatement => {
                Ok(Box::new(javelin_adapter::FinancialStatementPageState::new()))
            }
            Route::AccountMaster => Ok(Box::new(javelin_adapter::AccountMasterPageState::new(
                Arc::clone(&self.presenter_registry),
            ))),
            Route::SubsidiaryAccountMaster => {
                Ok(Box::new(javelin_adapter::SubsidiaryAccountMasterPageState::new(Arc::clone(
                    &self.presenter_registry,
                ))))
            }
            Route::ApplicationSettings => {
                Ok(Box::new(javelin_adapter::ApplicationSettingsPageState::new(Arc::clone(
                    &self.presenter_registry,
                ))))
            }
            _ => Err(AppError::NotImplemented(format!("Route {:?} not yet implemented", route))),
        }
    }
}
