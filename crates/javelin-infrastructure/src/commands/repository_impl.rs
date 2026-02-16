// Repository実装 - RepositoryTrait具象化

use crate::event_store::LmdbEventStore;
use javelin_domain::error::DomainResult;
use javelin_domain::event::DomainEvent;
use javelin_domain::repositories::EventRepository;
use std::sync::Arc;

pub struct EventRepositoryImpl<E: DomainEvent> {
    store: Arc<LmdbEventStore>,
    _phantom: std::marker::PhantomData<E>,
}

impl<E: DomainEvent> EventRepositoryImpl<E> {
    pub fn new(store: Arc<LmdbEventStore>) -> Self {
        Self {
            store,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<E: DomainEvent + serde::Serialize> EventRepository for EventRepositoryImpl<E> {
    type Event = E;

    async fn append(&self, event: Self::Event) -> DomainResult<()> {
        // イベントをシリアライズ
        let payload = serde_json::to_vec(&event)
            .map_err(|e| javelin_domain::error::DomainError::SerializationFailed(e.to_string()))?;

        // LMDBに保存
        self.store
            .append_event(
                event.event_type(),
                event.aggregate_id(),
                event.version(),
                &payload,
            )
            .await
            .map_err(|e| {
                javelin_domain::error::DomainError::RepositoryError(format!(
                    "Failed to append event: {}",
                    e
                ))
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use javelin_domain::event::Event;
    use serde::{Deserialize, Serialize};
    use tempfile::TempDir;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestEvent {
        event_type: String,
        aggregate_id: String,
        version: u64,
        data: String,
    }

    impl DomainEvent for TestEvent {
        fn event_type(&self) -> &str {
            &self.event_type
        }

        fn aggregate_id(&self) -> &str {
            &self.aggregate_id
        }

        fn version(&self) -> u64 {
            self.version
        }
    }

    #[tokio::test]
    async fn test_event_repository_append() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let event_store_path = temp_dir.path().join("events");

        let store = Arc::new(LmdbEventStore::new(&event_store_path).await.unwrap());
        let repo = EventRepositoryImpl::<TestEvent>::new(store);

        let event = TestEvent {
            event_type: "TestEvent".to_string(),
            aggregate_id: "test-001".to_string(),
            version: 1,
            data: "test data".to_string(),
        };

        let result = repo.append(event).await;
        assert!(result.is_ok(), "Event append should succeed");
    }
}
