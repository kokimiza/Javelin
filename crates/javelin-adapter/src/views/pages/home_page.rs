// HomePage - ホーム画面
// 責務: エントリーポイント、メニュー表示

use crate::controller::RecordUserActionController;
use crate::error::{AdapterError, AdapterResult};
use crate::views::components::{EventInfo, EventViewer};
use crate::views::layouts::{Breadcrumb, KeyBinding, MainLayout};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use javelin_application::output_port::EventNotification;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};
use std::io;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct HomePage {
    layout: MainLayout,
    selected_index: usize,
    show_events: bool,
    event_viewer: EventViewer,
    events: Vec<EventInfo>, // 状態をHomePageで管理
    menu_items: Vec<String>,
    controller: Arc<RecordUserActionController>,
    event_receiver: mpsc::UnboundedReceiver<EventNotification>,
}

impl HomePage {
    pub fn new(
        controller: Arc<RecordUserActionController>,
        event_receiver: mpsc::UnboundedReceiver<EventNotification>,
    ) -> Self {
        let menu_items = vec![
            "原始記録登録処理（毎日）".to_string(),
            "元帳集約処理（週次）".to_string(),
            "月次決算処理".to_string(),
            "試算表生成".to_string(),
            "財務諸表生成".to_string(),
        ];

        Self {
            layout: MainLayout::new("Javelin")
                .with_breadcrumbs(vec![Breadcrumb::new("ホーム")])
                .with_key_bindings(vec![
                    KeyBinding::new("q", "終了"),
                    KeyBinding::new("e", "イベント表示切替"),
                    KeyBinding::new("hjkl", "移動"),
                    KeyBinding::new("Enter", "選択"),
                ]),
            selected_index: 0,
            show_events: true,
            event_viewer: EventViewer::new(),
            events: Vec::new(), // イベント状態を初期化
            menu_items,
            controller,
            event_receiver,
        }
    }

    /// ホーム画面を実行
    pub fn run(&mut self) -> AdapterResult<()> {
        // ターミナル初期化
        enable_raw_mode().map_err(AdapterError::RawModeEnableFailed)?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .map_err(AdapterError::TerminalInitFailed)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).map_err(AdapterError::TerminalInitFailed)?;

        // メインループ実行
        let result = self.run_main_loop(&mut terminal);

        // クリーンアップ
        disable_raw_mode().map_err(AdapterError::RawModeDisableFailed)?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .map_err(AdapterError::TerminalCleanupFailed)?;
        terminal
            .show_cursor()
            .map_err(AdapterError::TerminalCleanupFailed)?;

        result
    }

    fn run_main_loop<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> AdapterResult<()> {
        loop {
            // イベント通知を受信してイベントリストに追加
            while let Ok(notification) = self.event_receiver.try_recv() {
                let event = EventInfo::new(
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    notification.user,
                    notification.location,
                    notification.action,
                );
                self.events.push(event);
                // 最新10件のみ保持
                if self.events.len() > 10 {
                    self.events.remove(0);
                }
            }

            terminal
                .draw(|f| {
                    self.layout
                        .render(f, self.show_events, |frame, main_area, event_area| {
                            self.render_content(frame, main_area);
                            if let Some(event_area) = event_area {
                                // イベントの参照を渡す
                                self.event_viewer.render(frame, event_area, &self.events);
                            }
                        });
                })
                .map_err(|e| AdapterError::RenderingFailed(e.to_string()))?;

            // イベント処理 - ブロッキングで待機（キーリピート防止）
            let event = event::read().map_err(AdapterError::EventReadFailed)?;

            let Event::Key(key) = event else {
                // キーイベント以外は無視
                continue;
            };

            // KeyReleaseとRepeatイベントは無視（KeyPressのみ処理）
            if key.kind != crossterm::event::KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('e') => {
                    self.show_events = !self.show_events;
                }
                // Vimライクな移動
                KeyCode::Char('j') | KeyCode::Down => {
                    if self.selected_index < self.menu_items.len() - 1 {
                        self.selected_index += 1;
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if self.selected_index > 0 {
                        self.selected_index -= 1;
                    }
                }
                KeyCode::Enter => {
                    // 選択されたメニュー項目を実行（Controller経由）
                    let action = format!(
                        "メニュー項目 '{}' が選択されました（未実装）",
                        self.menu_items[self.selected_index]
                    );

                    // 非同期処理をブロッキング実行
                    let controller = Arc::clone(&self.controller);
                    let action_clone = action.clone();
                    tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(async {
                            let _ = controller
                                .record_action("system", "HomePage", action_clone)
                                .await;
                        })
                    });
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn render_content(&self, frame: &mut Frame, area: Rect) {
        // メニュー項目
        let menu_items: Vec<ListItem> = self
            .menu_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected_index {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let cursor = if i == self.selected_index {
                    "▶ "
                } else {
                    "  "
                };

                ListItem::new(Line::from(vec![
                    Span::styled(cursor, Style::default().fg(Color::Yellow)),
                    Span::styled(item, style),
                ]))
            })
            .collect();

        // メニュー描画
        let menu_list = List::new(menu_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("メニュー")
                .style(Style::default().fg(Color::Cyan)),
        );
        frame.render_widget(menu_list, area);
    }
}
