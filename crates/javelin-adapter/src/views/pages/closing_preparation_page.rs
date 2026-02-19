// ClosingPreparationPage - 締準備画面
// 責務: 期間帰属確認・仮仕訳作成

use javelin_application::dtos::PrepareClosingResponse;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::views::components::{DataTable, EventViewer, LoadingSpinner};

#[derive(Debug, Clone, PartialEq)]
enum LoadingState {
    Loading,
    Loaded,
    Error(String),
}

pub struct ClosingPreparationPage {
    preparation_table: DataTable,
    event_viewer: EventViewer,
    loading_spinner: LoadingSpinner,
    loading_state: LoadingState,
    animation_frame: usize,
}

impl ClosingPreparationPage {
    pub fn new() -> Self {
        let headers = vec![
            "項目".to_string(),
            "確認状態".to_string(),
            "期間".to_string(),
            "金額".to_string(),
            "備考".to_string(),
        ];

        let preparation_table = DataTable::new("◆ 締準備 - 期間帰属確認 ◆", headers)
            .with_column_widths(vec![20, 12, 15, 15, 30]);

        let mut event_viewer = EventViewer::new();
        event_viewer.add_info("締準備画面を開きました");
        event_viewer.add_info("データを読み込んでいます...");

        Self {
            preparation_table,
            event_viewer,
            loading_spinner: LoadingSpinner::new(),
            loading_state: LoadingState::Loading,
            animation_frame: 0,
        }
    }

    pub fn set_data(&mut self, data: Vec<Vec<String>>) {
        let count = data.len();
        self.preparation_table.set_data(data);
        self.loading_state = LoadingState::Loaded;
        self.event_viewer.add_info(format!("データ読込完了: {} 件", count));
    }

    pub fn set_response(&mut self, response: PrepareClosingResponse) {
        // レスポンスをテーブルデータに変換
        let mut data = Vec::new();

        // 未登録取引
        data.push(vec![
            "未登録取引".to_string(),
            if response.unregistered_transactions_count == 0 {
                "確認済".to_string()
            } else {
                "要確認".to_string()
            },
            "2024-12".to_string(),
            "-".to_string(),
            format!("{} 件の未登録取引", response.unregistered_transactions_count),
        ]);

        // 銀行照合差異
        for diff in &response.bank_reconciliation_differences {
            data.push(vec![
                format!("銀行照合: {}", diff.bank_account),
                if diff.difference.abs() < 0.01 {
                    "確認済".to_string()
                } else {
                    "差異あり".to_string()
                },
                "2024-12".to_string(),
                format!("{:.0}", diff.difference),
                format!("帳簿: {:.0} / 銀行: {:.0}", diff.cash_log_balance, diff.bank_balance),
            ]);
        }

        // 発生仕訳
        if response.accrual_entries_created > 0 {
            data.push(vec![
                "発生仕訳作成".to_string(),
                "完了".to_string(),
                "2024-12".to_string(),
                "-".to_string(),
                format!("{} 件の仕訳を作成", response.accrual_entries_created),
            ]);
        }

        // 暫定財務諸表
        data.push(vec![
            "暫定財務諸表".to_string(),
            if response.provisional_financial_statements_generated {
                "生成済".to_string()
            } else {
                "未生成".to_string()
            },
            "2024-12".to_string(),
            "-".to_string(),
            "暫定財務諸表の生成状況".to_string(),
        ]);

        self.set_data(data);
    }

    pub fn set_error(&mut self, error: String) {
        self.loading_state = LoadingState::Error(error.clone());
        self.event_viewer.add_error(format!("エラー: {}", error));
    }

    pub fn is_loading(&self) -> bool {
        self.loading_state == LoadingState::Loading
    }

    pub fn tick(&mut self) {
        self.animation_frame = (self.animation_frame + 1) % 60;
        if self.loading_state == LoadingState::Loading {
            self.loading_spinner.tick();
        }
    }

    pub fn add_info(&mut self, message: impl Into<String>) {
        self.event_viewer.add_info(message);
    }

    pub fn add_error(&mut self, message: impl Into<String>) {
        self.event_viewer.add_error(message);
    }

    pub fn select_next(&mut self) {
        self.preparation_table.select_next();
    }

    pub fn select_previous(&mut self) {
        self.preparation_table.select_previous();
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
            .split(area);

        // 左側: テーブル/ローディングとステータスバー
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3)])
            .split(chunks[0]);

        // ローディング中はスピナー表示、それ以外はテーブル表示
        if self.loading_state == LoadingState::Loading {
            self.loading_spinner
                .render(frame, left_chunks[0], "締準備データを読み込んでいます...");
        } else {
            self.preparation_table.render(frame, left_chunks[0]);
        }

        self.render_status_bar(frame, left_chunks[1]);

        // 右側: イベントビューア
        self.event_viewer.render(frame, chunks[1]);
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let cursor = if self.animation_frame < 30 {
            "▮"
        } else {
            " "
        };

        let status_text = vec![Line::from(vec![
            Span::styled(" [↑↓] ", Style::default().fg(Color::DarkGray)),
            Span::styled("選択", Style::default().fg(Color::Gray)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[Enter] ", Style::default().fg(Color::DarkGray)),
            Span::styled("確認", Style::default().fg(Color::Gray)),
            Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
            Span::styled("[Esc] ", Style::default().fg(Color::DarkGray)),
            Span::styled("戻る", Style::default().fg(Color::Gray)),
            Span::styled(
                format!(" {}", cursor),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ])];

        let paragraph = Paragraph::new(status_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        frame.render_widget(paragraph, area);
    }
}

impl Default for ClosingPreparationPage {
    fn default() -> Self {
        Self::new()
    }
}
