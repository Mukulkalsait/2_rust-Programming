use ratatui::{self, style::Stylize};

use crate::app_state::state::AppState;

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
