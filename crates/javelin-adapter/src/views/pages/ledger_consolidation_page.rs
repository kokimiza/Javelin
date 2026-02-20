// LedgerConsolidationPage - 元帳集約画面
// 責務: 週次：総勘定元帳への転記処理

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
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

pub struct LedgerConsolidationPage {
    consolidation_table: DataTable,
    event_viewer: EventViewer,
    loading_spinner: LoadingSpinner,
    loading_state: LoadingState,
    animation_frame: usize,
}

impl LedgerConsolidationPage {
    pub fn new() -> Self {
        let headers = vec![
            "勘定科目".to_string(),
            "転記件数".to_string(),
            "借方合計".to_string(),
            "貸方合計".to_string(),
            "状態".to_string(),
        ];

        let consolidation_table = DataTable::new("◆ 元帳集約 - 総勘定元帳への転記 ◆", headers)
            .with_column_widths(vec![25, 12, 15, 15, 15]);

        let mut event_viewer = EventViewer::new();
        event_viewer.add_info("元帳集約画面を開きました");
        event_viewer.add_info("週次転記処理を準備しています...");

        Self {
            consolidation_table,
            event_viewer,
            loading_spinner: LoadingSpinner::new(),
            loading_state: LoadingState::Loading,
            animation_frame: 0,
        }
    }

    pub fn set_loaded(&mut self) {
        self.loading_state = LoadingState::Loaded;
        self.event_viewer.add_info("元帳集約データを読み込みました");
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
        self.consolidation_table.select_next();
    }

    pub fn select_previous(&mut self) {
        self.consolidation_table.select_previous();
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
                "元帳集約データを読み込んでいます...",
            );
        } else {
            self.consolidation_table.render(frame, left_chunks[0]);
        }

        self.render_status_bar(frame, left_chunks[1]);
        self.event_viewer.render(frame, chunks[1]);
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let status_text = match &self.loading_state {
            LoadingState::Loading => "読込中...",
            LoadingState::Loaded => "準備完了",
            LoadingState::Error(_) => "エラー",
        };

        let status_line = Line::from(vec![
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("Esc", Style::default().fg(Color::Cyan)),
            Span::styled("]戻る [", Style::default().fg(Color::DarkGray)),
            Span::styled("j/k", Style::default().fg(Color::Cyan)),
            Span::styled("]選択 | ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("状態: {}", status_text),
                Style::default().fg(if matches!(self.loading_state, LoadingState::Error(_)) {
                    Color::Red
                } else {
                    Color::Green
                }),
            ),
        ]);

        let status_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray));

        let status_paragraph = Paragraph::new(status_line).block(status_block);
        frame.render_widget(status_paragraph, area);
    }
}

impl Default for LedgerConsolidationPage {
    fn default() -> Self {
        Self::new()
    }
}
