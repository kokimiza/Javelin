// Application Builder - DIコンテナとアプリケーション構築
// Builderパターンによる依存性注入の管理

use crate::app_error::{AppError, AppResult};
use javelin_adapter::controller::RecordUserActionController;
use javelin_adapter::error_handler::ErrorHandler;
use javelin_adapter::presenter::Presenter;
use javelin_adapter::view_router::ViewRouter;
use javelin_adapter::views::pages::HomePage;
use javelin_application::interactor::RecordUserActionInteractor;
use javelin_application::query_service::{MasterData, MasterDataLoaderService};
use javelin_infrastructure::UserActionRepositoryImpl;
use javelin_infrastructure::event_store::LmdbEventStore;
use javelin_infrastructure::master_data_loader_impl::MasterDataLoaderImpl;
use javelin_infrastructure::projection_db::ProjectionDb;
use std::path::PathBuf;
use std::sync::Arc;

/// アプリケーション全体の構成
pub struct Application {
    view_router: ViewRouter,
    home_view: HomePage,
    // Infrastructure層（共有リソース）
    #[allow(dead_code)]
    event_store: Arc<LmdbEventStore>,
    #[allow(dead_code)]
    projection_db: Arc<ProjectionDb>,
    // マスタデータ（初期化時にロード）
    #[allow(dead_code)]
    master_data: MasterData,
}

impl Application {
    /// アプリケーションを実行
    pub fn run(mut self) -> AppResult<()> {
        // 現在のビューに応じて適切なビューを表示
        let result = match self.view_router.current_view() {
            javelin_adapter::view_router::ViewType::Home => self.home_view.run(),
            view_type => Err(javelin_adapter::error::AdapterError::PageNotImplemented(
                format!("{:?}", view_type),
            )),
        };

        // エラーハンドリングはアダプター層に委譲
        if let Err(e) = result {
            ErrorHandler::handle_and_display(e);
        }

        Ok(())
    }
}

/// アプリケーションビルダー
pub struct ApplicationBuilder {
    data_dir: Option<PathBuf>,
}

impl ApplicationBuilder {
    /// 新規ビルダーを作成
    pub fn new() -> Self {
        Self { data_dir: None }
    }

    /// データディレクトリを設定
    pub fn with_data_dir(mut self, path: PathBuf) -> Self {
        self.data_dir = Some(path);
        self
    }

    /// アプリケーションをビルド（async）
    pub async fn build(self) -> AppResult<Application> {
        // データディレクトリの決定
        let data_dir = self.data_dir.unwrap_or_else(|| {
            let mut path = std::env::current_dir().expect("Failed to get current directory");
            path.push("data");
            path
        });

        // データディレクトリの作成
        if !data_dir.exists() {
            tokio::fs::create_dir_all(&data_dir).await.map_err(|e| {
                AppError::DataDirectoryCreationFailed {
                    path: data_dir.display().to_string(),
                    source: e,
                }
            })?;
        }

        // Infrastructure層の構築（共有リソース）
        let event_store = Arc::new(LmdbEventStore::new(&data_dir.join("events")).await?);
        let projection_db = Arc::new(ProjectionDb::new(&data_dir.join("projections")).await?);

        // ユーザ操作記録リポジトリ
        let user_action_repo = Arc::new(
            UserActionRepositoryImpl::new(&data_dir.join("user_actions"))
                .await
                .map_err(AppError::InitializationFailed)?,
        );

        // マスタデータのロード（初期化時に自動実行）
        let master_data_loader = MasterDataLoaderImpl::new(&data_dir.join("master_data"))
            .await
            .map_err(AppError::InitializationFailed)?;
        let master_data = master_data_loader
            .load_master_data()
            .await
            .map_err(|e| AppError::InitializationFailed(Box::new(e)))?;

        println!("✓ Master data loaded successfully");
        println!("  - Accounts: {}", master_data.accounts.len());
        println!("  - Companies: {}", master_data.companies.len());
        println!("  - Language: {}", master_data.user_options.language);

        // Presenter（イベント通知チャネル）
        let (event_sender, event_receiver) = Presenter::create_channel();
        let presenter = Arc::new(Presenter::new(event_sender));

        // Application層の構築（Interactor）
        let record_user_action_interactor: Arc<
            dyn javelin_application::input_ports::RecordUserActionUseCase,
        > = Arc::new(RecordUserActionInteractor::new(
            user_action_repo,
            Arc::clone(&presenter) as Arc<Presenter>,
        ));

        // Adapter層の構築（Controller）
        let record_user_action_controller = Arc::new(RecordUserActionController::new(
            record_user_action_interactor,
        ));

        // View層の構築
        let view_router = ViewRouter::new();
        let home_view = HomePage::new(record_user_action_controller, event_receiver);

        Ok(Application {
            view_router,
            home_view,
            event_store,
            projection_db,
            master_data,
        })
    }
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    #[serial_test::serial]
    async fn test_builder_default() {
        let builder = ApplicationBuilder::new();
        assert!(builder.data_dir.is_none());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_builder_with_data_dir() {
        let path = PathBuf::from("/tmp/javelin");
        let builder = ApplicationBuilder::new().with_data_dir(path.clone());
        assert_eq!(builder.data_dir, Some(path));
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_builder_build() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path().to_path_buf();

        let app = ApplicationBuilder::new()
            .with_data_dir(temp_path.clone())
            .build()
            .await;

        assert!(app.is_ok(), "Application build should succeed");
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_application_initialization() {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path().to_path_buf();

        let result = ApplicationBuilder::new()
            .with_data_dir(temp_path)
            .build()
            .await;

        assert!(result.is_ok(), "Application should initialize successfully");
    }
}
