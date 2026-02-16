// MasterDataLoader - マスタデータとユーザ設定のロード
// 責務: アプリケーション起動時の初期データロード
// 禁止: Repository利用（Projectionのみ）

use crate::error::ApplicationResult;
use serde::{Deserialize, Serialize};

/// マスタデータとユーザ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterData {
    /// 勘定科目マスタ
    pub accounts: Vec<AccountMaster>,
    /// 会社マスタ
    pub companies: Vec<CompanyMaster>,
    /// ユーザ設定
    pub user_options: UserOptions,
}

/// 勘定科目マスタ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountMaster {
    pub code: String,
    pub name: String,
    pub account_type: AccountType,
    pub is_active: bool,
}

/// 勘定科目タイプ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

/// 会社マスタ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyMaster {
    pub code: String,
    pub name: String,
    pub is_active: bool,
}

/// ユーザ設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOptions {
    /// デフォルト会社コード
    pub default_company_code: Option<String>,
    /// 表示言語
    pub language: String,
    /// 小数点以下桁数
    pub decimal_places: u8,
    /// 日付フォーマット
    pub date_format: String,
}

impl Default for UserOptions {
    fn default() -> Self {
        Self {
            default_company_code: None,
            language: "ja".to_string(),
            decimal_places: 2,
            date_format: "YYYY-MM-DD".to_string(),
        }
    }
}

/// マスタデータローダークエリ
#[derive(Debug, Clone)]
pub struct LoadMasterDataQuery;

/// マスタデータローダーサービス
#[allow(async_fn_in_trait)]
pub trait MasterDataLoaderService: Send + Sync {
    /// マスタデータをロード
    async fn load_master_data(&self) -> ApplicationResult<MasterData>;
}
