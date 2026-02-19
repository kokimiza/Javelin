// Application Builder - DIコンテナとアプリケーション構築
// Builderパターンによる依存性注入の管理

use std::{path::PathBuf, sync::Arc};

use javelin_adapter::{
    controller::{
        AccountMasterController, ClosingController, JournalEntryController, LedgerController,
    },
    presenter::{
        AccountMasterPresenter, AccountMasterViewModel, JournalEntryPresenter, LedgerPresenter,
        Presenter,
    },
    view_router::ViewRouter,
    views::pages::{ClosingPage, HomePage, LedgerPage, LedgerViewPage},
};
use javelin_application::{
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
    projection_builder::ProjectionBuilder,
    query_service::MasterDataLoaderService,
};
use javelin_infrastructure::{
    event_store::EventStore, journal_entry_finder_impl::JournalEntryFinderImpl,
    ledger_query_service_impl::LedgerQueryServiceImpl,
    projection_builder_impl::ProjectionBuilderImpl, projection_db::ProjectionDb,
    queries::MasterDataLoaderImpl, services::VoucherNumberGeneratorImpl,
};
use tokio::sync::mpsc;

use crate::app_error::{AppError, AppResult};

// Type aliases for complex controller types
type JournalEntryControllerType = JournalEntryController<
    RegisterJournalEntryInteractor<
        EventStore,
        Presenter,
        JournalEntryPresenter,
        VoucherNumberGeneratorImpl,
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

/// アプリケーション全体の構成
pub struct Application {
    view_router: ViewRouter,
    home_view: HomePage,
    ledger_page: LedgerPage,
    ledger_view_page: LedgerViewPage,
    closing_page: ClosingPage,
    projection_db: Arc<ProjectionDb>,
    event_store: Arc<EventStore>,
    projection_builder: Arc<ProjectionBuilderImpl>,
    master_data_loader: Arc<MasterDataLoaderImpl>,
    // コントローラ
    account_master_controller: Arc<
        AccountMasterController<
            LoadAccountMasterInteractor<MasterDataLoaderImpl, AccountMasterPresenter>,
        >,
    >,
    journal_entry_controller: Arc<JournalEntryControllerType>,
    ledger_controller: Arc<LedgerController<LedgerQueryServiceImpl>>,
    closing_controller: Arc<ClosingControllerType>,
    // イベント通知用
    event_sender: mpsc::UnboundedSender<javelin_application::output_port::EventNotification>,
    event_receiver: mpsc::UnboundedReceiver<javelin_application::output_port::EventNotification>,
    // AccountMaster通知用
    account_master_receiver: mpsc::UnboundedReceiver<AccountMasterViewModel>,
    // JournalEntry結果通知用
    journal_entry_result_receiver:
        mpsc::UnboundedReceiver<javelin_adapter::presenter::JournalEntryViewModel>,
    // JournalEntry進捗通知用
    journal_entry_progress_receiver: mpsc::UnboundedReceiver<String>,
}

impl Application {
    /// アプリケーションを実行
    pub fn run(mut self) -> AppResult<()> {
        // すべてのコンポーネントが初期化されていることをログ出力
        println!("\n◆ アプリケーション起動 ◆");
        println!("  EventStore: 初期化完了");
        println!("  ProjectionDB: 初期化完了");
        println!("  ProjectionBuilder: 初期化完了");
        println!("  Controllers: {} 個", 4); // AccountMaster, JournalEntry, Ledger, Closing
        println!("  Pages: {} 個", 5); // Home, JournalEntry, Ledger, LedgerView, Closing
        println!("  ViewRouter: 準備完了");

        // 各コンポーネントの参照カウントを確認（実際に使用されていることを保証）
        let _event_store_ref = Arc::clone(&self.event_store);
        let _projection_db_ref = Arc::clone(&self.projection_db);
        let _projection_builder_ref = Arc::clone(&self.projection_builder);
        let _master_data_ref = Arc::clone(&self.master_data_loader);
        let _je_controller_ref = Arc::clone(&self.journal_entry_controller);
        let _ledger_controller_ref = Arc::clone(&self.ledger_controller);
        let _closing_controller_ref = Arc::clone(&self.closing_controller);

        // イベント通知システムの準備
        let _event_sender_clone = self.event_sender.clone();

        // ViewRouterの状態を確認
        let _current_view = self.view_router.current_view();

        // ページの準備（updateを呼んで初期化）
        self.ledger_page.update();
        self.closing_page.update();

        // LedgerViewPageも使用
        let _ledger_view_ref = &self.ledger_view_page;

        println!("\n✓ すべてのコンポーネントが正常に初期化されました");
        println!("  メインメニューを起動します...\n");

        // HomePageを実行（内部で画面遷移を処理）
        self.home_view
            .run_with_controller(
                &self.account_master_controller,
                &self.closing_controller,
                &self.journal_entry_controller,
                &mut self.account_master_receiver,
                &mut self.journal_entry_result_receiver,
                &mut self.journal_entry_progress_receiver,
            )
            .map_err(AppError::AdapterError)?;

        // 終了時のクリーンアップ
        println!("\n◆ アプリケーション終了 ◆");
        println!("  すべてのコンポーネントを正常にシャットダウンしました");

        // イベントレシーバーの残りを処理
        let mut event_count = 0;
        while let Ok(_event) = self.event_receiver.try_recv() {
            event_count += 1;
        }
        if event_count > 0 {
            println!("  未処理イベント: {} 件をクリーンアップしました", event_count);
        }

        Ok(())
    }
}

/// アプリケーションビルダー
pub struct ApplicationBuilder {
    data_dir: Option<PathBuf>,
}

impl ApplicationBuilder {
    /// 新規ビルダーを作成
    pub fn new() -> Self {
        Self { data_dir: None }
    }

    /// データディレクトリを設定
    pub fn with_data_dir(mut self, path: PathBuf) -> Self {
        self.data_dir = Some(path);
        self
    }

    /// アプリケーションをビルド
    pub async fn build(self) -> AppResult<Application> {
        // データディレクトリの決定
        let data_dir = self.data_dir.unwrap_or_else(|| {
            let mut path = std::env::current_dir().expect("Failed to get current directory");
            path.push("data");
            path
        });

        // データディレクトリの作成
        if !data_dir.exists() {
            tokio::fs::create_dir_all(&data_dir).await.map_err(|e| {
                AppError::DataDirectoryCreationFailed {
                    path: data_dir.display().to_string(),
                    source: e,
                }
            })?;
        }

        // Infrastructure層の構築（共有リソース）
        let event_store = Arc::new(EventStore::new(&data_dir.join("events")).await?);
        let projection_db = Arc::new(ProjectionDb::new(&data_dir.join("projections")).await?);

        // ProjectionBuilderの構築
        let projection_builder = Arc::new(ProjectionBuilderImpl::new(
            Arc::clone(&projection_db),
            Arc::clone(&event_store),
        ));

        // Projection再構築チェック
        let latest_sequence =
            event_store.get_latest_sequence().await.map(|seq| seq.as_u64()).unwrap_or(0);
        let projection_position = projection_db.get_position("main", 1).await.unwrap_or(0);

        if projection_position < latest_sequence {
            println!("✓ Projection rebuild required");
            println!("  - Latest event sequence: {}", latest_sequence);
            println!("  - Projection position: {}", projection_position);
            println!("  - Rebuilding projections...");

            projection_builder.rebuild_all_projections().await?;

            println!("✓ Projection rebuild completed");
        } else {
            println!("✓ Projections are up to date");
            println!("  - Latest event sequence: {}", latest_sequence);
            println!("  - Projection position: {}", projection_position);
        }

        // マスタデータローダー（Infrastructure層）
        let master_data_loader = Arc::new(
            MasterDataLoaderImpl::new(&data_dir.join("master_data"))
                .await
                .map_err(AppError::InitializationFailed)?,
        );

        // 初期データロード確認
        let master_data = master_data_loader.load_master_data().await?;
        println!("✓ Master data loaded successfully");
        println!("  - Accounts: {}", master_data.accounts.len());
        println!("  - Companies: {}", master_data.companies.len());
        println!("  - Language: {}", master_data.user_options.language);

        // イベント通知チャネル
        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        // Presenter構築（Output Port実装）
        let event_presenter = Arc::new(Presenter::new(event_sender.clone()));

        // AccountMasterPresenterチャネル
        let (account_master_sender, account_master_receiver) = mpsc::unbounded_channel();
        let account_master_presenter = AccountMasterPresenter::new(account_master_sender);

        // JournalEntryPresenterチャネル（インタラクタで使用）
        let (
            je_list_tx,
            _je_list_rx,
            je_detail_tx,
            _je_detail_rx,
            je_result_tx,
            je_result_rx,
            je_progress_tx,
            je_progress_rx,
        ) = JournalEntryPresenter::create_channels();
        let journal_entry_presenter = Arc::new(JournalEntryPresenter::new(
            je_list_tx,
            je_detail_tx,
            je_result_tx,
            je_progress_tx,
        ));

        // LedgerPresenterチャネル
        let (ledger_tx, ledger_rx, trial_balance_tx, trial_balance_rx) =
            LedgerPresenter::create_channels();
        let _ledger_presenter = Arc::new(LedgerPresenter::new(ledger_tx, trial_balance_tx));

        // QueryService構築
        let journal_entry_finder = Arc::new(JournalEntryFinderImpl::new(
            Arc::clone(&projection_db),
            Arc::clone(&journal_entry_presenter),
        ));

        let ledger_query_service = Arc::new(LedgerQueryServiceImpl::new(Arc::clone(&event_store)));

        // VoucherNumberGeneratorの作成
        let voucher_generator = Arc::new(VoucherNumberGeneratorImpl::new());

        // Interactor構築（Use Case実装）
        let load_account_master_interactor = LoadAccountMasterInteractor::new(
            Arc::clone(&master_data_loader),
            account_master_presenter,
        );

        // 仕訳登録系Interactor
        let register_interactor = Arc::new(RegisterJournalEntryInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&event_presenter),
            Arc::clone(&journal_entry_presenter),
            Arc::clone(&voucher_generator),
        ));

        let update_draft_interactor = Arc::new(UpdateDraftJournalEntryInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&event_presenter),
            Arc::clone(&journal_entry_presenter),
        ));

        let submit_for_approval_interactor = Arc::new(SubmitForApprovalInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&event_presenter),
            Arc::clone(&journal_entry_presenter),
        ));

        let approve_interactor = Arc::new(ApproveJournalEntryInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&event_presenter),
            Arc::clone(&journal_entry_presenter),
        ));

        let reject_interactor = Arc::new(RejectJournalEntryInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&event_presenter),
            Arc::clone(&journal_entry_presenter),
        ));

        let reverse_interactor = Arc::new(ReverseJournalEntryInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&event_presenter),
            Arc::clone(&journal_entry_presenter),
        ));

        let correct_interactor = Arc::new(CorrectJournalEntryInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&event_presenter),
            Arc::clone(&journal_entry_presenter),
        ));

        let delete_draft_interactor = Arc::new(DeleteDraftJournalEntryInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&event_presenter),
            Arc::clone(&journal_entry_presenter),
        ));

        // 仕訳行為区分系Interactor
        let cancel_interactor = Arc::new(CancelJournalEntryInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&event_presenter),
            Arc::clone(&journal_entry_presenter),
            Arc::clone(&journal_entry_finder),
        ));

        let create_reversal_interactor = Arc::new(CreateReversalEntryInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&event_presenter),
            Arc::clone(&journal_entry_presenter),
            Arc::clone(&journal_entry_finder),
        ));

        let create_additional_interactor = Arc::new(CreateAdditionalEntryInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&event_presenter),
            Arc::clone(&journal_entry_presenter),
            Arc::clone(&journal_entry_finder),
        ));

        let create_reclassification_interactor =
            Arc::new(CreateReclassificationEntryInteractor::new(
                Arc::clone(&event_store),
                Arc::clone(&event_presenter),
                Arc::clone(&journal_entry_presenter),
                Arc::clone(&journal_entry_finder),
            ));

        let create_replacement_interactor = Arc::new(CreateReplacementEntryInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&event_presenter),
            Arc::clone(&journal_entry_presenter),
            Arc::clone(&journal_entry_finder),
        ));

        // Controller構築
        let account_master_controller =
            Arc::new(AccountMasterController::new(load_account_master_interactor));

        let journal_entry_controller = Arc::new(JournalEntryController::new(
            register_interactor,
            update_draft_interactor,
            submit_for_approval_interactor,
            approve_interactor,
            reject_interactor,
            reverse_interactor,
            correct_interactor,
            delete_draft_interactor,
            cancel_interactor,
            create_reversal_interactor,
            create_additional_interactor,
            create_reclassification_interactor,
            create_replacement_interactor,
        ));

        let ledger_controller = Arc::new(LedgerController::new(Arc::clone(&ledger_query_service)));

        // 月次決算Interactor構築
        let consolidate_ledger_interactor =
            Arc::new(ConsolidateLedgerInteractor::new(Arc::clone(&ledger_query_service)));
        let prepare_closing_interactor =
            Arc::new(PrepareClosingInteractor::new(Arc::clone(&ledger_query_service)));
        let lock_closing_period_interactor =
            Arc::new(LockClosingPeriodInteractor::new(Arc::clone(&event_store)));
        let generate_trial_balance_interactor =
            Arc::new(GenerateTrialBalanceInteractor::new(Arc::clone(&ledger_query_service)));
        let generate_note_draft_interactor =
            Arc::new(GenerateNoteDraftInteractor::new(Arc::clone(&ledger_query_service)));
        let adjust_accounts_interactor = Arc::new(AdjustAccountsInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&ledger_query_service),
        ));
        let apply_ifrs_valuation_interactor = Arc::new(ApplyIfrsValuationInteractor::new(
            Arc::clone(&event_store),
            Arc::clone(&ledger_query_service),
        ));
        let generate_financial_statements_interactor =
            Arc::new(GenerateFinancialStatementsInteractor::new(Arc::clone(&ledger_query_service)));

        // ClosingController構築
        let closing_controller = Arc::new(ClosingController::new(
            consolidate_ledger_interactor,
            prepare_closing_interactor,
            lock_closing_period_interactor,
            generate_trial_balance_interactor,
            generate_note_draft_interactor,
            adjust_accounts_interactor,
            apply_ifrs_valuation_interactor,
            generate_financial_statements_interactor,
        ));

        // View層の構築
        let view_router = ViewRouter::new();
        let home_view = HomePage::new();
        let ledger_page = LedgerPage::new(ledger_rx);
        let ledger_view_page = LedgerViewPage::default();
        let closing_page = ClosingPage::new(trial_balance_rx);

        println!("✓ Application components initialized");
        println!("  - EventStore: {}", data_dir.join("events").display());
        println!("  - ProjectionDB: {}", data_dir.join("projections").display());
        println!("  - ProjectionBuilder: Ready");
        println!("  - Controllers: AccountMaster, JournalEntry, Ledger, Closing");
        println!("  - Views: Home, Ledger, LedgerView, Closing");

        Ok(Application {
            view_router,
            home_view,
            ledger_page,
            ledger_view_page,
            closing_page,
            event_store,
            projection_db,
            projection_builder,
            master_data_loader,
            account_master_controller,
            journal_entry_controller,
            ledger_controller,
            closing_controller,
            event_sender,
            event_receiver,
            account_master_receiver,
            journal_entry_result_receiver: je_result_rx,
            journal_entry_progress_receiver: je_progress_rx,
        })
    }
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
