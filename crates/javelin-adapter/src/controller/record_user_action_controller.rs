// ユーザ操作記録コントローラ

use std::sync::Arc;

use javelin_application::{
    dtos::{RecordUserActionRequest, RecordUserActionResponse},
    input_ports::RecordUserActionUseCase,
};

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

        self.use_case.execute(request).await.map_err(|e| e.to_string())
    }
}
