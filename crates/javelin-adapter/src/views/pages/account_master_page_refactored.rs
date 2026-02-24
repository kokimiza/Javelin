// AccountMasterPage - 勘定科目マスタ画面（テンプレート使用版）
// 責務: 勘定科目マスタの表示

use ratatui::{Frame, layout::Constraint};

use crate::{
    presenter::AccountMasterItemViewModel,
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// AccountMasterItemViewModelをMasterListItemとして実装
impl MasterListItem for AccountMasterItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["コード", "名称", "種別"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![Constraint::Length(10), Constraint::Min(20), Constraint::Length(10)]
    }

    fn to_row(&self) -> Vec<String> {
        vec![self.code.clone(), self.name.clone(), self.account_type_label.clone()]
    }
}

/// 勘定科目マスタ画面
pub struct AccountMasterPage {
    template: MasterListTemplate<AccountMasterItemViewModel>,
}

impl AccountMasterPage {
    pub fn new() -> Self {
        Self { template: MasterListTemplate::new("勘定科目マスタ") }
    }

    pub fn set_data(
        &mut self,
        accounts: Vec<AccountMasterItemViewModel>,
        current_page: usize,
        selected_index: usize,
    ) {
        self.template.set_data(accounts, current_page, selected_index);
    }

    pub fn set_loading(&mut self) {
        self.template.set_loading();
    }

    pub fn set_error(&mut self, error: String) {
        self.template.set_error(error);
    }

    pub fn total_items(&self) -> usize {
        self.template.total_items()
    }

    pub fn current_page_items_len(&self) -> usize {
        self.template.current_page_items_len()
    }

    pub fn render(&self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl Default for AccountMasterPage {
    fn default() -> Self {
        Self::new()
    }
}
