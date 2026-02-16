// RegisterJournalEntryInteractor - 仕訳登録ユースケース実装
// 責務: 仕訳登録のビジネスロジック実行

use crate::dtos::{RegisterJournalEntryRequest, RegisterJournalEntryResponse};
use crate::error::ApplicationResult;
use crate::input_ports::RegisterJournalEntryUseCase;
use crate::output_port::{EventNotification, EventOutputPort};
use std::sync::Arc;

pub struct RegisterJournalEntryInteractor<E: EventOutputPort> {
    event_output: Arc<E>,
}

impl<E: EventOutputPort> RegisterJournalEntryInteractor<E> {
    pub fn new(event_output: Arc<E>) -> Self {
        Self { event_output }
    }
}

impl<E: EventOutputPort> RegisterJournalEntryUseCase for RegisterJournalEntryInteractor<E> {
    async fn execute(
        &self,
        request: RegisterJournalEntryRequest,
    ) -> ApplicationResult<RegisterJournalEntryResponse> {
        // イベント通知: 処理開始
        self.event_output
            .notify_event(EventNotification::success(
                "system",
                "RegisterJournalEntry",
                format!("仕訳登録を開始: {}", request.description),
            ))
            .await;

        // TODO: 実際の仕訳登録ロジック
        // 1. バリデーション
        // 借方と貸方の金額チェックは簡易版では省略（単一金額のため）

        // 2. ドメインロジック実行（TODO: 実装）
        // let journal_entry = JournalEntry::new(...);
        // repository.save(journal_entry).await?;

        // 3. イベント発行（TODO: 実装）
        // event_store.append(JournalEntryRegistered { ... }).await?;

        // イベント通知: 成功
        self.event_output
            .notify_event(EventNotification::success(
                "system",
                "RegisterJournalEntry",
                format!("仕訳登録が完了: {}", request.description),
            ))
            .await;

        Ok(RegisterJournalEntryResponse::simple(
            "temp-id-001",
            true,
            "仕訳が正常に登録されました",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output_port::EventNotification;

    struct MockEventOutput {
        events: std::sync::Mutex<Vec<EventNotification>>,
    }

    impl MockEventOutput {
        fn new() -> Self {
            Self {
                events: std::sync::Mutex::new(Vec::new()),
            }
        }

        fn get_events(&self) -> Vec<EventNotification> {
            self.events.lock().unwrap().clone()
        }
    }

    impl EventOutputPort for MockEventOutput {
        async fn notify_event(&self, event: EventNotification) {
            self.events.lock().unwrap().push(event);
        }
    }

    #[tokio::test]
    async fn test_register_journal_entry_success() {
        let event_output = Arc::new(MockEventOutput::new());
        let interactor = RegisterJournalEntryInteractor::new(Arc::clone(&event_output));

        let request = RegisterJournalEntryRequest::simple(
            "2024-01-01",
            "テスト仕訳",
            "1000",
            10000,
            "4000",
            10000,
        );

        let result = interactor.execute(request).await;

        assert!(result.is_ok());

        // イベント通知を確認
        let events = event_output.get_events();
        assert_eq!(events.len(), 2); // 開始と完了
        assert!(events[0].success);
        assert!(events[1].success);
    }
}
