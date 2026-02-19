// RegisterJournalEntryInteractor - 仕訳登録ユースケース実装
// 責務: 仕訳登録のビジネスロジック実行

use std::sync::Arc;

use chrono::{Datelike, NaiveDate};
use javelin_domain::{
    entity::EntityId,
    financial_close::journal_entry::{
        entities::{JournalEntry, JournalEntryId},
        services::{JournalEntryService, VoucherNumberGenerator},
        values::{TransactionDate, UserId, VoucherNumber},
    },
    repositories::EventRepository,
};

use crate::{
    dtos::{RegisterJournalEntryRequest, RegisterJournalEntryResponse},
    error::{ApplicationError, ApplicationResult},
    input_ports::RegisterJournalEntryUseCase,
    output_port::{EventNotification, EventOutputPort, JournalEntryOutputPort},
};

pub struct RegisterJournalEntryInteractor<
    R: EventRepository,
    E: EventOutputPort,
    O: JournalEntryOutputPort,
    V: VoucherNumberGenerator,
> {
    event_repository: Arc<R>,
    event_output: Arc<E>,
    output_port: Arc<O>,
    voucher_generator: Arc<V>,
}

impl<R: EventRepository, E: EventOutputPort, O: JournalEntryOutputPort, V: VoucherNumberGenerator>
    RegisterJournalEntryInteractor<R, E, O, V>
{
    pub fn new(
        event_repository: Arc<R>,
        event_output: Arc<E>,
        output_port: Arc<O>,
        voucher_generator: Arc<V>,
    ) -> Self {
        Self { event_repository, event_output, output_port, voucher_generator }
    }
}

impl<R: EventRepository, E: EventOutputPort, O: JournalEntryOutputPort, V: VoucherNumberGenerator>
    RegisterJournalEntryUseCase for RegisterJournalEntryInteractor<R, E, O, V>
{
    async fn execute(&self, request: RegisterJournalEntryRequest) -> ApplicationResult<()> {
        // イベント通知: 処理開始
        self.event_output
            .notify_event(EventNotification::success(
                "system",
                "RegisterJournalEntry",
                "仕訳登録を開始".to_string(),
            ))
            .await;

        // 進捗通知: 入力検証開始
        self.output_port
            .notify_progress("入力データを検証しています...".to_string())
            .await;

        // 1. 入力バリデーション - 取引日付のパース
        let transaction_date =
            match NaiveDate::parse_from_str(&request.transaction_date, "%Y-%m-%d") {
                Ok(date) => date,
                Err(e) => {
                    let error_msg =
                        format!("日付形式が不正です: {} (エラー: {})", request.transaction_date, e);
                    self.output_port.notify_error(error_msg.clone()).await;
                    return Err(ApplicationError::ValidationFailed(vec![error_msg]));
                }
            };

        let transaction_date = match TransactionDate::new(transaction_date) {
            Ok(date) => date,
            Err(e) => {
                let error_msg = format!("取引日付が無効です: {}", e);
                self.output_port.notify_error(error_msg.clone()).await;
                return Err(ApplicationError::DomainError(e));
            }
        };

        // 2. 証憑番号の作成（空の場合は自動生成）
        let voucher_number_str = if request.voucher_number.is_empty() {
            // 取引日付から年度を取得（簡易的に年を使用）
            let fiscal_year = transaction_date.value().year() as u32;

            self.output_port
                .notify_progress("伝票番号を自動生成しています...".to_string())
                .await;

            match self.voucher_generator.generate_next(fiscal_year).await {
                Ok(vn) => vn,
                Err(e) => {
                    let error_msg = format!("伝票番号の自動生成に失敗しました: {}", e);
                    self.output_port.notify_error(error_msg.clone()).await;
                    return Err(ApplicationError::DomainError(e));
                }
            }
        } else {
            request.voucher_number.clone()
        };

        let voucher_number = match VoucherNumber::new(voucher_number_str) {
            Ok(vn) => vn,
            Err(e) => {
                let error_msg = format!("伝票番号が無効です: {}", e);
                self.output_port.notify_error(error_msg.clone()).await;
                return Err(ApplicationError::DomainError(e));
            }
        };

        // 3. ユーザーIDの作成
        let user_id = UserId::new(request.user_id.clone());

        // 進捗通知: 明細行の作成
        self.output_port
            .notify_progress("仕訳明細を作成しています...".to_string())
            .await;

        // 4. 仕訳明細の作成
        let lines: Result<Vec<_>, _> = request.lines.iter().map(|dto| dto.try_into()).collect();
        let lines = match lines {
            Ok(l) => l,
            Err(e) => {
                let error_msg = format!("仕訳明細の作成に失敗しました: {}", e);
                self.output_port.notify_error(error_msg.clone()).await;
                return Err(e);
            }
        };

        // 進捗通知: 借貸バランスチェック
        self.output_port
            .notify_progress("借貸バランスを検証しています...".to_string())
            .await;

        // 5. 借貸バランスチェック
        if let Err(e) = JournalEntryService::validate_balance(&lines) {
            let error_msg = format!("借貸バランスが一致しません: {}", e);
            self.output_port.notify_error(error_msg.clone()).await;
            return Err(ApplicationError::DomainError(e));
        }

        // 進捗通知: 仕訳エンティティの作成
        self.output_port
            .notify_progress("仕訳エンティティを作成しています...".to_string())
            .await;

        // 6. 仕訳IDの生成（UUIDを使用）
        let entry_id = JournalEntryId::new(uuid::Uuid::new_v4().to_string());

        // 7. 仕訳エンティティの作成（Draft状態）
        let journal_entry = match JournalEntry::new(
            entry_id.clone(),
            transaction_date,
            voucher_number,
            lines,
            user_id,
        ) {
            Ok(je) => je,
            Err(e) => {
                let error_msg = format!("仕訳エンティティの作成に失敗しました: {}", e);
                self.output_port.notify_error(error_msg.clone()).await;
                return Err(ApplicationError::DomainError(e));
            }
        };

        // 8. イベントの取得（DraftCreatedイベントが含まれる）
        let events = journal_entry.events();

        // 進捗通知: イベントストアへの保存
        self.output_port
            .notify_progress("イベントストアへ保存しています...".to_string())
            .await;

        // 9. イベントストアへの保存
        if let Err(e) = self.event_repository.append_events(entry_id.value(), events.to_vec()).await
        {
            let error_msg = format!("イベントストアへの保存に失敗しました: {}", e);
            self.output_port.notify_error(error_msg.clone()).await;
            return Err(ApplicationError::DomainError(e));
        }

        // 進捗通知: 完了処理
        self.output_port
            .notify_progress("登録処理を完了しています...".to_string())
            .await;

        // 10. レスポンスDTOを作成してOutput Portへ送信
        let response = RegisterJournalEntryResponse {
            entry_id: entry_id.value().to_string(),
            status: journal_entry.status().as_str().to_string(),
        };
        self.output_port.present_register_result(response).await;

        // イベント通知: 成功
        self.event_output
            .notify_event(EventNotification::success(
                "system",
                "RegisterJournalEntry",
                "仕訳登録が完了".to_string(),
            ))
            .await;

        Ok(())
    }
}
