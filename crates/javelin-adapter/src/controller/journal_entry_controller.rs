// JournalEntryController実装
// 仕訳登録に関する外部入力を受け付ける

use std::sync::Arc;

use javelin_application::{
    dtos::{
        ApproveJournalEntryRequest, CancelJournalEntryRequest, CorrectJournalEntryRequest,
        CreateAdditionalEntryRequest, CreateReclassificationEntryRequest,
        CreateReplacementEntryRequest, CreateReversalEntryRequest, DeleteDraftJournalEntryRequest,
        RegisterJournalEntryRequest, RejectJournalEntryRequest, ReverseJournalEntryRequest,
        SubmitForApprovalRequest, UpdateDraftJournalEntryRequest,
    },
    input_ports::{
        ApproveJournalEntryUseCase, CancelJournalEntryUseCase, CorrectJournalEntryUseCase,
        CreateAdditionalEntryUseCase, CreateReclassificationEntryUseCase,
        CreateReplacementEntryUseCase, CreateReversalEntryUseCase, DeleteDraftJournalEntryUseCase,
        RegisterJournalEntryUseCase, RejectJournalEntryUseCase, ReverseJournalEntryUseCase,
        SubmitForApprovalUseCase, UpdateDraftJournalEntryUseCase,
    },
};

/// 仕訳登録コントローラ
///
/// 仕訳登録に関するすべての操作を受け付ける。
/// ユースケースへの委譲のみを行い、ビジネスロジックは含まない。
pub struct JournalEntryController<R, U, S, A, J, V, C, D, CN, CR, CA, CL, CP>
where
    R: RegisterJournalEntryUseCase,
    U: UpdateDraftJournalEntryUseCase,
    S: SubmitForApprovalUseCase,
    A: ApproveJournalEntryUseCase,
    J: RejectJournalEntryUseCase,
    V: ReverseJournalEntryUseCase,
    C: CorrectJournalEntryUseCase,
    D: DeleteDraftJournalEntryUseCase,
    CN: CancelJournalEntryUseCase,
    CR: CreateReversalEntryUseCase,
    CA: CreateAdditionalEntryUseCase,
    CL: CreateReclassificationEntryUseCase,
    CP: CreateReplacementEntryUseCase,
{
    register_use_case: Arc<R>,
    update_draft_use_case: Arc<U>,
    submit_for_approval_use_case: Arc<S>,
    approve_use_case: Arc<A>,
    reject_use_case: Arc<J>,
    reverse_use_case: Arc<V>,
    correct_use_case: Arc<C>,
    delete_draft_use_case: Arc<D>,
    cancel_use_case: Arc<CN>,
    create_reversal_use_case: Arc<CR>,
    create_additional_use_case: Arc<CA>,
    create_reclassification_use_case: Arc<CL>,
    create_replacement_use_case: Arc<CP>,
}

impl<R, U, S, A, J, V, C, D, CN, CR, CA, CL, CP>
    JournalEntryController<R, U, S, A, J, V, C, D, CN, CR, CA, CL, CP>
where
    R: RegisterJournalEntryUseCase,
    U: UpdateDraftJournalEntryUseCase,
    S: SubmitForApprovalUseCase,
    A: ApproveJournalEntryUseCase,
    J: RejectJournalEntryUseCase,
    V: ReverseJournalEntryUseCase,
    C: CorrectJournalEntryUseCase,
    D: DeleteDraftJournalEntryUseCase,
    CN: CancelJournalEntryUseCase,
    CR: CreateReversalEntryUseCase,
    CA: CreateAdditionalEntryUseCase,
    CL: CreateReclassificationEntryUseCase,
    CP: CreateReplacementEntryUseCase,
{
    /// 新しいコントローラインスタンスを作成
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        register_use_case: Arc<R>,
        update_draft_use_case: Arc<U>,
        submit_for_approval_use_case: Arc<S>,
        approve_use_case: Arc<A>,
        reject_use_case: Arc<J>,
        reverse_use_case: Arc<V>,
        correct_use_case: Arc<C>,
        delete_draft_use_case: Arc<D>,
        cancel_use_case: Arc<CN>,
        create_reversal_use_case: Arc<CR>,
        create_additional_use_case: Arc<CA>,
        create_reclassification_use_case: Arc<CL>,
        create_replacement_use_case: Arc<CP>,
    ) -> Self {
        Self {
            register_use_case,
            update_draft_use_case,
            submit_for_approval_use_case,
            approve_use_case,
            reject_use_case,
            reverse_use_case,
            correct_use_case,
            delete_draft_use_case,
            cancel_use_case,
            create_reversal_use_case,
            create_additional_use_case,
            create_reclassification_use_case,
            create_replacement_use_case,
        }
    }

    /// 仕訳を登録（下書き作成）
    pub async fn register_journal_entry(
        &self,
        request: RegisterJournalEntryRequest,
    ) -> Result<(), String> {
        self.register_use_case.execute(request).await.map_err(|e| e.to_string())
    }

    /// 下書き仕訳を更新
    pub async fn update_draft_journal_entry(
        &self,
        request: UpdateDraftJournalEntryRequest,
    ) -> Result<(), String> {
        self.update_draft_use_case.execute(request).await.map_err(|e| e.to_string())
    }

    /// 承認申請
    pub async fn submit_for_approval(
        &self,
        request: SubmitForApprovalRequest,
    ) -> Result<(), String> {
        self.submit_for_approval_use_case
            .execute(request)
            .await
            .map_err(|e| e.to_string())
    }

    /// 仕訳を承認
    pub async fn approve_journal_entry(
        &self,
        request: ApproveJournalEntryRequest,
    ) -> Result<(), String> {
        self.approve_use_case.execute(request).await.map_err(|e| e.to_string())
    }

    /// 仕訳を差戻し
    pub async fn reject_journal_entry(
        &self,
        request: RejectJournalEntryRequest,
    ) -> Result<(), String> {
        self.reject_use_case.execute(request).await.map_err(|e| e.to_string())
    }

    /// 仕訳を取消
    pub async fn reverse_journal_entry(
        &self,
        request: ReverseJournalEntryRequest,
    ) -> Result<(), String> {
        self.reverse_use_case.execute(request).await.map_err(|e| e.to_string())
    }

    /// 仕訳を修正
    pub async fn correct_journal_entry(
        &self,
        request: CorrectJournalEntryRequest,
    ) -> Result<(), String> {
        self.correct_use_case.execute(request).await.map_err(|e| e.to_string())
    }

    /// 下書き仕訳を削除
    pub async fn delete_draft_journal_entry(
        &self,
        request: DeleteDraftJournalEntryRequest,
    ) -> Result<(), String> {
        self.delete_draft_use_case.execute(request).await.map_err(|e| e.to_string())
    }

    /// 取消仕訳を登録
    pub async fn cancel_journal_entry(
        &self,
        request: CancelJournalEntryRequest,
    ) -> Result<(), String> {
        self.cancel_use_case.execute(request).await.map_err(|e| e.to_string())
    }

    /// 反対仕訳を登録
    pub async fn create_reversal_entry(
        &self,
        request: CreateReversalEntryRequest,
    ) -> Result<(), String> {
        self.create_reversal_use_case.execute(request).await.map_err(|e| e.to_string())
    }

    /// 追加仕訳を登録
    pub async fn create_additional_entry(
        &self,
        request: CreateAdditionalEntryRequest,
    ) -> Result<(), String> {
        self.create_additional_use_case
            .execute(request)
            .await
            .map_err(|e| e.to_string())
    }

    /// 再分類仕訳を登録
    pub async fn create_reclassification_entry(
        &self,
        request: CreateReclassificationEntryRequest,
    ) -> Result<(), String> {
        self.create_reclassification_use_case
            .execute(request)
            .await
            .map_err(|e| e.to_string())
    }

    /// 洗替仕訳を登録
    pub async fn create_replacement_entry(
        &self,
        request: CreateReplacementEntryRequest,
    ) -> Result<(), String> {
        self.create_replacement_use_case
            .execute(request)
            .await
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    // テストはモックを使用して実装する必要があるため、ここでは省略
}
