use std::collections::HashMap;

/// THREE “screens”:
///   -  Main: summary screen - all past key-value pairs entered
///   -  Editing: shown when the user wishes to create a new key-value pair
///   -  Exiting: displays a prompt asking if the user wants to output the key-value pairs they have entered.
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

/// ## **RATATUI** does not automatically redraw the screen also does not remember anything about what it drew last frame.
/// > we are responsible for handling all [**STATES & UPDATING WIDGETS**] to reflect changes.
///
/// - user input two strings in Editing mode
/// - a key and a value
/// - to track **WHICH FIELD THE USER IS CURRENTLY ENTERING**
pub enum CurrentlyEditing {
    Key,
    Value,
}

// #[derive(Default, Debug)]
///  App Struct: a struct storing real data which can be passed around where its needed.
pub struct AppState {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}
