use crate::app::{AppState, app_state::CurrentScreen, app_state::CurrentlyEditing};
use ratatui::{self, style::Color};

pub fn ui(frame: &mut ratatui::Frame, app: &AppState) {
    // default layout chunk with
    //  - Direction => Vertical
    //  - Constrain => 3-1-3
    //  - split => frmae area.
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([ratatui::layout::Constraint::Length(3), ratatui::layout::Constraint::Min(1), ratatui::layout::Constraint::Length(3)])
        .split(frame.area());

    let title_block = ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).style(ratatui::style::Style::default());
    let title = ratatui::widgets::Paragraph::new(ratatui::text::Line::styled(
        "Create New Json",
        ratatui::style::Style::default().fg(ratatui::style::Color::Green),
    ))
    .block(title_block);
    frame.render_widget(title, chunks[0]);

    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => {
                ratatui::text::Span::styled("Normal Mode", ratatui::style::Style::default().fg(ratatui::style::Color::Green))
            }
            CurrentScreen::Editing => {
                ratatui::text::Span::styled("Editing Mode", ratatui::style::Style::default().fg(ratatui::style::Color::Yellow))
            }
            CurrentScreen::Exiting => {
                ratatui::text::Span::styled("Exiting", ratatui::style::Style::default().fg(ratatui::style::Color::LightRed))
            }
        }
        .to_owned(),
        ratatui::text::Span::styled(" | ", ratatui::style::Style::default().fg(ratatui::style::Color::White)),
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        ratatui::text::Span::styled("Editing Json Key", ratatui::style::Style::default().fg(ratatui::style::Color::Green))
                    }
                    CurrentlyEditing::Value => ratatui::text::Span::styled(
                        "Editing Json Value",
                        ratatui::style::Style::default().fg(ratatui::style::Color::LightGreen),
                    ),
                }
            } else {
                ratatui::text::Span::styled("Not Editing Anyting", ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray))
            }
        },
    ];

    let mode_footer = ratatui::widgets::Paragraph::new(ratatui::text::Line::from(current_navigation_text))
        .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL));

    let current_key_hint = {
        match app.current_screen {
            CurrentScreen::Main => {
                ratatui::text::Span::styled("(q) to quit / (e) to make new pari", ratatui::style::Style::default().fg(Color::Red))
            }
            CurrentScreen::Editing => ratatui::text::Span::styled(
                "(ESC) to cancle / (Tab) to swithc boxes / Enter to complete.",
                ratatui::style::Style::default().fg(ratatui::style::Color::Red),
            ),
            CurrentScreen::Exiting => ratatui::text::Span::styled(
                "(q) to quit / (e) to make new pair",
                ratatui::style::Style::default().fg(ratatui::style::Color::Red),
            ),
        }
    };

    let key_notes_footer = ratatui::widgets::Paragraph::new(ratatui::text::Line::from(current_key_hint))
        .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL));

    let footer_chunk = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([ratatui::layout::Constraint::Percentage(50), ratatui::layout::Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunk[0]);
    frame.render_widget(key_notes_footer, footer_chunk[1]);
}

/// Center rectangle part (Universal Popup function part.)
fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    // created popout part here.
    let popup_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Percentage(100 - percent_y) / 2,
            ratatui::layout::Constraint::Percentage(percent_y),
            ratatui::layout::Constraint::Percentage(100 - percent_y) / 2,
        ])
        .split(r);

    // u sed popout laoyut ihnside hrere
    ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage(100 - percent_x),
            ratatui::layout::Constraint::Percentage(percent_x),
            ratatui::layout::Constraint::Percentage(100 - percent_x) / 2,
        ])
        .split(popup_layout[1])[1]
}
