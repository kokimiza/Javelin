// OutputPort - 出力抽象
// 責務: Presenter連携

/// イベント通知情報
#[derive(Clone, Debug)]
pub struct EventNotification {
    pub user: String,
    pub location: String,
    pub action: String,
    pub success: bool,
}

impl EventNotification {
    pub fn success(
        user: impl Into<String>,
        location: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            user: user.into(),
            location: location.into(),
            action: action.into(),
            success: true,
        }
    }

    pub fn failure(
        user: impl Into<String>,
        location: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            user: user.into(),
            location: location.into(),
            action: action.into(),
            success: false,
        }
    }
}

/// OutputPort - ユースケース結果の出力
pub trait OutputPort: Send + Sync {
    type Output;

    fn present(&self, output: Self::Output);
}

/// EventOutputPort - イベント通知専用
pub trait EventOutputPort: Send + Sync {
    /// イベントをイベントビューアに通知（非同期）
    fn notify_event(
        &self,
        event: EventNotification,
    ) -> impl std::future::Future<Output = ()> + Send;
}
