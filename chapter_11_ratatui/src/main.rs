// FILE: /src/main.rs

mod app;

mod app_state;
use app_state::state::AppState;
use ratatui::{macros::ratatui_core::terminal, run};

/// returingn colorful report for faluer or resualt.
fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_resualt = AppState::default().run(&mut terminal);
    ratatui::restore();
    app_resualt
}
