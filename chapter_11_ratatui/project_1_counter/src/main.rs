// FILE: /src/main.rs
use color_eyre;
use ratatui;
//-------------EXT

mod app_state;
use app_state::state::AppState;
//------------INT

/// returingn colorful report for faluer or resualt.
fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    // init terminal
    let mut terminal = ratatui::init();

    // use AppState
    let app_resualt = AppState::default().run(&mut terminal);

    // restore terminal
    ratatui::restore();

    // return Result
    app_resualt
}

mod tests {
    use crate::app_state::state::AppState;
    use crossterm::event::KeyCode;

    #[test]
    fn handle_key_events() {
        let mut app = AppState::default();
        app.handle_key_events(KeyCode::Right.into()).unwrap();
        assert_eq!(app.counter, 1);

        app.handle_key_events(KeyCode::Left.into()).unwrap();
        assert_eq!(app.counter, 0);

        app.handle_key_events(KeyCode::Char('q').into()).unwrap();
        assert!(app.exit);
    }
}
