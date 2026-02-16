// Menu - メニューコンポーネント
// 責務: メニュー項目の表示と選択

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem},
};

pub struct MenuItem {
    pub label: String,
    pub key: char,
}

pub struct Menu {
    title: String,
    items: Vec<MenuItem>,
    selected_index: Option<usize>,
}

impl Menu {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
            selected_index: None,
        }
    }

    pub fn add_item(mut self, label: String, key: char) -> Self {
        self.items.push(MenuItem { label, key });
        self
    }

    pub fn selected_index(mut self, index: Option<usize>) -> Self {
        self.selected_index = index;
        self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if Some(i) == self.selected_index {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                ListItem::new(Line::from(format!("[{}] {}", item.key, item.label))).style(style)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.title.as_str()),
        );

        frame.render_widget(list, area);
    }
}
