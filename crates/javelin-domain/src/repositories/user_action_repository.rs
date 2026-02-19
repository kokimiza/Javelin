// UserActionRepository - ユーザ操作記録リポジトリ

/// ユーザ操作記録リポジトリ
pub trait UserActionRepository: Send + Sync {
    /// ユーザ操作を保存
    fn save_action(
        &self,
        user: &str,
        location: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<String, Box<dyn std::error::Error + Send + Sync>>> + Send;
}
