// LedgerViewPage - 元帳詳細閲覧画面
// 責務: 総勘定元帳・補助元帳の詳細閲覧（レトロで哀愁漂うデザイン）

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};
use tokio::sync::mpsc;

use crate::{
    format_amount, format_balance,
    presenter::LedgerViewModel,
    truncate_text,
    views::{
        components::{DataTable, InfoPanel},
        layouts::ViewLayout,
    },
};

pub struct LedgerViewPage {
    layout: ViewLayout,
    ledger_table: DataTable,
    info_panel: InfoPanel,
    /// ViewModelレシーバー
    ledger_receiver: mpsc::UnboundedReceiver<LedgerViewModel>,
}

impl LedgerViewPage {
    pub fn new(ledger_receiver: mpsc::UnboundedReceiver<LedgerViewModel>) -> Self {
        let layout = ViewLayout::new("総勘定元帳", "R-401", "2024年12月", 3);

        // レトロなヘッダー
        let headers = vec![
            "日付".to_string(),
            "証憑No".to_string(),
            "摘要".to_string(),
            "借方".to_string(),
            "貸方".to_string(),
            "残高".to_string(),
        ];

        let mut ledger_table = DataTable::new("勘定科目: 現金", headers)
            .with_column_widths(vec![12, 10, 30, 15, 15, 15]);

        // 最初の行を選択
        ledger_table.select_next();

        let mut info_panel = InfoPanel::new("勘定情報");
        info_panel.add_line("科目コード", "1001");
        info_panel.add_line("科目名", "現金");
        info_panel.add_line("期首残高", "0");
        info_panel.add_line("当期借方", "900,000");
        info_panel.add_line("当期貸方", "1,000,000");
        info_panel.add_line("期末残高", "100,000");

        Self { layout, ledger_table, info_panel, ledger_receiver }
    }

    /// ViewModelを受信してテーブルを更新
    pub fn update(&mut self) {
        if let Ok(view_model) = self.ledger_receiver.try_recv() {
            // テーブルデータを構築
            let rows: Vec<Vec<String>> = view_model
                .entries
                .iter()
                .map(|entry| {
                    vec![
                        entry.transaction_date.clone(),
                        entry.entry_number.clone(),
                        truncate_text!(&entry.description, 28),
                        format_amount!(entry.debit_amount),
                        format_amount!(entry.credit_amount),
                        format_balance!(entry.balance),
                    ]
                })
                .collect();

            self.ledger_table.set_data(rows);

            // 情報パネルを更新
            self.update_info_panel(&view_model);
        }
    }

    /// 情報パネルを更新
    fn update_info_panel(&mut self, ledger: &LedgerViewModel) {
        self.info_panel.clear();
        self.info_panel.add_line("科目コード", &ledger.account_code);
        self.info_panel.add_line("科目名", &ledger.account_name);
        self.info_panel.add_line("期首残高", &format_balance!(ledger.opening_balance));
        self.info_panel.add_line("当期借方", &format_amount!(ledger.total_debit));
        self.info_panel.add_line("当期貸方", &format_amount!(ledger.total_credit));
        self.info_panel.add_line("期末残高", &format_balance!(ledger.closing_balance));
    }

    /// 次の行を選択
    pub fn select_next(&mut self) {
        self.ledger_table.select_next();
    }

    /// 前の行を選択
    pub fn select_previous(&mut self) {
        self.ledger_table.select_previous();
    }

    /// 描画
    pub fn render(&mut self, frame: &mut Frame) {
        let ledger_table = &mut self.ledger_table;
        let info_panel = &self.info_panel;

        self.layout.render(frame, |frame, area| {
            // 左右分割: テーブル + 情報パネル
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(area);

            ledger_table.render(frame, chunks[0]);
            info_panel.render(frame, chunks[1]);
        });
    }
}

impl Default for LedgerViewPage {
    fn default() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        drop(tx);
        Self::new(rx)
    }
}
