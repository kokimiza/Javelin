// ReverseJournalEntryInteractor - 取消ユースケース実装

use std::sync::Arc;

use javelin_domain::{
    entity::EntityId,
    financial_close::journal_entry::{events::JournalEntryEvent, values::UserId},
    repositories::EventRepository,
};

use crate::{
    dtos::{ReverseJournalEntryRequest, ReverseJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::ReverseJournalEntryUseCase,
    output_port::{EventNotification, EventOutputPort, JournalEntryOutputPort},
};

pub struct ReverseJournalEntryInteractor<
    R: EventRepository,
    E: EventOutputPort,
    O: JournalEntryOutputPort,
> {
    event_repository: Arc<R>,
    event_output: Arc<E>,
    output_port: Arc<O>,
}

impl<R: EventRepository, E: EventOutputPort, O: JournalEntryOutputPort>
    ReverseJournalEntryInteractor<R, E, O>
{
    pub fn new(event_repository: Arc<R>, event_output: Arc<E>, output_port: Arc<O>) -> Self {
        Self { event_repository, event_output, output_port }
    }
}

impl<R: EventRepository, E: EventOutputPort, O: JournalEntryOutputPort> ReverseJournalEntryUseCase
    for ReverseJournalEntryInteractor<R, E, O>
{
    async fn execute(&self, request: ReverseJournalEntryRequest) -> ApplicationResult<()> {
        self.event_output
            .notify_event(EventNotification::success(
                "system",
                "ReverseJournalEntry",
                format!("取消処理を開始: {}", request.entry_id),
            ))
            .await;

        // 取消イベントを生成
        let user_id = UserId::new(request.user_id.clone());
        let reversal_entry_id = format!("REV-{}", request.entry_id);

        let event = JournalEntryEvent::Reversed {
            entry_id: reversal_entry_id.clone(),
            original_id: request.entry_id.clone(),
            reason: request.reason.clone(),
            reversed_by: user_id.value().to_string(),
            reversed_at: chrono::Utc::now(),
        };

        // イベントストアへの保存
        self.event_repository
            .append_events(&reversal_entry_id, vec![event])
            .await
            .map_err(ApplicationError::DomainError)?;

        let response = ReverseJournalEntryResponse {
            entry_id: reversal_entry_id,
            original_entry_id: request.entry_id,
            status: "Reversed".to_string(),
            reversed_at: chrono::Utc::now().to_rfc3339(),
        };
        self.output_port.present_reverse_result(response).await;

        self.event_output
            .notify_event(EventNotification::success(
                "system",
                "ReverseJournalEntry",
                "取消処理が完了",
            ))
            .await;

        Ok(())
    }
}
