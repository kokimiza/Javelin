/// Unit tests for Interactors
///
/// Task 9.10: Interactorのユニットテストを作成
/// - 正常な仕訳登録フロー
/// - バリデーションエラー
/// - EventStore保存失敗
///
/// Requirements: 1.1, 1.9

#[cfg(test)]
mod interactor_unit_tests {
    use std::sync::{Arc, Mutex};

    use javelin_domain::{
        error::DomainError, financial_close::journal_entry::events::JournalEntryEvent,
        repositories::EventRepository,
    };
    use tokio::sync::mpsc;

    use crate::{
        dtos::{JournalEntryLineDto, RegisterJournalEntryRequest, RegisterJournalEntryResponse},
        input_ports::RegisterJournalEntryUseCase,
        interactor::RegisterJournalEntryInteractor,
        output_port::{EventNotification, EventOutputPort, JournalEntryOutputPort},
    };

    /// モックEventRepository - 成功する
    struct MockEventRepository {
        saved_events: Arc<Mutex<Vec<(String, Vec<serde_json::Value>)>>>,
    }

    impl MockEventRepository {
        fn new() -> Self {
            Self { saved_events: Arc::new(Mutex::new(Vec::new())) }
        }

        fn get_saved_events(&self) -> Vec<(String, Vec<serde_json::Value>)> {
            self.saved_events.lock().unwrap().clone()
        }
    }

    impl EventRepository for MockEventRepository {
        type Event = JournalEntryEvent;

        async fn append(&self, _event: Self::Event) -> javelin_domain::error::DomainResult<()> {
            Ok(())
        }

        async fn append_events<T>(
            &self,
            aggregate_id: &str,
            events: Vec<T>,
        ) -> javelin_domain::error::DomainResult<u64>
        where
            T: serde::Serialize + Send + 'static,
        {
            let json_events: Vec<serde_json::Value> =
                events.into_iter().map(|e| serde_json::to_value(e).unwrap()).collect();

            self.saved_events
                .lock()
                .unwrap()
                .push((aggregate_id.to_string(), json_events.clone()));

            Ok(json_events.len() as u64)
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
        sender: mpsc::UnboundedSender<RegisterJournalEntryResponse>,
    }

    impl JournalEntryOutputPort for MockJournalEntryOutputPort {
        async fn present_register_result(&self, response: RegisterJournalEntryResponse) {
            let _ = self.sender.send(response);
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

    #[tokio::test]
    async fn test_successful_journal_entry_registration() {
        // 正常な仕訳登録フロー
        let repo = Arc::new(MockEventRepository::new());
        let event_output = Arc::new(MockEventOutputPort);
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let output_port = Arc::new(MockJournalEntryOutputPort { sender });
        let voucher_generator = Arc::new(MockVoucherNumberGenerator);

        let interactor = RegisterJournalEntryInteractor::new(
            Arc::clone(&repo),
            event_output,
            output_port,
            voucher_generator,
        );

        let request = RegisterJournalEntryRequest {
            transaction_date: "2024-01-15".to_string(),
            voucher_number: "V-001".to_string(),
            lines: vec![
                JournalEntryLineDto {
                    line_number: 1,
                    side: "Debit".to_string(),
                    account_code: "1010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
                JournalEntryLineDto {
                    line_number: 2,
                    side: "Credit".to_string(),
                    account_code: "4010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
            ],
            user_id: "user1".to_string(),
        };

        let result = interactor.execute(request).await;

        // 成功することを確認
        assert!(result.is_ok());

        // イベントが保存されたことを確認
        let saved_events = repo.get_saved_events();
        assert_eq!(saved_events.len(), 1);

        // レスポンスが送信されたことを確認
        let response = receiver.recv().await;
        assert!(response.is_some());
        let response = response.unwrap();
        assert_eq!(response.status, "Draft");
    }

    #[tokio::test]
    async fn test_validation_error_invalid_date() {
        // バリデーションエラー: 無効な日付形式
        let repo = Arc::new(MockEventRepository::new());
        let event_output = Arc::new(MockEventOutputPort);
        let (sender, _receiver) = mpsc::unbounded_channel();
        let output_port = Arc::new(MockJournalEntryOutputPort { sender });

        let voucher_generator = Arc::new(MockVoucherNumberGenerator);
        let interactor =
            RegisterJournalEntryInteractor::new(repo, event_output, output_port, voucher_generator);

        let request = RegisterJournalEntryRequest {
            transaction_date: "invalid-date".to_string(),
            voucher_number: "V-001".to_string(),
            lines: vec![
                JournalEntryLineDto {
                    line_number: 1,
                    side: "Debit".to_string(),
                    account_code: "1010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
                JournalEntryLineDto {
                    line_number: 2,
                    side: "Credit".to_string(),
                    account_code: "4010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
            ],
            user_id: "user1".to_string(),
        };

        let result = interactor.execute(request).await;

        // エラーが返されることを確認
        assert!(result.is_err());

        // ValidationFailedエラーであることを確認
        match result {
            Err(crate::error::ApplicationError::ValidationFailed(_)) => {
                // 期待通り
            }
            _ => panic!("Expected ValidationFailed error"),
        }
    }

    #[tokio::test]
    async fn test_validation_error_unbalanced_entry() {
        // バリデーションエラー: 借貸不一致
        let repo = Arc::new(MockEventRepository::new());
        let event_output = Arc::new(MockEventOutputPort);
        let (sender, _receiver) = mpsc::unbounded_channel();
        let output_port = Arc::new(MockJournalEntryOutputPort { sender });
        let voucher_generator = Arc::new(MockVoucherNumberGenerator);

        let interactor =
            RegisterJournalEntryInteractor::new(repo, event_output, output_port, voucher_generator);

        let request = RegisterJournalEntryRequest {
            transaction_date: "2024-01-15".to_string(),
            voucher_number: "V-001".to_string(),
            lines: vec![
                JournalEntryLineDto {
                    line_number: 1,
                    side: "Debit".to_string(),
                    account_code: "1010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
                JournalEntryLineDto {
                    line_number: 2,
                    side: "Credit".to_string(),
                    account_code: "4010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 50000.0, // 借貸不一致
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
            ],
            user_id: "user1".to_string(),
        };

        let result = interactor.execute(request).await;

        // エラーが返されることを確認
        assert!(result.is_err());

        // DomainErrorであることを確認
        match result {
            Err(crate::error::ApplicationError::DomainError(_)) => {
                // 期待通り
            }
            _ => panic!("Expected DomainError"),
        }
    }

    #[tokio::test]
    async fn test_event_store_save_failure() {
        // EventStore保存失敗
        let repo = Arc::new(FailingEventRepository);
        let event_output = Arc::new(MockEventOutputPort);
        let (sender, _receiver) = mpsc::unbounded_channel();
        let output_port = Arc::new(MockJournalEntryOutputPort { sender });
        let voucher_generator = Arc::new(MockVoucherNumberGenerator);

        let interactor =
            RegisterJournalEntryInteractor::new(repo, event_output, output_port, voucher_generator);

        let request = RegisterJournalEntryRequest {
            transaction_date: "2024-01-15".to_string(),
            voucher_number: "V-001".to_string(),
            lines: vec![
                JournalEntryLineDto {
                    line_number: 1,
                    side: "Debit".to_string(),
                    account_code: "1010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
                JournalEntryLineDto {
                    line_number: 2,
                    side: "Credit".to_string(),
                    account_code: "4010".to_string(),
                    sub_account_code: None,
                    department_code: None,
                    amount: 100000.0,
                    currency: "JPY".to_string(),
                    tax_type: "NonTaxable".to_string(),
                    tax_amount: 0.0,
                    description: None,
                },
            ],
            user_id: "user1".to_string(),
        };

        let result = interactor.execute(request).await;

        // エラーが返されることを確認
        assert!(result.is_err());

        // DomainErrorであることを確認
        match result {
            Err(crate::error::ApplicationError::DomainError(_)) => {
                // 期待通り
            }
            _ => panic!("Expected DomainError"),
        }
    }
}
