// Table - テーブルコンポーネント
// 責務: データのテーブル表示

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table as RatatuiTable},
};

pub struct Table {
    title: String,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    selected_row: Option<usize>,
}

impl Table {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            headers: Vec::new(),
            rows: Vec::new(),
            selected_row: None,
        }
    }

    pub fn headers(mut self, headers: Vec<String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn rows(mut self, rows: Vec<Vec<String>>) -> Self {
        self.rows = rows;
        self
    }

    pub fn selected_row(mut self, index: Option<usize>) -> Self {
        self.selected_row = index;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let header_cells = self
            .headers
            .iter()
            .map(|h| Cell::from(h.as_str()).style(Style::default().fg(Color::Yellow)));
        let header = Row::new(header_cells)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .height(1);

        let rows = self.rows.iter().enumerate().map(|(i, row)| {
            let cells = row.iter().map(|c| Cell::from(c.as_str()));
            let style = if Some(i) == self.selected_row {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            Row::new(cells).style(style).height(1)
        });

        let widths = vec![
            ratatui::layout::Constraint::Percentage(100 / self.headers.len() as u16,);
            self.headers.len()
        ];

        let table = RatatuiTable::new(rows, widths).header(header).block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.title.as_str()),
        );

        frame.render_widget(table, area);
    }
}
