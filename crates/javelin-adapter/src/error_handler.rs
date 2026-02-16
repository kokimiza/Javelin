// ErrorHandler - エラーハンドリング
// 責務: エラーの分類と適切な処理

use crate::error::AdapterError;
use color_eyre::{
    Help,
    eyre::{Result, eyre},
    owo_colors::OwoColorize,
};

/// エラーハンドリング結果
pub enum ErrorHandlingResult {
    /// 画面に表示して続行
    DisplayAndContinue(String),
    /// アプリケーションを終了（元のエラーを保持）
    Terminate(AdapterError),
}

/// エラーハンドラー
pub struct ErrorHandler;

impl ErrorHandler {
    /// エラーを処理
    pub fn handle(error: AdapterError) -> ErrorHandlingResult {
        match error {
            // 定義済みのビジネスエラー - 画面表示して続行
            AdapterError::PageNotFound(_)
            | AdapterError::PageNotImplemented(_)
            | AdapterError::InputValidationFailed(_)
            | AdapterError::DtoConversionFailed(_)
            | AdapterError::ApplicationError(_) => {
                ErrorHandlingResult::DisplayAndContinue(format!("{}", error))
            }

            // システムエラー - アプリケーションを終了（元のエラーを保持）
            AdapterError::TerminalInitFailed(_)
            | AdapterError::TerminalCleanupFailed(_)
            | AdapterError::RawModeEnableFailed(_)
            | AdapterError::RawModeDisableFailed(_)
            | AdapterError::RenderingFailed(_)
            | AdapterError::EventPollingFailed(_)
            | AdapterError::EventReadFailed(_)
            | AdapterError::Unknown(_) => ErrorHandlingResult::Terminate(error),
        }
    }

    /// エラーを処理して表示（color-eyre使用）
    ///
    /// ビジネスエラーの場合は何もせず、システムエラーの場合はstd::process::exit(1)を呼ぶ
    pub fn handle_and_display(error: AdapterError) {
        let result = Self::handle(error);

        match result {
            ErrorHandlingResult::DisplayAndContinue(msg) => {
                // ビジネスエラーは警告として表示
                eprintln!("\n{} {}", "⚠".yellow().bold(), msg.yellow());
            }
            ErrorHandlingResult::Terminate(original_error) => {
                // システムエラーはcolor-eyreで詳細表示して終了
                // 元のエラーの詳細情報を含める
                let error_details = match &original_error {
                    AdapterError::TerminalInitFailed(e) => format!("ターミナル初期化失敗: {}", e),
                    AdapterError::TerminalCleanupFailed(e) => {
                        format!("ターミナルクリーンアップ失敗: {}", e)
                    }
                    AdapterError::RawModeEnableFailed(e) => format!("Rawモード有効化失敗: {}", e),
                    AdapterError::RawModeDisableFailed(e) => format!("Rawモード無効化失敗: {}", e),
                    AdapterError::RenderingFailed(msg) => format!("描画失敗: {}", msg),
                    AdapterError::EventPollingFailed(e) => format!("イベントポーリング失敗: {}", e),
                    AdapterError::EventReadFailed(e) => format!("イベント読み込み失敗: {}", e),
                    AdapterError::Unknown(msg) => format!("未定義エラー: {}", msg),
                    _ => format!("{}", original_error),
                };

                let report = eyre!("システムエラーが発生しました")
                    .note("アプリケーションを終了します")
                    .note(format!("エラーコード: {}", original_error))
                    .note(format!("詳細: {}", error_details))
                    .suggestion("ターミナルの設定を確認してください")
                    .suggestion("別のターミナルで実行してみてください")
                    .suggestion("問題が解決しない場合は、ログを確認してください");

                eprintln!("\n{:?}", report);
                std::process::exit(1);
            }
        }
    }

    /// 初期化エラーを処理（color-eyre使用）
    pub fn handle_initialization_error(
        error: impl std::error::Error + Send + Sync + 'static,
    ) -> Result<()> {
        Err(eyre!(error)
            .wrap_err("アプリケーションの初期化に失敗しました")
            .suggestion("データディレクトリの権限を確認してください")
            .suggestion("ディスク容量が十分か確認してください"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_page_not_found() {
        let error = AdapterError::PageNotFound("TestPage".to_string());
        let result = ErrorHandler::handle(error);

        let ErrorHandlingResult::DisplayAndContinue(msg) = result else {
            panic!("Expected DisplayAndContinue");
        };

        assert!(msg.contains("Page not found"));
    }

    #[test]
    fn test_handle_page_not_implemented() {
        let error = AdapterError::PageNotImplemented("TestPage".to_string());
        let result = ErrorHandler::handle(error);

        let ErrorHandlingResult::DisplayAndContinue(msg) = result else {
            panic!("Expected DisplayAndContinue");
        };

        assert!(msg.contains("Page not implemented"));
    }

    #[test]
    fn test_handle_terminal_error() {
        let error = AdapterError::RenderingFailed("test error".to_string());
        let result = ErrorHandler::handle(error);

        let ErrorHandlingResult::Terminate(_) = result else {
            panic!("Expected Terminate");
        };
    }

    #[test]
    fn test_handle_unknown_error() {
        let error = AdapterError::Unknown("unknown error".to_string());
        let result = ErrorHandler::handle(error);

        let ErrorHandlingResult::Terminate(_) = result else {
            panic!("Expected Terminate");
        };
    }
}
