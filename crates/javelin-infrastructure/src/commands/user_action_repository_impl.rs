// UserActionRepositoryImpl - ユーザ操作記録リポジトリ実装
// 責務: LMDBへのユーザ操作記録の保存

use javelin_domain::repositories::UserActionRepository;
use lmdb::{Database, DatabaseFlags, Environment, Transaction, WriteFlags};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct UserAction {
    id: String,
    timestamp: String,
    user: String,
    location: String,
    action: String,
}

pub struct UserActionRepositoryImpl {
    env: Arc<Environment>,
    db: Database,
}

impl UserActionRepositoryImpl {
    pub async fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // ディレクトリが存在しない場合は作成
        if !path.exists() {
            tokio::fs::create_dir_all(path).await?;
        }

        // LMDB環境を作成
        let env = Environment::new()
            .set_max_dbs(1)
            .set_map_size(10 * 1024 * 1024) // 10MB
            .open(path)?;

        // データベース作成
        let db = env.create_db(Some("user_actions"), DatabaseFlags::empty())?;

        Ok(Self {
            env: Arc::new(env),
            db,
        })
    }
}

impl UserActionRepository for UserActionRepositoryImpl {
    fn save_action(
        &self,
        user: &str,
        location: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<String, Box<dyn std::error::Error + Send + Sync>>> + Send
    {
        let action_id = Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().to_rfc3339();
        let user = user.to_string();
        let location = location.to_string();
        let action = action.to_string();

        let user_action = UserAction {
            id: action_id.clone(),
            timestamp,
            user,
            location,
            action,
        };

        let env = Arc::clone(&self.env);
        let db = self.db;
        let action_id_clone = action_id.clone();

        async move {
            // JSONシリアライズ
            let value = serde_json::to_vec(&user_action)?;

            // LMDBへの書き込み（ブロッキング操作なのでspawn_blockingで実行）
            tokio::task::spawn_blocking(move || {
                let mut txn = env.begin_rw_txn()?;
                txn.put(db, &action_id_clone, &value, WriteFlags::empty())?;
                txn.commit()?;
                Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
            })
            .await??;

            Ok(action_id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use javelin_domain::repositories::UserActionRepository;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_save_action() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let repo = UserActionRepositoryImpl::new(temp_dir.path())
            .await
            .unwrap();

        let result = repo
            .save_action("test_user", "HomePage", "test action")
            .await;

        assert!(result.is_ok());
        let action_id = result.unwrap();
        assert!(!action_id.is_empty());

        // LMDBデータベースが作成されたことを確認
        let db_files = std::fs::read_dir(temp_dir.path()).unwrap();
        let file_count = db_files.count();
        assert!(file_count > 0, "LMDB files should be created");
    }

    #[tokio::test]
    async fn test_multiple_saves() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let repo = UserActionRepositoryImpl::new(temp_dir.path())
            .await
            .unwrap();

        // 複数のアクションを保存
        let id1 = repo
            .save_action("user1", "HomePage", "action1")
            .await
            .unwrap();
        let id2 = repo
            .save_action("user2", "SettingsPage", "action2")
            .await
            .unwrap();

        assert_ne!(id1, id2, "Action IDs should be unique");
    }
}
