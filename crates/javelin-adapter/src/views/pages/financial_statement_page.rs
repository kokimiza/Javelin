// FinancialStatementPage - 財務諸表生成画面
// 責務: 制度開示資料作成

use javelin_application::dtos::GenerateFinancialStatementsResponse;
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

pub struct FinancialStatementPage {
    statement_table: DataTable,
    event_viewer: EventViewer,
    loading_spinner: LoadingSpinner,
    loading_state: LoadingState,
    animation_frame: usize,
}

impl FinancialStatementPage {
    pub fn new() -> Self {
        let headers = vec!["財務諸表".to_string(), "項目".to_string(), "金額".to_string()];

        let statement_table = DataTable::new("◆ 財務諸表生成 - 制度開示資料 ◆", headers)
            .with_column_widths(vec![25, 35, 20]);

        let mut event_viewer = EventViewer::new();
        event_viewer.add_info("財務諸表生成画面を開きました");
        event_viewer.add_info("データを読み込んでいます...");

        Self {
            statement_table,
            event_viewer,
            loading_spinner: LoadingSpinner::new(),
            loading_state: LoadingState::Loading,
            animation_frame: 0,
        }
    }

    pub fn set_response(&mut self, response: GenerateFinancialStatementsResponse) {
        let mut data = Vec::new();

        // 財政状態計算書
        let bs = &response.statement_of_financial_position;
        data.push(vec![
            "財政状態計算書".to_string(),
            "流動資産".to_string(),
            format!("{:.0} {}", bs.current_assets, bs.current_assets_currency),
        ]);
        data.push(vec![
            "財政状態計算書".to_string(),
            "非流動資産".to_string(),
            format!("{:.0} {}", bs.non_current_assets, bs.non_current_assets_currency),
        ]);
        data.push(vec![
            "財政状態計算書".to_string(),
            "流動負債".to_string(),
            format!("{:.0} {}", bs.current_liabilities, bs.current_liabilities_currency),
        ]);
        data.push(vec![
            "財政状態計算書".to_string(),
            "非流動負債".to_string(),
            format!("{:.0} {}", bs.non_current_liabilities, bs.non_current_liabilities_currency),
        ]);
        data.push(vec![
            "財政状態計算書".to_string(),
            "純資産".to_string(),
            format!("{:.0} {}", bs.equity, bs.equity_currency),
        ]);

        // 損益計算書
        let pl = &response.statement_of_profit_or_loss;
        data.push(vec![
            "損益計算書".to_string(),
            "売上高".to_string(),
            format!("{:.0} {}", pl.revenue, pl.revenue_currency),
        ]);
        data.push(vec![
            "損益計算書".to_string(),
            "売上原価".to_string(),
            format!("{:.0} {}", pl.cost_of_sales, pl.cost_of_sales_currency),
        ]);
        data.push(vec![
            "損益計算書".to_string(),
            "営業利益".to_string(),
            format!("{:.0} {}", pl.operating_profit, pl.operating_profit_currency),
        ]);
        data.push(vec![
            "損益計算書".to_string(),
            "当期純利益".to_string(),
            format!("{:.0} {}", pl.net_profit, pl.net_profit_currency),
        ]);

        // キャッシュフロー計算書
        let cf = &response.statement_of_cash_flows;
        data.push(vec![
            "CF計算書".to_string(),
            "営業活動CF".to_string(),
            format!("{:.0} {}", cf.operating_activities, cf.operating_activities_currency),
        ]);
        data.push(vec![
            "CF計算書".to_string(),
            "投資活動CF".to_string(),
            format!("{:.0} {}", cf.investing_activities, cf.investing_activities_currency),
        ]);
        data.push(vec![
            "CF計算書".to_string(),
            "財務活動CF".to_string(),
            format!("{:.0} {}", cf.financing_activities, cf.financing_activities_currency),
        ]);

        self.statement_table.set_data(data);
        self.loading_state = LoadingState::Loaded;

        let cross_check = if response.cross_check_passed {
            "合格"
        } else {
            "不合格"
        };
        self.event_viewer
            .add_info(format!("財務諸表生成完了: クロスチェック {}", cross_check));
        self.event_viewer.add_info(format!(
            "財務指標: ROE {:.2}%, ROA {:.2}%, 流動比率 {:.2}",
            response.financial_indicators.roe * 100.0,
            response.financial_indicators.roa * 100.0,
            response.financial_indicators.current_ratio
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
        self.statement_table.select_next();
    }

    pub fn select_previous(&mut self) {
        self.statement_table.select_previous();
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
                "財務諸表データを読み込んでいます...",
            );
        } else {
            self.statement_table.render(frame, left_chunks[0]);
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

impl Default for FinancialStatementPage {
    fn default() -> Self {
        Self::new()
    }
}
