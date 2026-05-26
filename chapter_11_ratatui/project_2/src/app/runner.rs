use std::io;

use crate::app::{self, AppState};
use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{Terminal, backend::CrosstermBackend};

pub fn run_app(t: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app: &mut AppState) -> Result<()> {
    loop {
        let text = ratatui::widgets::Paragraph::new(format!(
            "Screen: {}\nCounter: {}\nPress: j/k(increment/decrement), q(quit)",
            app.screen, app.counter
        ));

        // ========================DRAWING  DX:
        t.draw(|frame| frame.render_widget(text, frame.area())).expect("Failed to draw Termanal Frame");

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') => app.counter += 1,
                    KeyCode::Char('k') => {
                        if app.counter > 0 {
                            app.counter -= 1
                        } else {
                            panic!("Fuck Microsoft");
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

// ANOTHER APPROCH:
//
// pub fn run_app<B.: ratatui::backend::Backend>(t: &mut ratatui::Terminal<B>, app: &mut AppState) -> Result<bool>
// where io::Error: From<B.::Error>,
// { loop { t.draw(|frame| draw(frame, app))?; } }
