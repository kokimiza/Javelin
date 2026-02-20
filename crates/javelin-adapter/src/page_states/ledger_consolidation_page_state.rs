// LedgerConsolidationPageState - PageState implementation for ledger consolidation screen

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, Route},
    views::pages::LedgerConsolidationPage,
};

pub struct LedgerConsolidationPageState {
    page: LedgerConsolidationPage,
}

impl LedgerConsolidationPageState {
    pub fn new() -> Self {
        Self { page: LedgerConsolidationPage::new() }
    }
}

impl PageState for LedgerConsolidationPageState {
    fn route(&self) -> Route {
        Route::LedgerConsolidation
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        _controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            // Tick animation
            self.page.tick();

            // Render the page
            terminal
                .draw(|frame| {
                    self.page.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            // Handle events with timeout for animation updates
            if event::poll(std::time::Duration::from_millis(100))
                .map_err(crate::error::AdapterError::EventReadFailed)?
                && let Event::Key(key) =
                    event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => {
                        return Ok(NavAction::Back);
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.page.select_next();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.page.select_previous();
                    }
                    _ => {}
                }
            }
        }
    }

    fn on_navigation_error(&mut self, error_message: &str) {
        self.page.add_error(error_message);
    }
}

impl Default for LedgerConsolidationPageState {
    fn default() -> Self {
        Self::new()
    }
}
