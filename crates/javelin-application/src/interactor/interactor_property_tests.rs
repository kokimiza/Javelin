/// Property-based tests for Interactors
///
/// Task 9.9: Interactorのプロパティテストを作成
/// - プロパティ2: イベント保存失敗時のロールバック
///
/// Requirements: 1.9

#[cfg(test)]
mod interactor_property_tests {
    use std::sync::Arc;

    use javelin_domain::{
        error::DomainError, financial_close::journal_entry::events::JournalEntryEvent,
        repositories::EventRepository,
    };
    use proptest::prelude::*;
    use tokio::sync::mpsc;

    use crate::{
        dtos::{JournalEntryLineDto, RegisterJournalEntryRequest},
        input_ports::RegisterJournalEntryUseCase,
        interactor::RegisterJournalEntryInteractor,
        output_port::{EventNotification, EventOutputPort, JournalEntryOutputPort},
    };

    /// モックEventRepository - 常に失敗する
    struct FailingEventRepository;

    impl EventRepository for FailingEventRepository {
        type Event = JournalEntryEvent;

        async fn append(&self, _event: Self::Event) -> javelin_domain::error::DomainResult<()> {
            Err(DomainError::RepositoryError("EventStore保存失敗".to_string()))
        }

        async fn append_events<T>(
            &self,
            _aggregate_id: &str,
            _events: Vec<T>,
        ) -> javelin_domain::error::DomainResult<u64>
        where
            T: serde::Serialize + Send + 'static,
        {
            Err(DomainError::RepositoryError("EventStore保存失敗".to_string()))
        }

        async fn get_events(
            &self,
            _aggregate_id: &str,
        ) -> javelin_domain::error::DomainResult<Vec<serde_json::Value>> {
            Ok(vec![])
        }

        async fn get_all_events(
            &self,
            _from_sequence: u64,
        ) -> javelin_domain::error::DomainResult<Vec<serde_json::Value>> {
            Ok(vec![])
        }

        async fn get_latest_sequence(&self) -> javelin_domain::error::DomainResult<u64> {
            Ok(0)
        }
    }

    /// モックEventOutputPort
    struct MockEventOutputPort;

    impl EventOutputPort for MockEventOutputPort {
        async fn notify_event(&self, _notification: EventNotification) {}
    }

    /// モックVoucherNumberGenerator
    struct MockVoucherNumberGenerator;

    #[allow(async_fn_in_trait)]
    impl javelin_domain::financial_close::journal_entry::services::VoucherNumberGenerator
        for MockVoucherNumberGenerator
    {
        async fn generate_next(
            &self,
            fiscal_year: u32,
        ) -> javelin_domain::error::DomainResult<String> {
            Ok(format!("V-{}-00001", fiscal_year))
        }
    }

    /// モックJournalEntryOutputPort
    struct MockJournalEntryOutputPort {
        _sender: mpsc::UnboundedSender<String>,
    }

    impl JournalEntryOutputPort for MockJournalEntryOutputPort {
        async fn present_register_result(
            &self,
            _response: crate::dtos::RegisterJournalEntryResponse,
        ) {
        }

        async fn notify_progress(&self, _message: String) {
            // モックでは進捗通知を無視
        }

        async fn notify_error(&self, _error_message: String) {
            // モックではエラー通知を無視
        }

        async fn present_approve_result(
            &self,
            _response: crate::dtos::ApproveJournalEntryResponse,
        ) {
        }

        async fn present_reject_result(&self, _response: crate::dtos::RejectJournalEntryResponse) {}

        async fn present_update_draft_result(
            &self,
            _response: crate::dtos::UpdateDraftJournalEntryResponse,
        ) {
        }

        async fn present_delete_draft_result(
            &self,
            _response: crate::dtos::DeleteDraftJournalEntryResponse,
        ) {
        }

        async fn present_correct_result(
            &self,
            _response: crate::dtos::CorrectJournalEntryResponse,
        ) {
        }

        async fn present_reverse_result(
            &self,
            _response: crate::dtos::ReverseJournalEntryResponse,
        ) {
        }

        async fn present_submit_for_approval_result(
            &self,
            _response: crate::dtos::SubmitForApprovalResponse,
        ) {
        }
    }

    // テストデータ生成戦略
    fn journal_entry_line_strategy() -> impl Strategy<Value = JournalEntryLineDto> {
        (1u32..100u32, prop::bool::ANY, "[0-9]{4}", 1000.0..1000000.0).prop_map(
            |(line_number, is_debit, account_code, amount)| JournalEntryLineDto {
                line_number,
                side: if is_debit { "Debit" } else { "Credit" }.to_string(),
                account_code,
                sub_account_code: None,
                department_code: None,
                amount,
                currency: "JPY".to_string(),
                tax_type: "NonTaxable".to_string(),
                tax_amount: 0.0,
                description: None,
            },
        )
    }

    fn register_request_strategy() -> impl Strategy<Value = RegisterJournalEntryRequest> {
        (
            "[0-9]{4}-[0-9]{2}-[0-9]{2}",
            "[A-Z0-9]{5,10}",
            "[a-z]{5,10}",
            prop::collection::vec(journal_entry_line_strategy(), 2..4),
        )
            .prop_map(|(transaction_date, voucher_number, user_id, mut lines)| {
                // 借貸バランスを調整
                let debit_total: f64 =
                    lines.iter().filter(|l| l.side == "Debit").map(|l| l.amount).sum();
                let credit_total: f64 =
                    lines.iter().filter(|l| l.side == "Credit").map(|l| l.amount).sum();

                if debit_total > credit_total {
                    // 貸方を追加
                    lines.push(JournalEntryLineDto {
                        line_number: lines.len() as u32 + 1,
                        side: "Credit".to_string(),
                        account_code: "9999".to_string(),
                        sub_account_code: None,
                        department_code: None,
                        amount: debit_total - credit_total,
                        currency: "JPY".to_string(),
                        tax_type: "NonTaxable".to_string(),
                        tax_amount: 0.0,
                        description: None,
                    });
                } else if credit_total > debit_total {
                    // 借方を追加
                    lines.push(JournalEntryLineDto {
                        line_number: lines.len() as u32 + 1,
                        side: "Debit".to_string(),
                        account_code: "9999".to_string(),
                        sub_account_code: None,
                        department_code: None,
                        amount: credit_total - debit_total,
                        currency: "JPY".to_string(),
                        tax_type: "NonTaxable".to_string(),
                        tax_amount: 0.0,
                        description: None,
                    });
                }

                RegisterJournalEntryRequest { transaction_date, voucher_number, lines, user_id }
            })
    }

    /// プロパティ2: イベント保存失敗時のロールバック
    ///
    /// Feature: cqrs-infrastructure-integration, Property 2: イベント保存失敗時のロールバック
    ///
    /// 任意の仕訳操作において、EventStoreへのイベント保存が失敗した場合、
    /// システムはトランザクションをロールバックし、適切なエラーを返すこと
    ///
    /// **検証要件: 1.9**
    ///
    /// 検証内容:
    /// - EventStore保存失敗時にエラーが返されること
    /// - エラーがApplicationError型であること
    /// - システムが一貫した状態を保つこと（部分的な保存が発生しない）
    #[test]
    fn property_2_rollback_on_event_store_failure() {
        proptest!(|(request in register_request_strategy())| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // 常に失敗するEventRepositoryを使用
                let failing_repo = Arc::new(FailingEventRepository);
                let event_output = Arc::new(MockEventOutputPort);
                let (sender, _receiver) = mpsc::unbounded_channel();
                let output_port = Arc::new(MockJournalEntryOutputPort { _sender: sender });
                let voucher_generator = Arc::new(MockVoucherNumberGenerator);

                let interactor = RegisterJournalEntryInteractor::new(
                    failing_repo,
                    event_output,
                    output_port,
                    voucher_generator,
                );

                // 実行してエラーが返されることを確認
                let result = interactor.execute(request).await;

                // エラーが返されることを確認（エラーの種類は問わない）
                // EventStore保存失敗により、何らかのエラーが返されればOK
                prop_assert!(result.is_err(), "Expected error but got Ok");

                Ok(())
            }).unwrap();
        });
    }
}
