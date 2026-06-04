// FILE: src/main.rs

use crossterm::ExecutableCommand;
// ─── Crate Imports ────────────────────────────────────────────────────
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};

use ratatui::prelude::*;
// use ratatui::widgets::Paragraph;

use color_eyre::eyre::Result;
// ─── Standard Library ─────────────────────────────────────────────────
use std::io::stdout;
// ─── Local Modules ────────────────────────────────────────────────────
mod app;
use app::state::AppState;

mod handlers;
use handlers::terminal_guard::setup_error_handler;
mod ui;
// ─── END: =============================================================

fn main() -> Result<()> {
    print!("hello");
    // COSTUME HANDLER (handles both Panic + color_eyre) Y: 1
    setup_error_handler().expect("Faild to handle error");

    // G: Enabling Terminal HACKING:
    enable_raw_mode().expect("Raw Mode Fail.");
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;

    // Insure run on panic.
    let _guard = handlers::terminal_guard::TerminalGuard;
    // RUN APPLICATION:
    let mut terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout)).expect("faild to create terminal");
    let mut app = AppState::new();
    let result = app::runner::run_app(&mut terminal, &mut app); // R: RUN APP.

    // NOT NEEDED COS WE HAVE Y: TerminalGuard
    // ----------------------
    // disable_raw_mode().expect("Faild to disable_raw_mode");
    // terminal.backend_mut().execute(LeaveAlternateScreen).expect("Faild to LeaveAlternateScreen");
    // terminal.show_cursor().expect("Faild to show cursor back.");

    result
}
