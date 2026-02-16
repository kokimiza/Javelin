// RecordUserActionInteractor - ユーザ操作記録ユースケース実装
// 責務: ユーザ操作をLMDBに記録

use crate::dtos::{RecordUserActionRequest, RecordUserActionResponse};
use crate::error::{ApplicationError, ApplicationResult};
use crate::input_ports::RecordUserActionUseCase;
use crate::output_port::{EventNotification, EventOutputPort};
use chrono::Utc;
use javelin_domain::repositories::UserActionRepository;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct RecordUserActionInteractor<R: UserActionRepository, E: EventOutputPort> {
    repository: Arc<R>,
    event_output: Arc<E>,
}

impl<R: UserActionRepository, E: EventOutputPort> RecordUserActionInteractor<R, E> {
    pub fn new(repository: Arc<R>, event_output: Arc<E>) -> Self {
        Self {
            repository,
            event_output,
        }
    }
}

impl<R: UserActionRepository, E: EventOutputPort> RecordUserActionUseCase
    for RecordUserActionInteractor<R, E>
{
    fn execute(
        &self,
        request: RecordUserActionRequest,
    ) -> Pin<Box<dyn Future<Output = ApplicationResult<RecordUserActionResponse>> + Send + '_>>
    {
        Box::pin(async move {
            // リポジトリに保存
            match self
                .repository
                .save_action(&request.user, &request.location, &request.action)
                .await
            {
                Ok(action_id) => {
                    let response = RecordUserActionResponse {
                        action_id: action_id.clone(),
                        recorded_at: Utc::now(),
                    };

                    // 成功をイベントビューアに通知
                    self.event_output
                        .notify_event(EventNotification::success(
                            request.user,
                            request.location,
                            request.action,
                        ))
                        .await;

                    Ok(response)
                }
                Err(e) => {
                    // 失敗をイベントビューアに通知
                    self.event_output
                        .notify_event(EventNotification::failure(
                            request.user.clone(),
                            request.location.clone(),
                            format!("操作記録失敗: {} - {}", request.action, e),
                        ))
                        .await;

                    Err(ApplicationError::UseCaseExecutionFailed(format!(
                        "Failed to record user action: {}",
                        e
                    )))
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output_port::EventNotification;
    use javelin_domain::repositories::UserActionRepository;

    struct MockRepository;

    impl UserActionRepository for MockRepository {
        async fn save_action(
            &self,
            _user: &str,
            _location: &str,
            _action: &str,
        ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            Ok("test-action-id".to_string())
        }
    }

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
    async fn test_record_user_action_success() {
        let repository = Arc::new(MockRepository);
        let event_output = Arc::new(MockEventOutput::new());
        let interactor = RecordUserActionInteractor::new(repository, Arc::clone(&event_output));

        let request = RecordUserActionRequest {
            user: "test_user".to_string(),
            location: "HomePage".to_string(),
            action: "メニュー選択".to_string(),
        };

        let result = interactor.execute(request).await;

        assert!(result.is_ok());

        // イベント通知を確認
        let events = event_output.get_events();
        assert_eq!(events.len(), 1);
        assert!(events[0].success);
        assert_eq!(events[0].user, "test_user");
    }
}
