use std::collections::HashMap;

use serde::de::value;

use crate::{
    AppState,
    app::app_state::{CurrentScreen, CurrentlyEditing},
};

impl AppState {
    /// ### Constructor function for AppState.
    /// easy to create AppState.
    pub fn new() -> Self {
        AppState {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    /// ### Called when the user saves a key-value pair
    ///  1. adds the two stored variables to the key-value pairs HashMap
    ///  2. resets the status of all of the editing variables.
    ///     * by calling String::new() method on both inputs,
    ///     * seting currently_editing to None
    pub fn save_key_value(&mut self) {
        // storing
        self.pairs.insert(self.key_input.clone(), self.value_input.clone());

        // reseting
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    /// ### Here we check if something is currently being edited, and if it is,
    /// > Toggle of editing between (key and vlaue)
    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = self.currently_editing {
            match edit_mode {
                // if key is editing edit value.
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                // if value is editing edit key.
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    /// ### A Convenience function to print out the serialized json from all of our key-value pairs.
    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{output}");
        Ok(())
    }
}
