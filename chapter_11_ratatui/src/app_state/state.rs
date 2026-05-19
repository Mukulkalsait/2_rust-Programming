use std::io::Result;

use color_eyre::eyre::bail;
use crossterm::event::{KeyCode, KeyEvent};
/// FILE: /src/app_state/state.rs
use ratatui::{self, style::Stylize};

#[derive(Default, Debug)]
/// AppState + baiscally Main Application State.
pub struct AppState {
    counter: u8,
    exit: bool,
}

/// Implimentation fo each function under AppStore
impl AppState {
    //============================================= Fundamental implimentations,
    fn exit(&mut self) {
        self.exit = true;
    }
    fn increment_counter(&mut self) -> color_eyre::eyre::Result<()> {
        self.counter += 1;
        if self.counter > 2 {
            bail!("🐦‍🔥 Connter Overflow.")
        }
        Ok(())
    }
    fn decrement_coutner(&mut self) -> color_eyre::eyre::Result<()> {
        self.counter -= 1;
        Ok(())
    }
    //============================================= Fundamental implimentations,

    /// allow to build frame from ratatui frame.
    fn draw(&self, frame: &mut ratatui::Frame) {
        frame.render_widget(self, frame.area());
    }

    // acutal run function which draw terminal in loop.
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> color_eyre::eyre::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// TEST ======================================================================================
    fn handle_key_events(&mut self, key_event: crossterm::event::KeyEvent) -> color_eyre::eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_coutner(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
        Ok(())
    }

    /// handling properly the events,
    fn handle_events(&mut self) -> std::io::Result<()> {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(key_event) if key_event.kind == crossterm::event::KeyEventKind::Press => {
                self.handle_key_events(key_event)
            }
            _ => {}
        };
        Ok(())
    }
}

impl ratatui::widgets::Widget for &AppState {
    /// R:
    /// Render function which reander Block + Counter Text
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        // Y: TITLE:
        let title = ratatui::text::Line::from("🚀 Counter Tutorial App Costume: ".bold());

        // Y: INSTRUCTION:
        let instructions = ratatui::text::Line::from(vec![
            "🔻 Decrement ".into(),
            "<Left>".blue().bold(),
            "🔺 Increment ".into(),
            "<Right>".blue().bold(),
            "⏹️ Quit ".into(),
            "<Q>".blue().bold(),
        ]);

        // Y: BLOCK: contain Frame + Title + Instructionos
        let block = ratatui::widgets::Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(ratatui::symbols::border::THICK);

        // Y: Counter Text
        let conunter_text =
            ratatui::text::Text::from(vec![ratatui::text::Line::from(vec![" Value: ".into(), self.counter.to_string().yellow()])]);

        ratatui::widgets::Paragraph::new(conunter_text).centered().block(block).render(area, buf);
    }
}
