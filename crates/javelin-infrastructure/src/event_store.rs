// EventStore実装 - LMDBイベント保存
// 保存方式: 追記専用
// トランザクション: ACID準拠
// バージョン管理: ストリーム単位

use crate::error::{InfrastructureError, InfrastructureResult};
use lmdb::{Cursor, Database, DatabaseFlags, Environment, Transaction, WriteFlags};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct StoredEvent {
    event_type: String,
    aggregate_id: String,
    version: u64,
    timestamp: String,
    payload: Vec<u8>,
}

pub struct LmdbEventStore {
    env: Arc<Environment>,
    db: Database,
}

impl LmdbEventStore {
    pub async fn new(path: &Path) -> InfrastructureResult<Self> {
        // ディレクトリが存在しない場合は作成
        if !path.exists() {
            tokio::fs::create_dir_all(path).await.map_err(|e| {
                InfrastructureError::EventStoreInitFailed {
                    path: path.display().to_string(),
                    source: e,
                }
            })?;
        }

        // LMDB環境の初期化
        let env = Environment::new()
            .set_max_dbs(1)
            .set_map_size(100 * 1024 * 1024) // 100MB
            .open(path)
            .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        let db = env
            .create_db(Some("events"), DatabaseFlags::empty())
            .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        Ok(Self {
            env: Arc::new(env),
            db,
        })
    }

    /// イベントを追記（async）
    pub async fn append_event(
        &self,
        event_type: &str,
        aggregate_id: &str,
        version: u64,
        payload: &[u8],
    ) -> InfrastructureResult<()> {
        let stored_event = StoredEvent {
            event_type: event_type.to_string(),
            aggregate_id: aggregate_id.to_string(),
            version,
            timestamp: chrono::Utc::now().to_rfc3339(),
            payload: payload.to_vec(),
        };

        let key = format!("{}:{}", aggregate_id, version);
        let value = serde_json::to_vec(&stored_event)
            .map_err(|e| InfrastructureError::SerializationFailed(e.to_string()))?;

        let env = Arc::clone(&self.env);
        let db = self.db;

        tokio::task::spawn_blocking(move || {
            let mut txn = env
                .begin_rw_txn()
                .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;
            txn.put(db, &key, &value, WriteFlags::empty())
                .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;
            txn.commit()
                .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;
            Ok::<_, InfrastructureError>(())
        })
        .await
        .map_err(|e| InfrastructureError::LmdbError(e.to_string()))??;

        Ok(())
    }

    /// イベントストリームを読み込み（async）
    pub async fn load_event_stream(
        &self,
        aggregate_id: &str,
    ) -> InfrastructureResult<Vec<StoredEvent>> {
        let env = Arc::clone(&self.env);
        let db = self.db;
        let prefix = format!("{}:", aggregate_id);

        let result: Result<Vec<StoredEvent>, InfrastructureError> =
            tokio::task::spawn_blocking(move || {
                let txn = env
                    .begin_ro_txn()
                    .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;
                let mut cursor = txn
                    .open_ro_cursor(db)
                    .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

                let mut events = Vec::new();
                for (key, value) in cursor.iter() {
                    let key_str = std::str::from_utf8(key)
                        .map_err(|e| InfrastructureError::DeserializationFailed(e.to_string()))?;

                    if key_str.starts_with(&prefix) {
                        let event: StoredEvent = serde_json::from_slice(value).map_err(|e| {
                            InfrastructureError::DeserializationFailed(e.to_string())
                        })?;
                        events.push(event);
                    }
                }

                Ok::<Vec<StoredEvent>, InfrastructureError>(events)
            })
            .await
            .map_err(|e| InfrastructureError::LmdbError(e.to_string()))?;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_event_store_initialization() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let event_store_path = temp_dir.path().join("events");

        let result = LmdbEventStore::new(&event_store_path).await;
        assert!(result.is_ok(), "EventStore should initialize successfully");
    }

    #[tokio::test]
    async fn test_event_store_append() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let event_store_path = temp_dir.path().join("events");

        let store = LmdbEventStore::new(&event_store_path).await.unwrap();
        let event_data = b"test event data";

        let result = store
            .append_event("TestEvent", "aggregate-001", 1, event_data)
            .await;
        assert!(result.is_ok(), "Event append should succeed");
    }

    #[tokio::test]
    async fn test_event_store_load_stream() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let event_store_path = temp_dir.path().join("events");

        let store = LmdbEventStore::new(&event_store_path).await.unwrap();

        // イベントを追加
        store
            .append_event("TestEvent", "aggregate-001", 1, b"event 1")
            .await
            .unwrap();
        store
            .append_event("TestEvent", "aggregate-001", 2, b"event 2")
            .await
            .unwrap();

        // ストリームをロード
        let result = store.load_event_stream("aggregate-001").await;
        assert!(result.is_ok(), "Event stream load should succeed");

        let events = result.unwrap();
        assert_eq!(events.len(), 2, "Should have 2 events");
        assert_eq!(events[0].version, 1);
        assert_eq!(events[1].version, 2);
    }

    #[tokio::test]
    async fn test_event_store_multiple_aggregates() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let event_store_path = temp_dir.path().join("events");

        let store = LmdbEventStore::new(&event_store_path).await.unwrap();

        // 異なる集約にイベントを追加
        store
            .append_event("TestEvent", "aggregate-001", 1, b"event 1")
            .await
            .unwrap();
        store
            .append_event("TestEvent", "aggregate-002", 1, b"event 2")
            .await
            .unwrap();

        // 各集約のストリームをロード
        let events1 = store.load_event_stream("aggregate-001").await.unwrap();
        let events2 = store.load_event_stream("aggregate-002").await.unwrap();

        assert_eq!(events1.len(), 1);
        assert_eq!(events2.len(), 1);
    }
}
