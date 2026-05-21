use color_eyre::eyre::{Ok, bail};
use crossterm::event::KeyCode;
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
        };
        Ok(())
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

impl ratatui::widgets::Widget for &AppState {
    /// R: where exactly is this thing geting called form?
    ///
    /// provided (
    ///     self
    ///     rti-layout-rect (RectanbleBox)
    ///     Buffer (basically positning of Rect area with cordaanets)
    /// )
    ///
    /// we created
    ///  A. Block with =>
    ///     1. Title => Title of program
    ///     2. Instruction => Bottom Line Instruction
    ///
    ///  B.COUNTER TEXT ------------------------Y:
    ///
    ///   a. instead of string.from we used   ---- Text::from( Vec![ Line::from ])
    ///   b. everytime used Text::from/Line::from in ratatui
    ///      we need. vec![]
    ///   c. now THIS VEC CONTAINS:
    ///        1. " Value: ".into().red(),
    ///        2. self.counter.to_string().yellow()
    ///
    ///
    ///
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        // --------------------------------------------------------------------------------------------------
        // Y: TITLE:
        let title = ratatui::text::Line::from("🚀 Counter Tutorial App Costume: ".bold());

        // Y: INSTRUCTION:
        let instructions = ratatui::text::Line::from(vec![
            "🔻 Decrement ".to_string().red(),
            "<Left>".blue().bold(),
            "🔺 Increment ".to_string().green(),
            "<Right>".blue().bold(),
            "⏹️ Quit ".to_string().white(),
            "<Q>".blue().bold(),
        ]);

        // --------------------------------------------------------------------------------------------------
        // Y: BLOCK: contain Frame + Title + Instructionos
        let block = ratatui::widgets::Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(ratatui::symbols::border::THICK);

        // ==================================================================================================

        let conunter_text = ratatui::text::Text::from(vec![ratatui::text::Line::from(vec![
            "Value: ".to_string().red().bold(),
            self.counter.to_string().yellow().bold(),
        ])]);

        // ==================================================================================================

        ratatui::widgets::Paragraph::new(conunter_text).centered().block(block).render(area, buf);
    }
}
