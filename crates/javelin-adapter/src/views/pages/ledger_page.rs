// LedgerPage - 元帳画面
// 責務: 元帳データの表示

use crate::views::layouts::MainLayout;

pub struct LedgerPage {
    layout: MainLayout,
}

impl Default for LedgerPage {
    fn default() -> Self {
        Self::new()
    }
}

impl LedgerPage {
    pub fn new() -> Self {
        Self {
            layout: MainLayout::new("元帳"),
        }
    }

    // TODO: 実装
}
