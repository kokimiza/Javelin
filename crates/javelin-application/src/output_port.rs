// OutputPort - 出力抽象
// 責務: Presenter連携

use crate::{
    dtos::response::{
        ApproveJournalEntryResponse, CorrectJournalEntryResponse, DeleteDraftJournalEntryResponse,
        JournalEntryDetail, JournalEntryListResult, JournalEntrySearchResultDto,
        LoadAccountMasterResponse, LoadApplicationSettingsResponse, LoadCompanyMasterResponse,
        LoadSubsidiaryAccountMasterResponse, RegisterJournalEntryResponse,
        RejectJournalEntryResponse, ReverseJournalEntryResponse, SubmitForApprovalResponse,
        UpdateDraftJournalEntryResponse,
    },
    query_service::{LedgerResult, TrialBalanceResult},
};

/// イベント通知情報
#[derive(Clone, Debug)]
pub struct EventNotification {
    pub user: String,
    pub location: String,
    pub action: String,
    pub success: bool,
}

impl EventNotification {
    pub fn success(
        user: impl Into<String>,
        location: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            user: user.into(),
            location: location.into(),
            action: action.into(),
            success: true,
        }
    }

    pub fn failure(
        user: impl Into<String>,
        location: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            user: user.into(),
            location: location.into(),
            action: action.into(),
            success: false,
        }
    }
}

/// OutputPort - ユースケース結果の出力
pub trait OutputPort: Send + Sync {
    type Output;

    fn present(&self, output: Self::Output);
}

/// EventOutputPort - イベント通知専用
pub trait EventOutputPort: Send + Sync {
    /// イベントをイベントビューアに通知（非同期）
    fn notify_event(
        &self,
        event: EventNotification,
    ) -> impl std::future::Future<Output = ()> + Send;
}

/// JournalEntryOutputPort - 仕訳ユースケース結果の出力
#[allow(async_fn_in_trait)]
pub trait JournalEntryOutputPort: Send + Sync {
    /// 仕訳登録結果を出力
    async fn present_register_result(&self, response: RegisterJournalEntryResponse);

    /// 処理進捗を通知
    async fn notify_progress(&self, message: String);

    /// エラーを通知
    async fn notify_error(&self, error_message: String);

    /// 下書き更新結果を出力
    async fn present_update_draft_result(&self, response: UpdateDraftJournalEntryResponse);

    /// 承認申請結果を出力
    async fn present_submit_for_approval_result(&self, response: SubmitForApprovalResponse);

    /// 承認結果を出力
    async fn present_approve_result(&self, response: ApproveJournalEntryResponse);

    /// 差戻し結果を出力
    async fn present_reject_result(&self, response: RejectJournalEntryResponse);

    /// 取消結果を出力
    async fn present_reverse_result(&self, response: ReverseJournalEntryResponse);

    /// 修正結果を出力
    async fn present_correct_result(&self, response: CorrectJournalEntryResponse);

    /// 削除結果を出力
    async fn present_delete_draft_result(&self, response: DeleteDraftJournalEntryResponse);
}

/// QueryOutputPort - クエリ結果の出力
#[allow(async_fn_in_trait)]
pub trait QueryOutputPort: Send + Sync {
    /// 仕訳一覧結果を出力
    async fn present_journal_entry_list(&self, result: JournalEntryListResult);

    /// 仕訳詳細結果を出力
    async fn present_journal_entry_detail(&self, result: JournalEntryDetail);

    /// 元帳結果を出力
    async fn present_ledger(&self, result: LedgerResult);

    /// 試算表結果を出力
    async fn present_trial_balance(&self, result: TrialBalanceResult);
}

/// AccountMasterOutputPort - 勘定科目マスタ結果の出力
#[allow(async_fn_in_trait)]
pub trait AccountMasterOutputPort: Send + Sync {
    /// 勘定科目マスタ結果を出力
    async fn present_account_master(&self, response: &LoadAccountMasterResponse);
}

/// CompanyMasterOutputPort - 会社マスタ結果の出力
#[allow(async_fn_in_trait)]
pub trait CompanyMasterOutputPort: Send + Sync {
    /// 会社マスタ結果を出力
    async fn present_company_master(&self, response: &LoadCompanyMasterResponse);
}

/// ApplicationSettingsOutputPort - アプリケーション設定結果の出力
#[allow(async_fn_in_trait)]
pub trait ApplicationSettingsOutputPort: Send + Sync {
    /// アプリケーション設定結果を出力
    async fn present_application_settings(&self, response: &LoadApplicationSettingsResponse);
}

/// SubsidiaryAccountMasterOutputPort - 補助科目マスタ結果の出力
#[allow(async_fn_in_trait)]
pub trait SubsidiaryAccountMasterOutputPort: Send + Sync {
    /// 補助科目マスタ結果を出力
    async fn present_subsidiary_account_master(
        &self,
        response: &LoadSubsidiaryAccountMasterResponse,
    );
}

/// SearchOutputPort - 仕訳検索結果の出力
pub trait SearchOutputPort: Send + Sync {
    /// 検索結果を出力
    fn present_search_result(&self, result: JournalEntrySearchResultDto);

    /// バリデーションエラーを出力
    fn present_validation_error(&self, message: String);

    /// 検索結果0件を出力
    fn present_no_results(&self);

    /// 進捗状況を出力
    fn present_progress(&self, message: String);

    /// 実行時間を出力（ミリ秒）
    fn present_execution_time(&self, elapsed_ms: usize);
}
