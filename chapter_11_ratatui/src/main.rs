// FILE: /src/main.rs

mod app;

mod app_state;
use app_state::state::AppState;

/// returingn colorful report for faluer or resualt.
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // ratatui::run(termapp)?; // helo world
    ratatui::run(|terminal| AppState::default().run(terminal))?;
    Ok(())
}
