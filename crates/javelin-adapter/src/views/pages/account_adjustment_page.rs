// AccountAdjustmentPage - 勘定補正画面
// 責務: 仮勘定整理・区分修正

use javelin_application::dtos::AdjustAccountsResponse;
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

pub struct AccountAdjustmentPage {
    adjustment_table: DataTable,
    event_viewer: EventViewer,
    loading_spinner: LoadingSpinner,
    loading_state: LoadingState,
    animation_frame: usize,
}

impl AccountAdjustmentPage {
    pub fn new() -> Self {
        let headers = vec![
            "補正種別".to_string(),
            "元勘定".to_string(),
            "先勘定".to_string(),
            "金額".to_string(),
            "理由".to_string(),
        ];

        let adjustment_table = DataTable::new("◆ 勘定補正 - 仮勘定整理 ◆", headers)
            .with_column_widths(vec![15, 20, 20, 15, 25]);

        let mut event_viewer = EventViewer::new();
        event_viewer.add_info("勘定補正画面を開きました");
        event_viewer.add_info("データを読み込んでいます...");

        Self {
            adjustment_table,
            event_viewer,
            loading_spinner: LoadingSpinner::new(),
            loading_state: LoadingState::Loading,
            animation_frame: 0,
        }
    }

    pub fn set_response(&mut self, response: AdjustAccountsResponse) {
        let mut data = Vec::new();

        // 勘定科目振替
        for reclassification in &response.reclassified_accounts {
            data.push(vec![
                "勘定振替".to_string(),
                reclassification.from_account.clone(),
                reclassification.to_account.clone(),
                format!("{:.0} {}", reclassification.amount, reclassification.currency),
                reclassification.reason.clone(),
            ]);
        }

        // 税効果調整
        for tax_effect in &response.tax_effect_adjustments {
            data.push(vec![
                "税効果".to_string(),
                "一時差異".to_string(),
                "繰延税金資産/負債".to_string(),
                format!(
                    "{:.0} {}",
                    tax_effect.deferred_tax_amount, tax_effect.deferred_tax_currency
                ),
                format!("税率: {:.1}%", tax_effect.tax_rate * 100.0),
            ]);
        }

        self.adjustment_table.set_data(data);
        self.loading_state = LoadingState::Loaded;
        self.event_viewer.add_info(format!(
            "勘定補正完了: 振替 {} 件、税効果 {} 件、調整仕訳 {} 件",
            response.reclassified_accounts.len(),
            response.tax_effect_adjustments.len(),
            response.adjustment_entries_created
        ));
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
        self.adjustment_table.select_next();
    }

    pub fn select_previous(&mut self) {
        self.adjustment_table.select_previous();
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
            .split(area);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3)])
            .split(chunks[0]);

        if self.loading_state == LoadingState::Loading {
            self.loading_spinner.render(
                frame,
                left_chunks[0],
                "勘定補正データを読み込んでいます...",
            );
        } else {
            self.adjustment_table.render(frame, left_chunks[0]);
        }

        self.render_status_bar(frame, left_chunks[1]);
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

impl Default for AccountAdjustmentPage {
    fn default() -> Self {
        Self::new()
    }
}
