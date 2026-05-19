use ratatui;

// returning crossterm backend.
pub fn termapp(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render); // draw single frame to whole terminal

        // handling raw mode breaking out.
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut ratatui::Frame) {
    frame.render_widget("Hellow WORLD", frame.area()); // widget rendering to the whole area.
}
