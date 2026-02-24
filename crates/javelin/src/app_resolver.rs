// PageStateResolver - ルートからPageStateへの解決
// 責務: Route → PageState のマッピング

use std::sync::Arc;

use javelin_adapter::{
    HomePageState, PageState, PresenterRegistry, Route, SearchPageState, navigation::Controllers,
};

use crate::app_error::{AppError, AppResult};

/// PageStateの解決を担当
pub struct PageStateResolver {
    presenter_registry: Arc<PresenterRegistry>,
    controllers: Arc<Controllers>,
}

impl PageStateResolver {
    pub fn new(presenter_registry: Arc<PresenterRegistry>, controllers: Arc<Controllers>) -> Self {
        Self { presenter_registry, controllers }
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
                Ok(Box::new(javelin_adapter::LedgerConsolidationPageState::new(&self.controllers)))
            }
            Route::LedgerConsolidationExecution => {
                Ok(Box::new(javelin_adapter::LedgerConsolidationExecutionPageState::new()))
            }
            Route::ClosingPreparation => {
                Ok(Box::new(javelin_adapter::ClosingPreparationPageState::new(&self.controllers)))
            }
            Route::ClosingPreparationExecution => {
                Ok(Box::new(javelin_adapter::ClosingPreparationExecutionPageState::new()))
            }
            Route::ClosingLock => Ok(Box::new(javelin_adapter::ClosingLockPageState::new())),
            Route::TrialBalance => Ok(Box::new(javelin_adapter::TrialBalancePageState::new())),
            Route::AccountAdjustment => {
                Ok(Box::new(javelin_adapter::AccountAdjustmentPageState::new(&self.controllers)))
            }
            Route::AccountAdjustmentExecution => {
                Ok(Box::new(javelin_adapter::AccountAdjustmentExecutionPageState::new()))
            }
            Route::NoteDraft => Ok(Box::new(javelin_adapter::NoteDraftPageState::new())),
            Route::IfrsValuation => {
                Ok(Box::new(javelin_adapter::IfrsValuationPageState::new(&self.controllers)))
            }
            Route::IfrsValuationExecution => {
                Ok(Box::new(javelin_adapter::IfrsValuationExecutionPageState::new()))
            }
            Route::FinancialStatement => {
                Ok(Box::new(javelin_adapter::FinancialStatementPageState::new(&self.controllers)))
            }
            Route::FinancialStatementExecution => {
                Ok(Box::new(javelin_adapter::FinancialStatementExecutionPageState::new()))
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
