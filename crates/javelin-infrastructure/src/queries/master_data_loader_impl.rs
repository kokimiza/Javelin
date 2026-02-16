// MasterDataLoaderの実装
// 責務: LMDBからマスタデータをロード

use javelin_application::error::ApplicationResult;
use javelin_application::query_service::{
    AccountMaster, AccountType, CompanyMaster, MasterData, MasterDataLoaderService, UserOptions,
};
use lmdb::{Cursor, Database, DatabaseFlags, Environment, Transaction};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct StoredMasterData {
    accounts: Vec<AccountMaster>,
    companies: Vec<CompanyMaster>,
    user_options: UserOptions,
}

/// マスタデータローダーの実装
pub struct MasterDataLoaderImpl {
    env: Arc<Environment>,
    db: Database,
}

impl MasterDataLoaderImpl {
    pub async fn new(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // ディレクトリが存在しない場合は作成
        if !path.exists() {
            tokio::fs::create_dir_all(path).await?;
        }

        // LMDB環境を作成
        let env = Environment::new()
            .set_max_dbs(1)
            .set_map_size(50 * 1024 * 1024) // 50MB
            .open(path)?;

        // データベース作成
        let db = env.create_db(Some("master_data"), DatabaseFlags::empty())?;

        let loader = Self {
            env: Arc::new(env),
            db,
        };

        // 初回起動時にデフォルトデータを投入
        loader.initialize_if_empty().await?;

        Ok(loader)
    }

    /// デフォルトのマスタデータを生成
    fn create_default_master_data() -> MasterData {
        MasterData {
            accounts: vec![
                AccountMaster {
                    code: "1000".to_string(),
                    name: "現金".to_string(),
                    account_type: AccountType::Asset,
                    is_active: true,
                },
                AccountMaster {
                    code: "1100".to_string(),
                    name: "普通預金".to_string(),
                    account_type: AccountType::Asset,
                    is_active: true,
                },
                AccountMaster {
                    code: "2000".to_string(),
                    name: "買掛金".to_string(),
                    account_type: AccountType::Liability,
                    is_active: true,
                },
                AccountMaster {
                    code: "3000".to_string(),
                    name: "資本金".to_string(),
                    account_type: AccountType::Equity,
                    is_active: true,
                },
                AccountMaster {
                    code: "4000".to_string(),
                    name: "売上高".to_string(),
                    account_type: AccountType::Revenue,
                    is_active: true,
                },
                AccountMaster {
                    code: "5000".to_string(),
                    name: "売上原価".to_string(),
                    account_type: AccountType::Expense,
                    is_active: true,
                },
            ],
            companies: vec![
                CompanyMaster {
                    code: "0001".to_string(),
                    name: "本社".to_string(),
                    is_active: true,
                },
                CompanyMaster {
                    code: "0002".to_string(),
                    name: "支社A".to_string(),
                    is_active: true,
                },
            ],
            user_options: UserOptions::default(),
        }
    }

    /// データベースが空の場合、デフォルトデータを投入
    async fn initialize_if_empty(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let env = Arc::clone(&self.env);
        let db = self.db;

        let is_empty = tokio::task::spawn_blocking(move || {
            let txn = env.begin_ro_txn()?;
            let mut cursor = txn.open_ro_cursor(db)?;
            let is_empty = cursor.iter().next().is_none();
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(is_empty)
        })
        .await??;

        if is_empty {
            let default_data = Self::create_default_master_data();
            self.save_master_data(&default_data).await?;
        }

        Ok(())
    }

    /// マスタデータをLMDBに保存
    async fn save_master_data(
        &self,
        data: &MasterData,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let stored_data = StoredMasterData {
            accounts: data.accounts.clone(),
            companies: data.companies.clone(),
            user_options: data.user_options.clone(),
        };

        let value = serde_json::to_vec(&stored_data)?;
        let env = Arc::clone(&self.env);
        let db = self.db;

        tokio::task::spawn_blocking(move || {
            let mut txn = env.begin_rw_txn()?;
            txn.put(db, &"master_data", &value, lmdb::WriteFlags::empty())?;
            txn.commit()?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
        })
        .await??;

        Ok(())
    }
}

impl MasterDataLoaderService for MasterDataLoaderImpl {
    async fn load_master_data(&self) -> ApplicationResult<MasterData> {
        let env = Arc::clone(&self.env);
        let db = self.db;

        let result = tokio::task::spawn_blocking(move || {
            let txn = env.begin_ro_txn()?;
            let value = txn.get(db, &"master_data")?;
            let stored: StoredMasterData = serde_json::from_slice(value)?;

            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(MasterData {
                accounts: stored.accounts,
                companies: stored.companies,
                user_options: stored.user_options,
            })
        })
        .await
        .map_err(|e| {
            javelin_application::error::ApplicationError::QueryExecutionFailed(e.to_string())
        })?
        .map_err(|e| {
            javelin_application::error::ApplicationError::QueryExecutionFailed(e.to_string())
        })?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_load_master_data() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let master_db_path = temp_dir.path().join("master_data");

        let loader = MasterDataLoaderImpl::new(&master_db_path).await.unwrap();

        let result = loader.load_master_data().await;
        assert!(result.is_ok(), "Master data loading should succeed");

        let master_data = result.unwrap();
        assert!(!master_data.accounts.is_empty(), "Should have accounts");
        assert!(!master_data.companies.is_empty(), "Should have companies");
        assert_eq!(
            master_data.user_options.language, "ja",
            "Default language should be ja"
        );
    }

    #[tokio::test]
    async fn test_default_master_data_structure() {
        let master_data = MasterDataLoaderImpl::create_default_master_data();

        // 勘定科目の検証
        assert_eq!(master_data.accounts.len(), 6);
        assert!(
            master_data
                .accounts
                .iter()
                .any(|a| a.code == "1000" && a.name == "現金")
        );

        // 会社マスタの検証
        assert_eq!(master_data.companies.len(), 2);
        assert!(
            master_data
                .companies
                .iter()
                .any(|c| c.code == "0001" && c.name == "本社")
        );

        // ユーザ設定の検証
        assert_eq!(master_data.user_options.decimal_places, 2);
        assert_eq!(master_data.user_options.date_format, "YYYY-MM-DD");
    }

    #[tokio::test]
    async fn test_persistence() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let master_db_path = temp_dir.path().join("master_data");

        // 最初のローダーでデータを保存
        {
            let loader = MasterDataLoaderImpl::new(&master_db_path).await.unwrap();
            let data = loader.load_master_data().await.unwrap();
            assert_eq!(data.accounts.len(), 6);
        }

        // 新しいローダーで同じデータを読み込めることを確認
        {
            let loader = MasterDataLoaderImpl::new(&master_db_path).await.unwrap();
            let data = loader.load_master_data().await.unwrap();
            assert_eq!(data.accounts.len(), 6);
            assert_eq!(data.companies.len(), 2);
        }
    }
}
