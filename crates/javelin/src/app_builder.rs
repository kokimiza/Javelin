// ApplicationBuilder - アプリケーションのビルド
// 責務: 各セットアップモジュールを呼び出してApplicationを構築

use std::path::PathBuf;

use crate::{
    app::Application,
    app_error::AppResult,
    app_setup::{setup_controllers, setup_infrastructure},
};

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

    /// アプリケーションをビルド
    pub async fn build(self) -> AppResult<Application> {
        // データディレクトリの決定
        let data_dir = self.data_dir.unwrap_or_else(|| {
            let mut path = std::env::current_dir().expect("Failed to get current directory");
            path.push("data");
            path
        });

        println!("✓ Data directory: {}", data_dir.display());

        // インフラ層のセットアップ
        let infra = setup_infrastructure(&data_dir).await?;

        // コントローラのセットアップ
        let controller_components = setup_controllers(
            &data_dir,
            infra.event_store.clone(),
            infra.master_data_loader.clone(),
        )
        .await?;

        // TerminalManagerの作成
        let terminal_manager = javelin_adapter::views::terminal_manager::TerminalManager::new()
            .map_err(|e| crate::app_error::AppError::InitializationFailed(Box::new(e)))?;

        // Applicationの構築
        Ok(Application::new(
            controller_components.controllers,
            controller_components.presenter_registry,
            terminal_manager,
            controller_components.closing_page,
            infra.projection_db,
            infra.event_store,
            infra.projection_builder,
            infra.master_data_loader,
            controller_components.event_sender,
            controller_components.event_receiver,
            infra.infra_error_receiver,
        ))
    }
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
