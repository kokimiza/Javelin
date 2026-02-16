// ViewRouter - ビュー間のナビゲーション管理
// 責務: ビュー切り替え、状態遷移

/// ビューの種類
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewType {
    Home,
    JournalEntry,        // 仕訳入力
    LedgerConsolidation, // 元帳集約
    ClosingPreparation,  // 締準備
    TrialBalance,        // 試算表
    FinancialStatement,  // 財務諸表
}

/// ビュールーター
pub struct ViewRouter {
    current_view: ViewType,
}

impl ViewRouter {
    pub fn new() -> Self {
        Self {
            current_view: ViewType::Home,
        }
    }

    /// 現在のビューを取得
    pub fn current_view(&self) -> &ViewType {
        &self.current_view
    }

    /// ビューを切り替え
    pub fn navigate_to(&mut self, view: ViewType) {
        self.current_view = view;
    }

    /// ホームに戻る
    pub fn navigate_home(&mut self) {
        self.current_view = ViewType::Home;
    }
}

impl Default for ViewRouter {
    fn default() -> Self {
        Self::new()
    }
}
