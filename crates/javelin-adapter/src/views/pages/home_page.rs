// HomePage - ホーム画面（業務メニュー + システムマスタメニュー）
// 責務: 業務メニューとシステムマスタメニューの表示、h/lで枠切り替え、j/kで内部フォーカス移動

use ratatui::Frame;

use crate::views::{
    components::{ListItemData, ListSelector},
    layouts::MenuLayout,
};

/// ViewType enum (temporary, for compatibility)
///
/// This enum is kept temporarily to maintain compatibility.
/// In the future, HomePage should be refactored to return Route directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewType {
    Home,
    JournalEntry,
    Search,
    Ledger,
    LedgerConsolidation,
    ClosingPreparation,
    ClosingLock,
    TrialBalance,
    NoteDraft,
    AccountAdjustment,
    IfrsValuation,
    FinancialStatement,
    AccountMasterManagement,
    SubsidiaryAccountMasterManagement,
    UserSettingsManagement,
    DataImport,
    DataExport,
}

/// メニュータイプ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuType {
    Business, // 業務メニュー
    System,   // システムマスタメニュー
}

pub struct HomePage {
    layout: MenuLayout,
    business_menu_selector: ListSelector,
    system_menu_selector: ListSelector,
    active_menu: MenuType,
}

impl HomePage {
    pub fn new() -> Self {
        let mut layout = MenuLayout::new("財務会計システム JAVELIN", "主計部", "担当者");
        layout.event_viewer_mut().add_info("システム起動完了");
        layout.event_viewer_mut().add_info("バージョン: 1.0.0");

        let business_menu_items = vec![
            ListItemData::new("101", "原始記録登録", "日次：仕訳帳・キャッシュログ入力"),
            ListItemData::new("102", "仕訳検索", "日次：仕訳の検索・照会"),
            ListItemData::new("201", "元帳集約", "週次：総勘定元帳への転記処理"),
            ListItemData::new("301", "締準備", "月次：期間帰属確認・仮仕訳作成"),
            ListItemData::new("302", "締日固定", "月次：取引データのロック処理"),
            ListItemData::new("303", "試算表生成", "月次：残高検証・異常値抽出"),
            ListItemData::new("304", "注記草案生成", "月次：開示情報の整理"),
            ListItemData::new("305", "勘定補正", "月次：仮勘定整理・区分修正"),
            ListItemData::new("306", "IFRS評価", "月次：見積会計・公正価値測定"),
            ListItemData::new("307", "財務諸表生成", "月次：制度開示資料作成"),
            ListItemData::new("401", "元帳閲覧", "照会：総勘定元帳・補助元帳"),
        ];

        let system_menu_items = vec![
            ListItemData::new("901", "勘定科目マスタ", "勘定科目の登録・編集・削除"),
            ListItemData::new("902", "補助科目マスタ", "補助科目の登録・編集・削除"),
            ListItemData::new(
                "903",
                "設定マスタ",
                "言語・表示形式・会計年度・締日・バックアップ設定",
            ),
            ListItemData::new("904", "データインポート", "外部データの一括取込"),
            ListItemData::new("905", "データエクスポート", "マスタデータの出力"),
        ];

        let business_menu_selector = ListSelector::new("業務メニュー", business_menu_items);
        let system_menu_selector = ListSelector::new("システムマスタ", system_menu_items);

        Self {
            layout,
            business_menu_selector,
            system_menu_selector,
            active_menu: MenuType::Business,
        }
    }

    /// メニュー枠を切り替え（h/l）
    pub fn switch_menu(&mut self) {
        self.active_menu = match self.active_menu {
            MenuType::Business => {
                self.layout.event_viewer_mut().add_info("システムマスタメニューに切替");
                MenuType::System
            }
            MenuType::System => {
                self.layout.event_viewer_mut().add_info("業務メニューに切替");
                MenuType::Business
            }
        };
    }

    /// エラーメッセージをイベントログに追加
    pub fn add_error(&mut self, message: &str) {
        self.layout.event_viewer_mut().add_error(message);
    }

    /// 選択を上に移動
    pub fn select_previous(&mut self) {
        match self.active_menu {
            MenuType::Business => self.business_menu_selector.select_previous(),
            MenuType::System => self.system_menu_selector.select_previous(),
        }
    }

    /// 選択を下に移動
    pub fn select_next(&mut self) {
        match self.active_menu {
            MenuType::Business => self.business_menu_selector.select_next(),
            MenuType::System => self.system_menu_selector.select_next(),
        }
    }

    /// 選択された項目に対応するビューを取得
    pub fn get_selected_view(&self) -> Option<ViewType> {
        match self.active_menu {
            MenuType::Business => {
                self.business_menu_selector.selected_index().and_then(|idx| match idx {
                    0 => Some(ViewType::JournalEntry),
                    1 => Some(ViewType::Search),
                    2 => Some(ViewType::LedgerConsolidation),
                    3 => Some(ViewType::ClosingPreparation),
                    4 => Some(ViewType::ClosingLock),
                    5 => Some(ViewType::TrialBalance),
                    6 => Some(ViewType::NoteDraft),
                    7 => Some(ViewType::AccountAdjustment),
                    8 => Some(ViewType::IfrsValuation),
                    9 => Some(ViewType::FinancialStatement),
                    10 => Some(ViewType::Ledger),
                    _ => None,
                })
            }
            MenuType::System => {
                self.system_menu_selector.selected_index().and_then(|idx| match idx {
                    0 => Some(ViewType::AccountMasterManagement),
                    1 => Some(ViewType::SubsidiaryAccountMasterManagement),
                    2 => Some(ViewType::UserSettingsManagement),
                    3 => Some(ViewType::DataImport),
                    4 => Some(ViewType::DataExport),
                    _ => None,
                })
            }
        }
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame) {
        use ratatui::layout::{Constraint, Direction, Layout};

        let active_menu = self.active_menu;
        let business_selector = &mut self.business_menu_selector;
        let system_selector = &mut self.system_menu_selector;

        self.layout.render(frame, |frame, area| {
            // メインエリアを上下分割: 業務メニュー(上) + システムマスタ(下)
            let menu_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(area);

            // 業務メニュー（枠なし、ListSelectorが自分で枠を描画）
            let is_business_active = active_menu == MenuType::Business;
            business_selector.set_active(is_business_active);
            business_selector.render(frame, menu_chunks[0]);

            // システムマスタメニュー（枠なし、ListSelectorが自分で枠を描画）
            let is_system_active = active_menu == MenuType::System;
            system_selector.set_active(is_system_active);
            system_selector.render(frame, menu_chunks[1]);
        });
    }
}

impl Default for HomePage {
    fn default() -> Self {
        Self::new()
    }
}
