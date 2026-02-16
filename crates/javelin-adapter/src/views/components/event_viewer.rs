// EventViewer - イベントビューアコンポーネント
// 責務: イベントソーシングのイベントを時系列で表示（状態保持なし）

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

/// イベント情報
#[derive(Clone, Debug)]
pub struct EventInfo {
    pub timestamp: String,
    pub user: String,
    pub location: String,
    pub action: String,
}

impl EventInfo {
    pub fn new(
        timestamp: impl Into<String>,
        user: impl Into<String>,
        location: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            timestamp: timestamp.into(),
            user: user.into(),
            location: location.into(),
            action: action.into(),
        }
    }
}

/// イベントビューア（状態なし）
pub struct EventViewer;

impl EventViewer {
    pub fn new() -> Self {
        Self
    }

    /// イベントリストを描画（状態を外部から受け取る）
    pub fn render(&self, frame: &mut Frame, area: Rect, events: &[EventInfo]) {
        if events.is_empty() {
            self.render_empty(frame, area);
        } else {
            self.render_events(frame, area, events);
        }
    }

    fn render_empty(&self, frame: &mut Frame, area: Rect) {
        let empty_msg = Paragraph::new("イベントはまだありません")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("イベントログ")
                    .style(Style::default().fg(Color::Cyan)),
            );

        frame.render_widget(empty_msg, area);
    }

    fn render_events(&self, frame: &mut Frame, area: Rect, events: &[EventInfo]) {
        // イベントを新しい順に表示
        let max_width = area.width.saturating_sub(4) as usize;

        let items: Vec<ListItem> = events
            .iter()
            .rev()
            .map(|event| {
                // 長いテキストを折り返し
                let action_text = &event.action;
                let action_lines: Vec<Line> = if action_text.len() > max_width {
                    action_text
                        .chars()
                        .collect::<Vec<_>>()
                        .chunks(max_width)
                        .map(|chunk| {
                            Line::from(Span::styled(
                                chunk.iter().collect::<String>(),
                                Style::default().fg(Color::White),
                            ))
                        })
                        .collect()
                } else {
                    vec![Line::from(Span::styled(
                        action_text,
                        Style::default().fg(Color::White),
                    ))]
                };

                let mut lines = vec![
                    Line::from(vec![Span::styled(
                        &event.timestamp,
                        Style::default().fg(Color::Yellow),
                    )]),
                    Line::from(vec![
                        Span::raw("👤 "),
                        Span::styled(&event.user, Style::default().fg(Color::Green)),
                        Span::raw(" @ "),
                        Span::styled(&event.location, Style::default().fg(Color::Cyan)),
                    ]),
                ];

                lines.extend(action_lines);
                lines.push(Line::from(""));

                ListItem::new(lines)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("イベントログ")
                .style(Style::default().fg(Color::Cyan)),
        );

        frame.render_widget(list, area);
    }
}

impl Default for EventViewer {
    fn default() -> Self {
        Self::new()
    }
}
