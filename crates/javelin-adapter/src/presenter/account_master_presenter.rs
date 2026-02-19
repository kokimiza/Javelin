// AccountMasterPresenter - 勘定科目マスタPresenter
// 責務: 勘定科目マスタデータをビュー用に整形

use javelin_application::{dtos::LoadAccountMasterResponse, output_port::AccountMasterOutputPort};
use tokio::sync::mpsc;

/// 勘定科目マスタのビューモデル
#[derive(Debug, Clone)]
pub struct AccountMasterViewModel {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

/// 勘定科目マスタPresenter
pub struct AccountMasterPresenter {
    /// データ更新通知チャネル
    sender: mpsc::UnboundedSender<AccountMasterViewModel>,
}

impl AccountMasterPresenter {
    pub fn new(sender: mpsc::UnboundedSender<AccountMasterViewModel>) -> Self {
        Self { sender }
    }

    /// チャネルを作成
    pub fn create_channel() -> (
        mpsc::UnboundedSender<AccountMasterViewModel>,
        mpsc::UnboundedReceiver<AccountMasterViewModel>,
    ) {
        mpsc::unbounded_channel()
    }
}

#[allow(async_fn_in_trait)]
impl AccountMasterOutputPort for AccountMasterPresenter {
    async fn present_account_master(&self, response: &LoadAccountMasterResponse) {
        // ヘッダー
        let headers = vec!["コード".to_string(), "科目名".to_string(), "区分".to_string()];

        // データ行
        let rows: Vec<Vec<String>> = response
            .accounts
            .iter()
            .map(|acc| {
                vec![
                    acc.code.clone(),
                    acc.name.clone(),
                    Self::format_account_type(&acc.account_type),
                ]
            })
            .collect();

        let view_model = AccountMasterViewModel { headers, rows };

        // チャネル経由で通知（失敗しても無視）
        let _ = self.sender.send(view_model);
    }
}

impl AccountMasterPresenter {
    /// 科目タイプを日本語に変換
    fn format_account_type(account_type: &str) -> String {
        match account_type {
            "Asset" => "資産".to_string(),
            "Liability" => "負債".to_string(),
            "Equity" => "純資産".to_string(),
            "Revenue" => "収益".to_string(),
            "Expense" => "費用".to_string(),
            _ => account_type.to_string(),
        }
    }
}
