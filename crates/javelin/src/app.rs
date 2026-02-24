// Application - アプリケーション本体
// 責務: ナビゲーションループの実行

use std::sync::Arc;

use javelin_adapter::{
    HomePageState, NavigationStack, PresenterRegistry, navigation::Controllers,
    views::terminal_manager::TerminalManager,
};
use javelin_infrastructure::{
    event_store::EventStore, projection_builder_impl::ProjectionBuilderImpl,
    projection_db::ProjectionDb, queries::MasterDataLoaderImpl,
};
use tokio::sync::mpsc;

use crate::{app_error::AppResult, app_resolver::PageStateResolver};

/// アプリケーション全体の構成
pub struct Application {
    nav_stack: NavigationStack,
    controllers: Arc<Controllers>,
    _presenter_registry: Arc<PresenterRegistry>,
    terminal_manager: TerminalManager,
    resolver: PageStateResolver,
    // Infrastructure層（保持のみ）
    _closing_page: javelin_adapter::views::pages::ClosingPage,
    _projection_db: Arc<ProjectionDb>,
    _event_store: Arc<EventStore>,
    _projection_builder: Arc<ProjectionBuilderImpl>,
    _master_data_loader: Arc<MasterDataLoaderImpl>,
    // イベント通知用（保持のみ）
    _event_sender: mpsc::UnboundedSender<javelin_application::output_port::EventNotification>,
    _event_receiver: mpsc::UnboundedReceiver<javelin_application::output_port::EventNotification>,
    // インフラエラー通知用
    infra_error_receiver: mpsc::UnboundedReceiver<String>,
}

impl Application {
    /// 新しいApplicationを作成
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        controllers: Controllers,
        presenter_registry: Arc<PresenterRegistry>,
        terminal_manager: TerminalManager,
        closing_page: javelin_adapter::views::pages::ClosingPage,
        projection_db: Arc<ProjectionDb>,
        event_store: Arc<EventStore>,
        projection_builder: Arc<ProjectionBuilderImpl>,
        master_data_loader: Arc<MasterDataLoaderImpl>,
        event_sender: mpsc::UnboundedSender<javelin_application::output_port::EventNotification>,
        event_receiver: mpsc::UnboundedReceiver<
            javelin_application::output_port::EventNotification,
        >,
        infra_error_receiver: mpsc::UnboundedReceiver<String>,
    ) -> Self {
        let controllers_arc = Arc::new(controllers);
        let resolver =
            PageStateResolver::new(Arc::clone(&presenter_registry), Arc::clone(&controllers_arc));

        Self {
            nav_stack: NavigationStack::new(),
            controllers: controllers_arc,
            _presenter_registry: presenter_registry,
            terminal_manager,
            resolver,
            _closing_page: closing_page,
            _projection_db: projection_db,
            _event_store: event_store,
            _projection_builder: projection_builder,
            _master_data_loader: master_data_loader,
            _event_sender: event_sender,
            _event_receiver: event_receiver,
            infra_error_receiver,
        }
    }

    /// アプリケーションを実行
    pub fn run(mut self) -> AppResult<()> {
        println!("\n◆ アプリケーション起動 ◆");
        println!("  Navigation: Stack-based architecture");
        println!("  Controllers: 準備完了");
        println!("  PresenterRegistry: 準備完了");
        println!("\n✓ すべてのコンポーネントが正常に初期化されました");
        println!("  メインメニューを起動します...\n");

        // Push home page as initial screen
        self.nav_stack.push(Box::new(HomePageState::new()));

        // Main navigation loop
        loop {
            // インフラエラーをポーリングしてイベントログに表示
            while let Ok(error_message) = self.infra_error_receiver.try_recv() {
                if let Some(page) = self.nav_stack.current() {
                    page.on_navigation_error(&error_message);
                }
            }

            // Get current page
            let current_page = match self.nav_stack.current() {
                Some(page) => page,
                None => break, // Exit when stack is empty
            };

            // Run page event loop
            let nav_action =
                match current_page.run(self.terminal_manager.terminal_mut(), &self.controllers) {
                    Ok(action) => action,
                    Err(e) => {
                        let error_message = format!("Page error: {}", e);
                        current_page.on_navigation_error(&error_message);
                        javelin_adapter::NavAction::Back
                    }
                };

            // Handle navigation action
            match nav_action {
                javelin_adapter::NavAction::Go(route) => {
                    match self.resolver.resolve(route.clone()) {
                        Ok(new_page) => {
                            self.nav_stack.push(new_page);
                        }
                        Err(e) => {
                            let error_message = format!("Navigation error: {:?} - {}", route, e);
                            if let Some(page) = self.nav_stack.current() {
                                page.on_navigation_error(&error_message);
                            }
                        }
                    }
                }
                javelin_adapter::NavAction::Back => {
                    self.nav_stack.pop();
                }
                javelin_adapter::NavAction::None => {
                    // Continue on current page
                }
            }
        }

        println!("\n◆ アプリケーション終了 ◆");
        println!("  すべてのコンポーネントを正常にシャットダウンしました");

        Ok(())
    }
}
