use color_eyre::eyre::Result;
use crossterm::{
    ExecutableCommand,
    event::DisableMouseCapture,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use std::io::stdout;

/// Gurad that restore terminal when its drop.
///  - if exited
///  - if Paniced
///  
/// > Impled with Drop trait.
/// costume fn setup_error_handler() to handle everyting,
/// + color_eyre is added directly here handler fn.
pub struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = stdout().execute(LeaveAlternateScreen);
        let _ = stdout().execute(DisableMouseCapture);
    }
}

/// this fn replaces the panic! hook with costume panic hook,
/// which first cleanup the termainl then shows panic messages.
///
///
///
/// > std::panic::set_hook(Box::new(move |panic_info| {...}
/// Box<> for heap and move to take ownership of original_hook.
/// this is costume hook which run orignal hook at end,
/// but before that adds terminal cleanup
pub fn setup_error_handler() -> Result<()> {
    // IMP: instead fo using panic hook color_eyre comes with costume colorfulled panic.
    //
    // both panic + eye_hook
    let (panic_hook, eye_hook) = color_eyre::config::HookBuilder::default().display_env_section(false).into_hooks();
    eye_hook.install().expect("Faild to install eye_hook");

    // let original_hook = std::panic::take_hook(); // saves orignal hook of panic.(the one rust provide)
    let original_hook = panic_hook.into_panic_hook(); // instead panic of color_eyre. Y: 2.

    std::panic::set_hook(Box::new(move |panic_info| {
        // Restore terminal BEFORE printing panic
        let _ = disable_raw_mode();
        let _ = stdout().execute(LeaveAlternateScreen);
        let _ = stdout().execute(DisableMouseCapture);
        // Then call original hook
        original_hook(panic_info);
    })); // replaces it with the costume one. Y: 2

    Ok(())
}
