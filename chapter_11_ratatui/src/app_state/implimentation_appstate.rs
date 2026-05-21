use crate::app_state::state::AppState;

use color_eyre::eyre::{Ok, bail};
use crossterm::event::KeyCode;

/// Implimentation fo each function under AppStore
impl AppState {
    //  ======================================================================================

    /// Build Frame from ratatui::Frame
    /// Frame => Single Freame in area.
    fn draw_ratatui_terminal_draw(&self, ratatui_frame: &mut ratatui::Frame) {
        ratatui_frame.render_widget(self, ratatui_frame.area());
    }

    /// Acutal run function which draw terminal in loop. IMP:
    ///
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> color_eyre::eyre::Result<()> {
        while !self.exit {
            // .dray => draw full terminal screen.
            terminal.draw(|r_frame| self.draw_ratatui_terminal_draw(r_frame)).expect("failed to draw frame");

            // keybingins events.
            self.handle_events().expect("Failed to handle keybindings events.");
        }
        Ok(())
    }

    //  ======================================================================================
    /// handling properly the events,
    /// match with crossterm-event-read().expect {
    ///  // we match with
    ///     crossterm-event-key(key_event) // passing the event in
    ///     used if key_event.kind == crossterm-event-KeyEventKind::Press
    ///     => then only run key_event_handler
    ///     or
    ///     _=> Ok(()),
    ///
    /// }
    fn handle_events(&mut self) -> color_eyre::eyre::Result<()> {
        match crossterm::event::read().expect("failed to read crossterm events") {
            crossterm::event::Event::Key(key_event) if key_event.kind == crossterm::event::KeyEventKind::Press => {
                self.handle_key_events(key_event)
            }
            _ => Ok(()),
        }
    }

    /// Fairly simple one run the function from keycodes.
    /// The same crossterm-event-KeyEvent are passed with key_event and &self
    /// go to decleration fo  KeyCode and you will find every key.
    fn handle_key_events(&mut self, key_event: crossterm::event::KeyEvent) -> color_eyre::eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_coutner().unwrap(),
            KeyCode::Right => self.increment_counter().unwrap(),
            _ => {}
        }
        Ok(())
    }

    //============================================= Fundamental implimentations,
    fn exit(&mut self) {
        self.exit = true;
    }
    fn increment_counter(&mut self) -> color_eyre::eyre::Result<()> {
        self.counter += 1;
        if self.counter > 10 {
            bail!("🐦‍🔥 Connter Overflow.")
        }
        Ok(())
    }
    fn decrement_coutner(&mut self) -> color_eyre::eyre::Result<()> {
        if self.counter > 0 {
            self.counter -= 1;
        } else {
            bail!("🐦‍🔥 Connter Overflow.")
        }
        Ok(())
    }
    //============================================= Fundamental implimentations,
}
