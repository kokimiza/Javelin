// RecordUserAction - ユーザ操作記録ユースケース
// 目的: 画面操作をイベントとして記録

use std::{future::Future, pin::Pin};

use crate::{
    dtos::{RecordUserActionRequest, RecordUserActionResponse},
    error::ApplicationResult,
};

/// ユーザ操作記録ユースケース
pub trait RecordUserActionUseCase: Send + Sync {
    fn execute(
        &self,
        request: RecordUserActionRequest,
    ) -> Pin<Box<dyn Future<Output = ApplicationResult<RecordUserActionResponse>> + Send + '_>>;
}
