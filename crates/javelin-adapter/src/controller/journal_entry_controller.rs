// JournalEntryController実装
// 仕訳登録に関する外部入力を受け付ける

use std::sync::Arc;

use javelin_application::dtos::RegisterJournalEntryRequest;
use javelin_infrastructure::{event_store::EventStore, services::VoucherNumberGeneratorImpl};

/// 仕訳登録コントローラ
///
/// 仕訳登録に関するすべての操作を受け付ける。
/// ユースケースへの委譲のみを行い、ビジネスロジックは含まない。
pub struct JournalEntryController {
    event_store: Arc<EventStore>,
    voucher_generator: Arc<VoucherNumberGeneratorImpl>,
    presenter_registry: Arc<crate::navigation::PresenterRegistry>,
}

impl JournalEntryController {
    /// 新しいコントローラインスタンスを作成
    pub fn new(
        event_store: Arc<EventStore>,
        voucher_generator: Arc<VoucherNumberGeneratorImpl>,
        presenter_registry: Arc<crate::navigation::PresenterRegistry>,
    ) -> Self {
        Self { event_store, voucher_generator, presenter_registry }
    }

    /// PresenterRegistryへの参照を取得
    pub fn presenter_registry(&self) -> &Arc<crate::navigation::PresenterRegistry> {
        &self.presenter_registry
    }

    /// 仕訳を登録（下書き作成）
    ///
    /// # Arguments
    /// * `page_id` - ページインスタンスID（PresenterRegistry検索用）
    /// * `request` - 登録リクエスト
    ///
    /// # Returns
    /// * `Ok(())` - 登録成功（結果はOutputPort経由で通知）
    /// * `Err(String)` - 登録失敗
    pub async fn handle_register_journal_entry(
        &self,
        page_id: uuid::Uuid,
        request: RegisterJournalEntryRequest,
    ) -> Result<(), String> {
        use javelin_application::input_ports::RegisterJournalEntryUseCase;

        // PresenterRegistryからpage_id用のPresenterを取得
        if let Some(journal_entry_presenter_arc) =
            self.presenter_registry.get_journal_entry_presenter(page_id)
        {
            // ArcからPresenterをclone
            let journal_entry_presenter = (*journal_entry_presenter_arc).clone();

            // EventPresenterはダミーを作成（イベント通知は不要）
            let (event_tx, _) = tokio::sync::mpsc::unbounded_channel();
            let event_presenter = Arc::new(crate::presenter::Presenter::new(event_tx));

            // このページ専用のInteractorを動的に作成
            let interactor = javelin_application::interactor::RegisterJournalEntryInteractor::new(
                Arc::clone(&self.event_store),
                event_presenter,
                journal_entry_presenter.into(),
                Arc::clone(&self.voucher_generator),
            );

            // 実行
            interactor.execute(request).await.map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err(format!("JournalEntryPresenter not found for page_id: {}", page_id))
        }
    }
}
