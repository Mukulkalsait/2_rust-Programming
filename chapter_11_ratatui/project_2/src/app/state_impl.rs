use crate::AppState;

impl AppState {
    pub fn new() -> Self {
        Self { screen: String::from("Main"), counter: 0 }
    }
}
