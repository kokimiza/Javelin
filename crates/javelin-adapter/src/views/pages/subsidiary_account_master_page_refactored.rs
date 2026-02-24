// SubsidiaryAccountMasterPage - 補助科目マスタ画面（テンプレート使用版）
// 責務: 補助科目マスタの表示

use ratatui::{Frame, layout::Constraint};

use crate::{
    presenter::SubsidiaryAccountMasterItemViewModel,
    views::layouts::templates::{MasterListItem, MasterListTemplate},
};

/// SubsidiaryAccountMasterItemViewModelをMasterListItemとして実装
impl MasterListItem for SubsidiaryAccountMasterItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["コード", "名称", "親科目"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![Constraint::Length(10), Constraint::Min(20), Constraint::Length(10)]
    }

    fn to_row(&self) -> Vec<String> {
        vec![self.code.clone(), self.name.clone(), self.parent_account_code.clone()]
    }
}

/// 補助科目マスタ画面
pub struct SubsidiaryAccountMasterPage {
    template: MasterListTemplate<SubsidiaryAccountMasterItemViewModel>,
}

impl SubsidiaryAccountMasterPage {
    pub fn new() -> Self {
        Self { template: MasterListTemplate::new("補助科目マスタ") }
    }

    pub fn set_data(
        &mut self,
        accounts: Vec<SubsidiaryAccountMasterItemViewModel>,
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

impl Default for SubsidiaryAccountMasterPage {
    fn default() -> Self {
        Self::new()
    }
}
