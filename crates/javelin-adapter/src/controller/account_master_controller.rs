// AccountMasterController - 勘定科目マスタコントローラ
// 責務: 勘定科目マスタ取得リクエストの受付

use javelin_application::{
    dtos::{LoadAccountMasterRequest, LoadAccountMasterResponse},
    error::ApplicationResult,
    input_ports::LoadAccountMasterInputPort,
};

/// 勘定科目マスタコントローラ
pub struct AccountMasterController<I>
where
    I: LoadAccountMasterInputPort,
{
    interactor: I,
}

impl<I> AccountMasterController<I>
where
    I: LoadAccountMasterInputPort,
{
    pub fn new(interactor: I) -> Self {
        Self { interactor }
    }

    /// 勘定科目マスタを取得
    pub async fn load_account_master(
        &self,
        filter: Option<String>,
        active_only: bool,
    ) -> ApplicationResult<LoadAccountMasterResponse> {
        let request = LoadAccountMasterRequest { filter, active_only };
        self.interactor.execute(request).await
    }
}
