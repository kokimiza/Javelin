// Form - フォームコンポーネント
// 責務: 入力フォームの表示

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub struct FormField {
    pub label: String,
    pub value: String,
    pub is_focused: bool,
}

pub struct Form {
    title: String,
    fields: Vec<FormField>,
}

impl Form {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            fields: Vec::new(),
        }
    }

    pub fn add_field(mut self, label: String, value: String, is_focused: bool) -> Self {
        self.fields.push(FormField {
            label,
            value,
            is_focused,
        });
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let lines: Vec<Line> = self
            .fields
            .iter()
            .flat_map(|field| {
                let style = if field.is_focused {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                };

                vec![
                    Line::from(format!("{}: {}", field.label, field.value)).style(style),
                    Line::from(""),
                ]
            })
            .collect();

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.title.as_str()),
        );

        frame.render_widget(paragraph, area);
    }
}
