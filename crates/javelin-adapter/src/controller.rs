// Controller - 外部入力受付
// 責務: DTO変換、InputPort呼び出し
// 禁止: 業務判断

use javelin_application::dtos::{RecordUserActionRequest, RecordUserActionResponse};
use javelin_application::input_ports::RecordUserActionUseCase;
use std::sync::Arc;

/// ユーザ操作記録コントローラ
pub struct RecordUserActionController {
    use_case: Arc<dyn RecordUserActionUseCase>,
}

impl RecordUserActionController {
    pub fn new(use_case: Arc<dyn RecordUserActionUseCase>) -> Self {
        Self { use_case }
    }

    /// ユーザ操作を記録
    pub async fn record_action(
        &self,
        user: impl Into<String>,
        location: impl Into<String>,
        action: impl Into<String>,
    ) -> Result<RecordUserActionResponse, String> {
        let request = RecordUserActionRequest {
            user: user.into(),
            location: location.into(),
            action: action.into(),
        };

        self.use_case
            .execute(request)
            .await
            .map_err(|e| e.to_string())
    }
}
