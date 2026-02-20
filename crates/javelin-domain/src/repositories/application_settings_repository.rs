// ApplicationSettingsRepository - アプリケーション設定リポジトリトレイト

use crate::{error::DomainResult, masters::ApplicationSettings};

/// アプリケーション設定リポジトリトレイト
#[allow(async_fn_in_trait)]
pub trait ApplicationSettingsRepository: Send + Sync {
    /// アプリケーション設定を取得
    async fn find(&self) -> DomainResult<Option<ApplicationSettings>>;

    /// アプリケーション設定を保存
    async fn save(&self, settings: &ApplicationSettings) -> DomainResult<()>;
}
