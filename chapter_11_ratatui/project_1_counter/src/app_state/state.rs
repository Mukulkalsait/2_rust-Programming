/// FILE: /src/app_state/state.rs

#[derive(Default, Debug)]
/// AppState + baiscally Main Application State.
pub struct AppState {
    pub counter: u8,
    pub exit: bool,
}
