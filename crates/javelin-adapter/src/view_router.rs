// ViewRouter - ビュー間のナビゲーション管理
// 責務: ビュー切り替え、状態遷移

/// ビューの種類
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewType {
    Home,
    // 日次処理
    JournalEntry, // 4.1 原始記録登録処理
    // 週次処理
    LedgerConsolidation, // 4.2 元帳集約処理
    // 月次処理
    ClosingPreparation, // 4.3 締準備処理
    ClosingLock,        // 4.4 締日固定処理
    TrialBalance,       // 4.5 試算表生成処理
    NoteDraft,          // 4.6 注記草案生成処理
    AccountAdjustment,  // 4.7 勘定補正処理
    IfrsValuation,      // 4.8 IFRS評価処理
    FinancialStatement, // 4.9 財務諸表生成処理
    // 閲覧
    Ledger, // 元帳閲覧
    // システムマスタ
    AccountMasterManagement,  // 勘定科目マスタ管理
    CompanyMasterManagement,  // 会社マスタ管理
    UserSettingsManagement,   // ユーザ設定管理
    SystemSettingsManagement, // システム設定管理
    DataBackup,               // データバックアップ
    DataImport,               // データインポート
    DataExport,               // データエクスポート
}

/// ビュールーター
pub struct ViewRouter {
    current_view: ViewType,
    view_history: Vec<ViewType>,
}

impl ViewRouter {
    pub fn new() -> Self {
        Self { current_view: ViewType::Home, view_history: Vec::new() }
    }

    /// 現在のビューを取得
    pub fn current_view(&self) -> &ViewType {
        &self.current_view
    }

    /// ビューを切り替え
    pub fn navigate_to(&mut self, view: ViewType) {
        self.view_history.push(self.current_view.clone());
        self.current_view = view;
    }

    /// ホームに戻る
    pub fn navigate_home(&mut self) {
        self.view_history.clear();
        self.current_view = ViewType::Home;
    }

    /// 前の画面に戻る
    pub fn navigate_back(&mut self) {
        if let Some(prev_view) = self.view_history.pop() {
            self.current_view = prev_view;
        }
    }
}

impl Default for ViewRouter {
    fn default() -> Self {
        Self::new()
    }
}
