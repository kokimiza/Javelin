// ApplicationSettingsPage - アプリケーション設定画面（テンプレート使用版）
// 責務: アプリケーション設定の表示

use ratatui::Frame;

use crate::{
    presenter::ApplicationSettingsViewModel,
    views::layouts::templates::{SettingsItem, SettingsTemplate},
};

/// ApplicationSettingsViewModelをSettingsItemとして実装
impl SettingsItem for ApplicationSettingsViewModel {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            (
                "デフォルト会社コード".to_string(),
                self.default_company_code.as_deref().unwrap_or("未設定").to_string(),
            ),
            ("言語".to_string(), self.language_label.clone()),
            ("小数点以下桁数".to_string(), self.decimal_places.to_string()),
            ("日付フォーマット".to_string(), self.date_format.clone()),
            ("会計年度開始月".to_string(), self.fiscal_year_start_month_label.clone()),
            ("締日".to_string(), format!("{}日", self.closing_day)),
            ("自動バックアップ".to_string(), self.auto_backup_label.clone()),
            ("バックアップ保持日数".to_string(), format!("{}日", self.backup_retention_days)),
        ]
    }
}

/// アプリケーション設定画面
pub struct ApplicationSettingsPage {
    template: SettingsTemplate<ApplicationSettingsViewModel>,
}

impl ApplicationSettingsPage {
    pub fn new() -> Self {
        Self { template: SettingsTemplate::new("アプリケーション設定") }
    }

    pub fn set_data(&mut self, view_model: ApplicationSettingsViewModel) {
        self.template.set_data(view_model);
    }

    pub fn set_loading(&mut self) {
        self.template.set_loading();
    }

    pub fn set_error(&mut self, error: String) {
        self.template.set_error(error);
    }

    pub fn render(&self, frame: &mut Frame) {
        self.template.render(frame);
    }
}

impl Default for ApplicationSettingsPage {
    fn default() -> Self {
        Self::new()
    }
}
