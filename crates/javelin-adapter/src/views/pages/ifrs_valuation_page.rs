// IfrsValuationPage - IFRS評価画面
// 責務: 見積会計・公正価値測定

use javelin_application::dtos::ApplyIfrsValuationResponse;
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

pub struct IfrsValuationPage {
    valuation_table: DataTable,
    event_viewer: EventViewer,
    loading_spinner: LoadingSpinner,
    loading_state: LoadingState,
    animation_frame: usize,
}

impl IfrsValuationPage {
    pub fn new() -> Self {
        let headers = vec![
            "評価項目".to_string(),
            "種別".to_string(),
            "帳簿価額".to_string(),
            "評価額".to_string(),
            "差額".to_string(),
        ];

        let valuation_table = DataTable::new("◆ IFRS評価 - 公正価値測定 ◆", headers)
            .with_column_widths(vec![20, 15, 15, 15, 15]);

        let mut event_viewer = EventViewer::new();
        event_viewer.add_info("IFRS評価画面を開きました");
        event_viewer.add_info("データを読み込んでいます...");

        Self {
            valuation_table,
            event_viewer,
            loading_spinner: LoadingSpinner::new(),
            loading_state: LoadingState::Loading,
            animation_frame: 0,
        }
    }

    pub fn set_response(&mut self, response: ApplyIfrsValuationResponse) {
        let mut data = Vec::new();

        // 予想信用損失
        if response.expected_credit_loss > 0.0 {
            data.push(vec![
                "予想信用損失".to_string(),
                "ECL".to_string(),
                "-".to_string(),
                format!(
                    "{:.0} {}",
                    response.expected_credit_loss, response.expected_credit_loss_currency
                ),
                format!("{:.0}", response.expected_credit_loss),
            ]);
        }

        // 公正価値調整
        for adjustment in &response.fair_value_adjustments {
            let diff = adjustment.adjustment;
            data.push(vec![
                adjustment.financial_asset.clone(),
                "公正価値".to_string(),
                format!("{:.0}", adjustment.book_value),
                format!("{:.0}", adjustment.fair_value),
                format!("{:+.0}", diff),
            ]);
        }

        // 減損損失
        for impairment in &response.impairment_losses {
            data.push(vec![
                impairment.asset.clone(),
                "減損".to_string(),
                format!("{:.0}", impairment.carrying_amount),
                format!("{:.0}", impairment.recoverable_amount),
                format!("-{:.0}", impairment.impairment_loss),
            ]);
        }

        // 棚卸資産評価減
        for writedown in &response.inventory_write_downs {
            data.push(vec![
                writedown.item.clone(),
                "評価減".to_string(),
                format!("{:.0}", writedown.cost),
                format!("{:.0}", writedown.net_realizable_value),
                format!("-{:.0}", writedown.write_down_amount),
            ]);
        }

        self.valuation_table.set_data(data);
        self.loading_state = LoadingState::Loaded;
        self.event_viewer.add_info(format!(
            "IFRS評価完了: 公正価値 {} 件、減損 {} 件、評価減 {} 件、リース {} 件",
            response.fair_value_adjustments.len(),
            response.impairment_losses.len(),
            response.inventory_write_downs.len(),
            response.lease_measurements.len()
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
        self.valuation_table.select_next();
    }

    pub fn select_previous(&mut self) {
        self.valuation_table.select_previous();
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
                "IFRS評価データを読み込んでいます...",
            );
        } else {
            self.valuation_table.render(frame, left_chunks[0]);
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

impl Default for IfrsValuationPage {
    fn default() -> Self {
        Self::new()
    }
}
