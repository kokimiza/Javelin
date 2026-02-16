// ClosingPage - 決算処理画面
// 責務: 決算処理の実行と進捗表示

use crate::views::layouts::MainLayout;

pub struct ClosingPage {
    layout: MainLayout,
}

impl Default for ClosingPage {
    fn default() -> Self {
        Self::new()
    }
}

impl ClosingPage {
    pub fn new() -> Self {
        Self {
            layout: MainLayout::new("月次決算処理"),
        }
    }

    // TODO: 実装
}
