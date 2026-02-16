// JournalEntryPage - 仕訳入力画面
// 責務: 仕訳データの入力UI

use crate::views::layouts::MainLayout;

pub struct JournalEntryPage {
    layout: MainLayout,
}

impl Default for JournalEntryPage {
    fn default() -> Self {
        Self::new()
    }
}

impl JournalEntryPage {
    pub fn new() -> Self {
        Self {
            layout: MainLayout::new("仕訳入力"),
        }
    }

    // TODO: 実装
}
