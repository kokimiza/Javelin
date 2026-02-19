// HomePage - ホーム画面（業務メニュー + システムマスタメニュー）
// 責務: 業務メニューとシステムマスタメニューの表示、h/lで枠切り替え、j/kで内部フォーカス移動

use std::sync::Arc;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use javelin_application::interactor::{
    AdjustAccountsInteractor, ApplyIfrsValuationInteractor, ApproveJournalEntryInteractor,
    CancelJournalEntryInteractor, ConsolidateLedgerInteractor, CorrectJournalEntryInteractor,
    CreateAdditionalEntryInteractor, CreateReclassificationEntryInteractor,
    CreateReplacementEntryInteractor, CreateReversalEntryInteractor,
    DeleteDraftJournalEntryInteractor, GenerateFinancialStatementsInteractor,
    GenerateNoteDraftInteractor, GenerateTrialBalanceInteractor, LoadAccountMasterInteractor,
    LockClosingPeriodInteractor, PrepareClosingInteractor, RegisterJournalEntryInteractor,
    RejectJournalEntryInteractor, ReverseJournalEntryInteractor, SubmitForApprovalInteractor,
    UpdateDraftJournalEntryInteractor,
};
use javelin_infrastructure::{
    event_store::EventStore, journal_entry_finder_impl::JournalEntryFinderImpl,
    ledger_query_service_impl::LedgerQueryServiceImpl, queries::MasterDataLoaderImpl,
};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    controller::{AccountMasterController, ClosingController, JournalEntryController},
    error::AdapterResult,
    presenter::{AccountMasterPresenter, AccountMasterViewModel, JournalEntryPresenter, Presenter},
    view_router::ViewType,
    views::{
        components::{ListItemData, ListSelector},
        layouts::MenuLayout,
        page_navigator::PageNavigator,
        terminal_manager::TerminalManager,
    },
};

type AccountMasterControllerType = AccountMasterController<
    LoadAccountMasterInteractor<MasterDataLoaderImpl, AccountMasterPresenter>,
>;

type ClosingControllerType = ClosingController<
    ConsolidateLedgerInteractor<LedgerQueryServiceImpl>,
    PrepareClosingInteractor<LedgerQueryServiceImpl>,
    LockClosingPeriodInteractor<EventStore>,
    GenerateTrialBalanceInteractor<LedgerQueryServiceImpl>,
    GenerateNoteDraftInteractor<LedgerQueryServiceImpl>,
    AdjustAccountsInteractor<EventStore, LedgerQueryServiceImpl>,
    ApplyIfrsValuationInteractor<EventStore, LedgerQueryServiceImpl>,
    GenerateFinancialStatementsInteractor<LedgerQueryServiceImpl>,
>;

type JournalEntryControllerType = JournalEntryController<
    RegisterJournalEntryInteractor<
        EventStore,
        Presenter,
        JournalEntryPresenter,
        javelin_infrastructure::services::VoucherNumberGeneratorImpl,
    >,
    UpdateDraftJournalEntryInteractor<EventStore, Presenter, JournalEntryPresenter>,
    SubmitForApprovalInteractor<EventStore, Presenter, JournalEntryPresenter>,
    ApproveJournalEntryInteractor<EventStore, Presenter, JournalEntryPresenter>,
    RejectJournalEntryInteractor<EventStore, Presenter, JournalEntryPresenter>,
    ReverseJournalEntryInteractor<EventStore, Presenter, JournalEntryPresenter>,
    CorrectJournalEntryInteractor<EventStore, Presenter, JournalEntryPresenter>,
    DeleteDraftJournalEntryInteractor<EventStore, Presenter, JournalEntryPresenter>,
    CancelJournalEntryInteractor<
        EventStore,
        Presenter,
        JournalEntryPresenter,
        JournalEntryFinderImpl<JournalEntryPresenter>,
    >,
    CreateReversalEntryInteractor<
        EventStore,
        Presenter,
        JournalEntryPresenter,
        JournalEntryFinderImpl<JournalEntryPresenter>,
    >,
    CreateAdditionalEntryInteractor<
        EventStore,
        Presenter,
        JournalEntryPresenter,
        JournalEntryFinderImpl<JournalEntryPresenter>,
    >,
    CreateReclassificationEntryInteractor<
        EventStore,
        Presenter,
        JournalEntryPresenter,
        JournalEntryFinderImpl<JournalEntryPresenter>,
    >,
    CreateReplacementEntryInteractor<
        EventStore,
        Presenter,
        JournalEntryPresenter,
        JournalEntryFinderImpl<JournalEntryPresenter>,
    >,
>;

/// メニュータイプ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuType {
    Business, // 業務メニュー
    System,   // システムマスタメニュー
}

pub struct HomePage {
    layout: MenuLayout,
    business_menu_selector: ListSelector,
    system_menu_selector: ListSelector,
    active_menu: MenuType,
}

impl HomePage {
    pub fn new() -> Self {
        let mut layout = MenuLayout::new("財務会計システム JAVELIN", "主計部", "担当者");
        layout.event_viewer_mut().add_info("システム起動完了");
        layout.event_viewer_mut().add_info("バージョン: 1.0.0");

        let business_menu_items = vec![
            ListItemData::new("101", "原始記録登録", "日次：仕訳帳・キャッシュログ入力"),
            ListItemData::new("201", "元帳集約", "週次：総勘定元帳への転記処理"),
            ListItemData::new("301", "締準備", "月次：期間帰属確認・仮仕訳作成"),
            ListItemData::new("302", "締日固定", "月次：取引データのロック処理"),
            ListItemData::new("303", "試算表生成", "月次：残高検証・異常値抽出"),
            ListItemData::new("304", "注記草案生成", "月次：開示情報の整理"),
            ListItemData::new("305", "勘定補正", "月次：仮勘定整理・区分修正"),
            ListItemData::new("306", "IFRS評価", "月次：見積会計・公正価値測定"),
            ListItemData::new("307", "財務諸表生成", "月次：制度開示資料作成"),
            ListItemData::new("401", "元帳閲覧", "照会：総勘定元帳・補助元帳"),
        ];

        let system_menu_items = vec![
            ListItemData::new("901", "勘定科目マスタ", "勘定科目の登録・編集・削除"),
            ListItemData::new("902", "会社マスタ", "会社情報の登録・編集・削除"),
            ListItemData::new("903", "ユーザ設定", "言語・表示形式・デフォルト値設定"),
            ListItemData::new("904", "システム設定", "会計年度・締日・バックアップ設定"),
            ListItemData::new("905", "データバックアップ", "マスタデータのバックアップ・復元"),
            ListItemData::new("906", "データインポート", "外部データの一括取込"),
            ListItemData::new("907", "データエクスポート", "マスタデータの出力"),
        ];

        let business_menu_selector = ListSelector::new("業務メニュー", business_menu_items);
        let system_menu_selector = ListSelector::new("システムマスタ", system_menu_items);

        Self {
            layout,
            business_menu_selector,
            system_menu_selector,
            active_menu: MenuType::Business,
        }
    }

    /// メニュー枠を切り替え（h/l）
    fn switch_menu(&mut self) {
        self.active_menu = match self.active_menu {
            MenuType::Business => {
                self.layout.event_viewer_mut().add_info("システムマスタメニューに切替");
                MenuType::System
            }
            MenuType::System => {
                self.layout.event_viewer_mut().add_info("業務メニューに切替");
                MenuType::Business
            }
        };
    }

    /// 選択を上に移動
    pub fn select_previous(&mut self) {
        match self.active_menu {
            MenuType::Business => self.business_menu_selector.select_previous(),
            MenuType::System => self.system_menu_selector.select_previous(),
        }
    }

    /// 選択を下に移動
    pub fn select_next(&mut self) {
        match self.active_menu {
            MenuType::Business => self.business_menu_selector.select_next(),
            MenuType::System => self.system_menu_selector.select_next(),
        }
    }

    /// 選択された項目に対応するビューを取得
    pub fn get_selected_view(&self) -> Option<ViewType> {
        match self.active_menu {
            MenuType::Business => {
                self.business_menu_selector.selected_index().and_then(|idx| match idx {
                    0 => Some(ViewType::JournalEntry),
                    1 => Some(ViewType::LedgerConsolidation),
                    2 => Some(ViewType::ClosingPreparation),
                    3 => Some(ViewType::ClosingLock),
                    4 => Some(ViewType::TrialBalance),
                    5 => Some(ViewType::NoteDraft),
                    6 => Some(ViewType::AccountAdjustment),
                    7 => Some(ViewType::IfrsValuation),
                    8 => Some(ViewType::FinancialStatement),
                    9 => Some(ViewType::Ledger),
                    _ => None,
                })
            }
            MenuType::System => {
                self.system_menu_selector.selected_index().and_then(|idx| match idx {
                    0 => Some(ViewType::AccountMasterManagement),
                    1 => Some(ViewType::CompanyMasterManagement),
                    2 => Some(ViewType::UserSettingsManagement),
                    3 => Some(ViewType::SystemSettingsManagement),
                    4 => Some(ViewType::DataBackup),
                    5 => Some(ViewType::DataImport),
                    6 => Some(ViewType::DataExport),
                    _ => None,
                })
            }
        }
    }

    /// アプリケーションを実行（コントローラ付き）
    pub fn run_with_controller(
        &mut self,
        account_master_controller: &Arc<AccountMasterControllerType>,
        closing_controller: &Arc<ClosingControllerType>,
        journal_entry_controller: &Arc<JournalEntryControllerType>,
        account_master_receiver: &mut tokio::sync::mpsc::UnboundedReceiver<AccountMasterViewModel>,
        journal_entry_result_receiver: &mut tokio::sync::mpsc::UnboundedReceiver<
            crate::presenter::JournalEntryViewModel,
        >,
        journal_entry_progress_receiver: &mut tokio::sync::mpsc::UnboundedReceiver<String>,
    ) -> AdapterResult<()> {
        let mut terminal_manager = TerminalManager::new()?;
        let terminal = terminal_manager.terminal_mut();

        self.run_main_loop(
            terminal,
            account_master_controller,
            closing_controller,
            journal_entry_controller,
            account_master_receiver,
            journal_entry_result_receiver,
            journal_entry_progress_receiver,
        )
    }

    /// メインループ
    #[allow(clippy::too_many_arguments)]
    fn run_main_loop(
        &mut self,
        terminal: &mut DefaultTerminal,
        account_master_controller: &Arc<AccountMasterControllerType>,
        closing_controller: &Arc<ClosingControllerType>,
        journal_entry_controller: &Arc<JournalEntryControllerType>,
        account_master_receiver: &mut tokio::sync::mpsc::UnboundedReceiver<AccountMasterViewModel>,
        journal_entry_result_receiver: &mut tokio::sync::mpsc::UnboundedReceiver<
            crate::presenter::JournalEntryViewModel,
        >,
        journal_entry_progress_receiver: &mut tokio::sync::mpsc::UnboundedReceiver<String>,
    ) -> AdapterResult<()> {
        loop {
            // 描画
            terminal
                .draw(|frame| {
                    self.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            // イベント処理
            if let Event::Key(key) =
                event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => self.switch_menu(),
                    KeyCode::Char('l') => self.switch_menu(),
                    KeyCode::Char('k') => self.select_previous(),
                    KeyCode::Char('j') => self.select_next(),
                    KeyCode::Enter => {
                        if let Some(view_type) = self.get_selected_view() {
                            self.layout
                                .event_viewer_mut()
                                .add_info(format!("画面遷移: {:?}", view_type));

                            // 画面遷移を実行
                            let receiver = std::mem::replace(
                                account_master_receiver,
                                tokio::sync::mpsc::unbounded_channel().1,
                            );
                            let result_receiver = std::mem::replace(
                                journal_entry_result_receiver,
                                tokio::sync::mpsc::unbounded_channel().1,
                            );
                            let progress_receiver = std::mem::replace(
                                journal_entry_progress_receiver,
                                tokio::sync::mpsc::unbounded_channel().1,
                            );
                            let (
                                result,
                                returned_receiver,
                                returned_result_receiver,
                                returned_progress_receiver,
                            ) = PageNavigator::navigate_to_view(
                                terminal,
                                view_type,
                                account_master_controller,
                                closing_controller,
                                journal_entry_controller,
                                receiver,
                                result_receiver,
                                progress_receiver,
                            );
                            *account_master_receiver = returned_receiver;
                            *journal_entry_result_receiver = returned_result_receiver;
                            *journal_entry_progress_receiver = returned_progress_receiver;

                            // エラーハンドリング
                            if let Err(e) = result {
                                self.layout.event_viewer_mut().add_error(format!("エラー: {}", e));
                            } else {
                                self.layout.event_viewer_mut().add_info("画面遷移完了");
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame) {
        use ratatui::layout::{Constraint, Direction, Layout};

        let active_menu = self.active_menu;
        let business_selector = &mut self.business_menu_selector;
        let system_selector = &mut self.system_menu_selector;

        self.layout.render(frame, |frame, area| {
            // メインエリアを上下分割: 業務メニュー(上) + システムマスタ(下)
            let menu_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(area);

            // 業務メニュー（枠なし、ListSelectorが自分で枠を描画）
            let is_business_active = active_menu == MenuType::Business;
            business_selector.set_active(is_business_active);
            business_selector.render(frame, menu_chunks[0]);

            // システムマスタメニュー（枠なし、ListSelectorが自分で枠を描画）
            let is_system_active = active_menu == MenuType::System;
            system_selector.set_active(is_system_active);
            system_selector.render(frame, menu_chunks[1]);
        });
    }
}

impl Default for HomePage {
    fn default() -> Self {
        Self::new()
    }
}
