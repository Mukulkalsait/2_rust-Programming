use std::io::{self, stderr};

// FILE: /src/main.rs
use color_eyre;
use crossterm::event::{DisableMouseCapture, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{self, LeaveAlternateScreen, disable_raw_mode};
use ratatui::Terminal;
use ratatui::backend::Backend;
//----------EXT
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};

//---------INT
pub mod app;
use crate::app::AppState;
use crate::app::app_state::{CurrentScreen, CurrentlyEditing};

pub mod logic;
pub mod user_interface;
use crate::user_interface::ui;

fn main() -> color_eyre::eyre::Result<()> {
    // Enable raw mode + stgandard error Y: 1.
    enable_raw_mode().expect("failed to enable raw mode");
    let mut stderr = std::io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture).expect("failed");

    // starting ratatui backend with crossterm  Y: 2
    let backend = ratatui::backend::CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend).expect("failed to create new term");

    // creating new appsate and runing terminal with that AppState Y: 3
    let mut app = AppState::new();
    let res = run_app(&mut terminal, &mut app);

    // ratatui changes appstate or terminal so before exit we RESTORE  Y: 4
    disable_raw_mode()?; // disable raw mode
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?; // 🛑 altscreen & mousecapture 
    terminal.show_cursor()?; // term show curosr.

    // whenevery TUI ends the terminal hangs in bad shape to solve that, we have this.
    // The if statement at the end of boilerplate checks if the run_app function errored, or if it returned an Ok state. If it returned an Ok state, we need to check if we should print the json.
    // If we call our print function before we call execute!(LeaveAlternateScreen), our prints will be rendered on an old screen and lost when we leave the alternate screen. (For more information on how this works, read the Crossterm documentation)
    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("Error: {:?}", err);
    }
    Ok(())
}

/// function generic across the ratatui::backend::Backend.  
/// This trait approach allows us to make our code backend agnostic.
/// The method accepts an object of type Terminal which implements the ratatui::backend::Backend trait.
/// This trait includes the three (four counting the TestBackend) officially supported backends included in ratatui.
/// allows 3rd party backends Impolementation.
/// run_app requires mutable borrow to AppState object.
/// The return type is still io::Result<bool>,
/// but there is one extra bound:
/// io::Error: From<B.::Error>.
/// needed because Terminal::draw() returns a Result
/// using the backend’s error type, and ? can only propagate that error if it can be converted into io::Error.
/// With that conversion in place,
/// run_app can use ? on both event::read() and terminal.draw(...),
/// while still returning Ok(true) or Ok(false) to indicate whether the finished json should be printed.
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> io::Error
where
    io::Error: From<B::Error>,
{
    loop {
        // terminal => Terminal<Backend>
        // draw coammdna to draw and pass frame f to ui.
        terminal.draw(|f| ui(f, app))?;

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            // if key is released dont do anyting.
            if key.kind == crossterm::event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                // while in main screen Y: MAIN
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        // editin on
                        app.current_screen = CurrentScreen::Editing;
                        // start editing key
                        app.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    // exit
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    // anyting else do noting.
                    _ => {}
                },
                // while Editing Y: EDITING
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('q') | KeyCode::Char('n') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                // while Exiting Y: EXITING
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {}
                        }
                    }
                    KeyCode::Backspace => {}
                    KeyCode::Esc => {}
                    KeyCode::Tab => {}
                    KeyCode::Char(value) => {}
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
