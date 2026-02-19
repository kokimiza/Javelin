// PageNavigator - ページ遷移とイベントループ管理
// 責務: ページ間のナビゲーションとイベント処理

use std::{sync::Arc, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use javelin_application::{
    dtos::{
        AdjustAccountsRequest, ApplyIfrsValuationRequest, ConsolidateLedgerRequest,
        GenerateFinancialStatementsRequest, GenerateNoteDraftRequest, GenerateTrialBalanceRequest,
        LockClosingPeriodRequest, PrepareClosingRequest, PrepareClosingResponse,
    },
    interactor::{
        AdjustAccountsInteractor, ApplyIfrsValuationInteractor, ApproveJournalEntryInteractor,
        CancelJournalEntryInteractor, ConsolidateLedgerInteractor, CorrectJournalEntryInteractor,
        CreateAdditionalEntryInteractor, CreateReclassificationEntryInteractor,
        CreateReplacementEntryInteractor, CreateReversalEntryInteractor,
        DeleteDraftJournalEntryInteractor, GenerateFinancialStatementsInteractor,
        GenerateNoteDraftInteractor, GenerateTrialBalanceInteractor, LoadAccountMasterInteractor,
        LockClosingPeriodInteractor, PrepareClosingInteractor, RegisterJournalEntryInteractor,
        RejectJournalEntryInteractor, ReverseJournalEntryInteractor, SubmitForApprovalInteractor,
        UpdateDraftJournalEntryInteractor,
    },
};
use javelin_infrastructure::{
    event_store::EventStore, journal_entry_finder_impl::JournalEntryFinderImpl,
    ledger_query_service_impl::LedgerQueryServiceImpl, queries::MasterDataLoaderImpl,
};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    controller::{AccountMasterController, ClosingController, JournalEntryController},
    error::{AdapterError, AdapterResult},
    pages::ClosingPage,
    presenter::{AccountMasterPresenter, AccountMasterViewModel, JournalEntryPresenter, Presenter},
    view_router::ViewType,
    views::pages::{
        AccountAdjustmentPage, ClosingLockPage, ClosingPreparationPage, FinancialStatementPage,
        IfrsValuationPage, JournalEntryFormPage, LedgerPage, LedgerViewPage, NoteDraftPage,
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

pub struct PageNavigator;

impl PageNavigator {
    /// 選択された画面に遷移
    #[allow(clippy::too_many_arguments)]
    pub fn navigate_to_view(
        terminal: &mut DefaultTerminal,
        view_type: ViewType,
        account_master_controller: &Arc<AccountMasterControllerType>,
        closing_controller: &Arc<ClosingControllerType>,
        journal_entry_controller: &Arc<JournalEntryControllerType>,
        account_master_receiver: tokio::sync::mpsc::UnboundedReceiver<AccountMasterViewModel>,
        journal_entry_result_receiver: tokio::sync::mpsc::UnboundedReceiver<
            crate::presenter::JournalEntryViewModel,
        >,
        journal_entry_progress_receiver: tokio::sync::mpsc::UnboundedReceiver<String>,
    ) -> (
        AdapterResult<()>,
        tokio::sync::mpsc::UnboundedReceiver<AccountMasterViewModel>,
        tokio::sync::mpsc::UnboundedReceiver<crate::presenter::JournalEntryViewModel>,
        tokio::sync::mpsc::UnboundedReceiver<String>,
    ) {
        match view_type {
            ViewType::JournalEntry => {
                // 101 原始記録登録 - 仕訳入力フォーム
                let mut page = JournalEntryFormPage::new();
                page.set_account_master_receiver(account_master_receiver);
                page.set_result_receiver(journal_entry_result_receiver);
                page.set_progress_receiver(journal_entry_progress_receiver);
                let result = Self::run_form_page_loop(
                    terminal,
                    &mut page,
                    account_master_controller,
                    journal_entry_controller,
                );
                let receiver = page.take_account_master_receiver().unwrap();
                let result_receiver = page.take_result_receiver().unwrap();
                let progress_receiver = page.take_progress_receiver().unwrap();
                (result, receiver, result_receiver, progress_receiver)
            }
            ViewType::Ledger => {
                // 401 元帳閲覧 - 元帳詳細
                let mut page = LedgerViewPage::default();
                let result = Self::run_selection_page_loop(terminal, &mut page);
                (
                    result,
                    account_master_receiver,
                    journal_entry_result_receiver,
                    journal_entry_progress_receiver,
                )
            }
            ViewType::LedgerConsolidation => {
                // 201 元帳集約 - 元帳一覧
                let mut page = LedgerPage::default();
                let result = Self::run_closing_page_loop(
                    terminal,
                    &mut page,
                    closing_controller,
                    |controller| async move {
                        controller
                            .consolidate_ledger(ConsolidateLedgerRequest {
                                fiscal_year: 2024,
                                period: 12,
                                from_date: "2024-12-01".to_string(),
                                to_date: "2024-12-31".to_string(),
                            })
                            .await
                    },
                );
                (
                    result,
                    account_master_receiver,
                    journal_entry_result_receiver,
                    journal_entry_progress_receiver,
                )
            }
            ViewType::TrialBalance => {
                // 303 試算表生成 - 決算画面（試算表表示）
                let mut page = ClosingPage::default();
                let result = Self::run_closing_page_loop(
                    terminal,
                    &mut page,
                    closing_controller,
                    |controller| async move {
                        controller
                            .generate_trial_balance(GenerateTrialBalanceRequest {
                                fiscal_year: 2024,
                                period: 12,
                            })
                            .await
                    },
                );
                (
                    result,
                    account_master_receiver,
                    journal_entry_result_receiver,
                    journal_entry_progress_receiver,
                )
            }
            ViewType::ClosingPreparation => {
                // 301 締準備 - 期間帰属確認・仮仕訳作成
                let mut page = ClosingPreparationPage::default();

                // レスポンス受信用チャネル
                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

                // 画面開始時にデータをロード
                let controller = Arc::clone(closing_controller);
                tokio::spawn(async move {
                    match controller
                        .prepare_closing(PrepareClosingRequest { fiscal_year: 2024, period: 12 })
                        .await
                    {
                        Ok(response) => {
                            let _ = tx.send(Ok(response));
                        }
                        Err(e) => {
                            let _ = tx.send(Err(e));
                        }
                    }
                });

                let result = Self::run_closing_preparation_loop(terminal, &mut page, &mut rx);
                (
                    result,
                    account_master_receiver,
                    journal_entry_result_receiver,
                    journal_entry_progress_receiver,
                )
            }
            ViewType::ClosingLock => {
                // 302 締日固定 - 取引データのロック処理
                let mut page = ClosingLockPage::default();

                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                let controller = Arc::clone(closing_controller);
                tokio::spawn(async move {
                    match controller
                        .lock_closing_period(LockClosingPeriodRequest {
                            fiscal_year: 2024,
                            period: 12,
                            locked_by: "system".to_string(),
                        })
                        .await
                    {
                        Ok(response) => {
                            let _ = tx.send(Ok(response));
                        }
                        Err(e) => {
                            let _ = tx.send(Err(e));
                        }
                    }
                });

                let result = Self::run_closing_lock_loop(terminal, &mut page, &mut rx);
                (
                    result,
                    account_master_receiver,
                    journal_entry_result_receiver,
                    journal_entry_progress_receiver,
                )
            }
            ViewType::NoteDraft => {
                // 304 注記草案生成 - 開示情報の整理
                let mut page = NoteDraftPage::default();

                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                let controller = Arc::clone(closing_controller);
                tokio::spawn(async move {
                    match controller
                        .generate_note_draft(GenerateNoteDraftRequest {
                            fiscal_year: 2024,
                            period: 12,
                        })
                        .await
                    {
                        Ok(response) => {
                            let _ = tx.send(Ok(response));
                        }
                        Err(e) => {
                            let _ = tx.send(Err(e));
                        }
                    }
                });

                let result = Self::run_note_draft_loop(terminal, &mut page, &mut rx);
                (
                    result,
                    account_master_receiver,
                    journal_entry_result_receiver,
                    journal_entry_progress_receiver,
                )
            }
            ViewType::AccountAdjustment => {
                // 305 勘定補正 - 仮勘定整理・区分修正
                let mut page = AccountAdjustmentPage::default();

                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                let controller = Arc::clone(closing_controller);
                tokio::spawn(async move {
                    match controller
                        .adjust_accounts(AdjustAccountsRequest { fiscal_year: 2024, period: 12 })
                        .await
                    {
                        Ok(response) => {
                            let _ = tx.send(Ok(response));
                        }
                        Err(e) => {
                            let _ = tx.send(Err(e));
                        }
                    }
                });

                let result = Self::run_account_adjustment_loop(terminal, &mut page, &mut rx);
                (
                    result,
                    account_master_receiver,
                    journal_entry_result_receiver,
                    journal_entry_progress_receiver,
                )
            }
            ViewType::IfrsValuation => {
                // 306 IFRS評価 - 見積会計・公正価値測定
                let mut page = IfrsValuationPage::default();

                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                let controller = Arc::clone(closing_controller);
                tokio::spawn(async move {
                    match controller
                        .apply_ifrs_valuation(ApplyIfrsValuationRequest {
                            fiscal_year: 2024,
                            period: 12,
                        })
                        .await
                    {
                        Ok(response) => {
                            let _ = tx.send(Ok(response));
                        }
                        Err(e) => {
                            let _ = tx.send(Err(e));
                        }
                    }
                });

                let result = Self::run_ifrs_valuation_loop(terminal, &mut page, &mut rx);
                (
                    result,
                    account_master_receiver,
                    journal_entry_result_receiver,
                    journal_entry_progress_receiver,
                )
            }
            ViewType::FinancialStatement => {
                // 307 財務諸表生成 - 制度開示資料作成
                let mut page = FinancialStatementPage::default();

                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                let controller = Arc::clone(closing_controller);
                tokio::spawn(async move {
                    match controller
                        .generate_financial_statements(GenerateFinancialStatementsRequest {
                            fiscal_year: 2024,
                            period: 12,
                        })
                        .await
                    {
                        Ok(response) => {
                            let _ = tx.send(Ok(response));
                        }
                        Err(e) => {
                            let _ = tx.send(Err(e));
                        }
                    }
                });

                let result = Self::run_financial_statement_loop(terminal, &mut page, &mut rx);
                (
                    result,
                    account_master_receiver,
                    journal_entry_result_receiver,
                    journal_entry_progress_receiver,
                )
            }
            _ => {
                // 未実装の画面
                let result = Self::show_not_implemented_message(terminal, view_type);
                (
                    result,
                    account_master_receiver,
                    journal_entry_result_receiver,
                    journal_entry_progress_receiver,
                )
            }
        }
    }

    /// フォームページのイベントループ
    fn run_form_page_loop(
        terminal: &mut DefaultTerminal,
        page: &mut JournalEntryFormPage,
        account_master_controller: &Arc<AccountMasterControllerType>,
        journal_entry_controller: &Arc<JournalEntryControllerType>,
    ) -> AdapterResult<()> {
        loop {
            terminal
                .draw(|frame| {
                    page.render(frame);
                })
                .map_err(|e| AdapterError::RenderingFailed(e.to_string()))?;

            // ローディングアニメーション更新とデータポーリング
            page.tick_loading();
            page.poll_account_master_data();
            // 結果を先にポーリング（エラー時に進捗メッセージをクリアするため）
            page.poll_result_data();
            // その後、進捗メッセージをポーリング
            page.poll_progress_messages();

            // データロード要求をチェック
            if page.has_pending_account_load() {
                let controller = account_master_controller.clone();
                tokio::spawn(async move {
                    let _ = controller.load_account_master(None, true).await;
                });
                page.clear_pending_account_load();
            }

            // 100msタイムアウトでイベントをポーリング
            if event::poll(Duration::from_millis(100)).map_err(AdapterError::EventReadFailed)?
                && let Event::Key(key) = event::read().map_err(AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                let input_mode = page.input_mode();

                match input_mode {
                    crate::input_mode::InputMode::Normal => match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Char('h') => page.move_left(),
                        KeyCode::Char('j') => page.move_down(),
                        KeyCode::Char('k') => page.move_up(),
                        KeyCode::Char('l') => page.move_right(),
                        KeyCode::Char('i') => page.enter_modify_mode(),
                        KeyCode::Char('m') => page.switch_edit_mode_next(),
                        KeyCode::Char('M') => page.switch_edit_mode_previous(),
                        KeyCode::Tab => page.add_line(),
                        KeyCode::BackTab => page.remove_line(),
                        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            // 確定処理を実行 (Ctrl+s)
                            if !page.is_submitting() {
                                // バリデーション
                                match page.to_register_request("system_user".to_string()) {
                                    Ok(request) => {
                                        page.start_submit();
                                        // コントローラ経由で登録処理を呼び出す（非同期）
                                        let controller = journal_entry_controller.clone();
                                        tokio::spawn(async move {
                                            if let Err(e) =
                                                controller.register_journal_entry(request).await
                                            {
                                                eprintln!("仕訳登録エラー: {}", e);
                                            }
                                        });
                                    }
                                    Err(e) => {
                                        page.set_submit_failed(e);
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    crate::input_mode::InputMode::Modify => {
                        let is_overlay_visible = page.is_overlay_visible();

                        if is_overlay_visible {
                            match key.code {
                                KeyCode::Char('k') | KeyCode::Up => page.overlay_select_previous(),
                                KeyCode::Char('j') | KeyCode::Down => page.overlay_select_next(),
                                KeyCode::Enter => page.overlay_confirm_selection(),
                                KeyCode::Esc => page.overlay_cancel(),
                                _ => {}
                            }
                        } else {
                            match key.code {
                                KeyCode::Char(ch) => page.input_char(ch),
                                KeyCode::Backspace => page.backspace(),
                                KeyCode::Esc => page.cancel_modify_mode(),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// 決算準備ページのイベントループ
    fn run_closing_preparation_loop(
        terminal: &mut DefaultTerminal,
        page: &mut ClosingPreparationPage,
        response_rx: &mut tokio::sync::mpsc::UnboundedReceiver<
            crate::error::AdapterResult<PrepareClosingResponse>,
        >,
    ) -> AdapterResult<()> {
        loop {
            terminal
                .draw(|frame| {
                    page.render(frame);
                })
                .map_err(|e| AdapterError::RenderingFailed(e.to_string()))?;

            page.tick();

            if let Ok(result) = response_rx.try_recv() {
                match result {
                    Ok(response) => {
                        page.set_response(response);
                    }
                    Err(e) => {
                        page.set_error(format!("{}", e));
                    }
                }
            }

            if event::poll(Duration::from_millis(100)).map_err(AdapterError::EventReadFailed)?
                && let Event::Key(key) = event::read().map_err(AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('k') => page.select_previous(),
                    KeyCode::Down | KeyCode::Char('j') => page.select_next(),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// 締日固定ページのイベントループ
    fn run_closing_lock_loop(
        terminal: &mut DefaultTerminal,
        page: &mut ClosingLockPage,
        response_rx: &mut tokio::sync::mpsc::UnboundedReceiver<
            crate::error::AdapterResult<javelin_application::dtos::LockClosingPeriodResponse>,
        >,
    ) -> AdapterResult<()> {
        loop {
            terminal
                .draw(|frame| {
                    page.render(frame);
                })
                .map_err(|e| AdapterError::RenderingFailed(e.to_string()))?;

            page.tick();

            if let Ok(result) = response_rx.try_recv() {
                match result {
                    Ok(response) => {
                        page.set_response(response);
                    }
                    Err(e) => {
                        page.set_error(format!("{}", e));
                    }
                }
            }

            if event::poll(Duration::from_millis(100)).map_err(AdapterError::EventReadFailed)?
                && let Event::Key(key) = event::read().map_err(AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('k') => page.select_previous(),
                    KeyCode::Down | KeyCode::Char('j') => page.select_next(),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// 注記草案ページのイベントループ
    fn run_note_draft_loop(
        terminal: &mut DefaultTerminal,
        page: &mut NoteDraftPage,
        response_rx: &mut tokio::sync::mpsc::UnboundedReceiver<
            crate::error::AdapterResult<javelin_application::dtos::GenerateNoteDraftResponse>,
        >,
    ) -> AdapterResult<()> {
        loop {
            terminal
                .draw(|frame| {
                    page.render(frame);
                })
                .map_err(|e| AdapterError::RenderingFailed(e.to_string()))?;

            page.tick();

            if let Ok(result) = response_rx.try_recv() {
                match result {
                    Ok(response) => {
                        page.set_response(response);
                    }
                    Err(e) => {
                        page.set_error(format!("{}", e));
                    }
                }
            }

            if event::poll(Duration::from_millis(100)).map_err(AdapterError::EventReadFailed)?
                && let Event::Key(key) = event::read().map_err(AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('k') => page.select_previous(),
                    KeyCode::Down | KeyCode::Char('j') => page.select_next(),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// 勘定補正ページのイベントループ
    fn run_account_adjustment_loop(
        terminal: &mut DefaultTerminal,
        page: &mut AccountAdjustmentPage,
        response_rx: &mut tokio::sync::mpsc::UnboundedReceiver<
            crate::error::AdapterResult<javelin_application::dtos::AdjustAccountsResponse>,
        >,
    ) -> AdapterResult<()> {
        loop {
            terminal
                .draw(|frame| {
                    page.render(frame);
                })
                .map_err(|e| AdapterError::RenderingFailed(e.to_string()))?;

            page.tick();

            if let Ok(result) = response_rx.try_recv() {
                match result {
                    Ok(response) => {
                        page.set_response(response);
                    }
                    Err(e) => {
                        page.set_error(format!("{}", e));
                    }
                }
            }

            if event::poll(Duration::from_millis(100)).map_err(AdapterError::EventReadFailed)?
                && let Event::Key(key) = event::read().map_err(AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('k') => page.select_previous(),
                    KeyCode::Down | KeyCode::Char('j') => page.select_next(),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// IFRS評価ページのイベントループ
    fn run_ifrs_valuation_loop(
        terminal: &mut DefaultTerminal,
        page: &mut IfrsValuationPage,
        response_rx: &mut tokio::sync::mpsc::UnboundedReceiver<
            crate::error::AdapterResult<javelin_application::dtos::ApplyIfrsValuationResponse>,
        >,
    ) -> AdapterResult<()> {
        loop {
            terminal
                .draw(|frame| {
                    page.render(frame);
                })
                .map_err(|e| AdapterError::RenderingFailed(e.to_string()))?;

            page.tick();

            if let Ok(result) = response_rx.try_recv() {
                match result {
                    Ok(response) => {
                        page.set_response(response);
                    }
                    Err(e) => {
                        page.set_error(format!("{}", e));
                    }
                }
            }

            if event::poll(Duration::from_millis(100)).map_err(AdapterError::EventReadFailed)?
                && let Event::Key(key) = event::read().map_err(AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('k') => page.select_previous(),
                    KeyCode::Down | KeyCode::Char('j') => page.select_next(),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// 財務諸表生成ページのイベントループ
    fn run_financial_statement_loop(
        terminal: &mut DefaultTerminal,
        page: &mut FinancialStatementPage,
        response_rx: &mut tokio::sync::mpsc::UnboundedReceiver<
            crate::error::AdapterResult<
                javelin_application::dtos::GenerateFinancialStatementsResponse,
            >,
        >,
    ) -> AdapterResult<()> {
        loop {
            terminal
                .draw(|frame| {
                    page.render(frame);
                })
                .map_err(|e| AdapterError::RenderingFailed(e.to_string()))?;

            page.tick();

            if let Ok(result) = response_rx.try_recv() {
                match result {
                    Ok(response) => {
                        page.set_response(response);
                    }
                    Err(e) => {
                        page.set_error(format!("{}", e));
                    }
                }
            }

            if event::poll(Duration::from_millis(100)).map_err(AdapterError::EventReadFailed)?
                && let Event::Key(key) = event::read().map_err(AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('k') => page.select_previous(),
                    KeyCode::Down | KeyCode::Char('j') => page.select_next(),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// 決算ページのイベントループ（ENTERキーでユースケース実行）
    fn run_closing_page_loop<P, F, Fut, R>(
        terminal: &mut DefaultTerminal,
        page: &mut P,
        closing_controller: &Arc<ClosingControllerType>,
        execute_fn: F,
    ) -> AdapterResult<()>
    where
        P: SelectionPage,
        F: Fn(Arc<ClosingControllerType>) -> Fut,
        Fut: std::future::Future<Output = crate::error::AdapterResult<R>>,
        R: std::fmt::Debug,
    {
        loop {
            terminal
                .draw(|frame| {
                    page.render(frame);
                })
                .map_err(|e| AdapterError::RenderingFailed(e.to_string()))?;

            // 100msタイムアウトでイベントをポーリング
            if event::poll(Duration::from_millis(100)).map_err(AdapterError::EventReadFailed)?
                && let Event::Key(key) = event::read().map_err(AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('k') => page.select_previous(),
                    KeyCode::Down | KeyCode::Char('j') => page.select_next(),
                    KeyCode::Enter => {
                        // ユースケース実行
                        let controller = Arc::clone(closing_controller);
                        let result = tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(execute_fn(controller));
                        match result {
                            Ok(_) => {
                                // 成功メッセージは画面に表示済み
                            }
                            Err(e) => {
                                eprintln!("Error: {:?}", e);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// 選択ページのイベントループ
    fn run_selection_page_loop<P>(terminal: &mut DefaultTerminal, page: &mut P) -> AdapterResult<()>
    where
        P: SelectionPage,
    {
        loop {
            terminal
                .draw(|frame| {
                    page.render(frame);
                })
                .map_err(|e| AdapterError::RenderingFailed(e.to_string()))?;

            if let Event::Key(key) = event::read().map_err(AdapterError::EventReadFailed)? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('k') => page.select_previous(),
                    KeyCode::Down | KeyCode::Char('j') => page.select_next(),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// 未実装メッセージを表示
    fn show_not_implemented_message(
        terminal: &mut DefaultTerminal,
        view_type: ViewType,
    ) -> AdapterResult<()> {
        use ratatui::{
            layout::Alignment,
            style::{Color, Modifier, Style},
            text::{Line, Span},
            widgets::{Block, Borders, Paragraph},
        };

        loop {
            terminal
                .draw(|frame| {
                    let area = frame.area();
                    let message = Paragraph::new(vec![
                        Line::from(""),
                        Line::from(Span::styled(
                            "この機能は未実装です",
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                        )),
                        Line::from(""),
                        Line::from(Span::styled(
                            format!("画面: {:?}", view_type),
                            Style::default().fg(Color::Gray),
                        )),
                        Line::from(""),
                        Line::from(Span::styled("[Esc] 戻る", Style::default().fg(Color::Cyan))),
                    ])
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .title("未実装")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Yellow)),
                    );

                    frame.render_widget(message, area);
                })
                .map_err(|e| AdapterError::RenderingFailed(e.to_string()))?;

            if let Event::Key(key) = event::read().map_err(AdapterError::EventReadFailed)? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if matches!(key.code, KeyCode::Esc) {
                    break;
                }
            }
        }

        Ok(())
    }
}

/// 選択機能を持つページのトレイト
pub trait SelectionPage {
    fn render(&mut self, frame: &mut Frame);
    fn select_next(&mut self);
    fn select_previous(&mut self);
}

// 各ページの実装
impl SelectionPage for LedgerViewPage {
    fn render(&mut self, frame: &mut Frame) {
        LedgerViewPage::render(self, frame);
    }

    fn select_next(&mut self) {
        LedgerViewPage::select_next(self);
    }

    fn select_previous(&mut self) {
        LedgerViewPage::select_previous(self);
    }
}

impl SelectionPage for LedgerPage {
    fn render(&mut self, frame: &mut Frame) {
        LedgerPage::render(self, frame);
    }

    fn select_next(&mut self) {
        LedgerPage::select_next(self);
    }

    fn select_previous(&mut self) {
        LedgerPage::select_previous(self);
    }
}

impl SelectionPage for ClosingPage {
    fn render(&mut self, frame: &mut Frame) {
        ClosingPage::render(self, frame);
    }

    fn select_next(&mut self) {
        ClosingPage::select_next(self);
    }

    fn select_previous(&mut self) {
        ClosingPage::select_previous(self);
    }
}

impl SelectionPage for ClosingPreparationPage {
    fn render(&mut self, frame: &mut Frame) {
        ClosingPreparationPage::render(self, frame);
    }

    fn select_next(&mut self) {
        ClosingPreparationPage::select_next(self);
    }

    fn select_previous(&mut self) {
        ClosingPreparationPage::select_previous(self);
    }
}

impl SelectionPage for ClosingLockPage {
    fn render(&mut self, frame: &mut Frame) {
        ClosingLockPage::render(self, frame);
    }

    fn select_next(&mut self) {
        ClosingLockPage::select_next(self);
    }

    fn select_previous(&mut self) {
        ClosingLockPage::select_previous(self);
    }
}

impl SelectionPage for NoteDraftPage {
    fn render(&mut self, frame: &mut Frame) {
        NoteDraftPage::render(self, frame);
    }

    fn select_next(&mut self) {
        NoteDraftPage::select_next(self);
    }

    fn select_previous(&mut self) {
        NoteDraftPage::select_previous(self);
    }
}

impl SelectionPage for AccountAdjustmentPage {
    fn render(&mut self, frame: &mut Frame) {
        AccountAdjustmentPage::render(self, frame);
    }

    fn select_next(&mut self) {
        AccountAdjustmentPage::select_next(self);
    }

    fn select_previous(&mut self) {
        AccountAdjustmentPage::select_previous(self);
    }
}

impl SelectionPage for IfrsValuationPage {
    fn render(&mut self, frame: &mut Frame) {
        IfrsValuationPage::render(self, frame);
    }

    fn select_next(&mut self) {
        IfrsValuationPage::select_next(self);
    }

    fn select_previous(&mut self) {
        IfrsValuationPage::select_previous(self);
    }
}

impl SelectionPage for FinancialStatementPage {
    fn render(&mut self, frame: &mut Frame) {
        FinancialStatementPage::render(self, frame);
    }

    fn select_next(&mut self) {
        FinancialStatementPage::select_next(self);
    }

    fn select_previous(&mut self) {
        FinancialStatementPage::select_previous(self);
    }
}
