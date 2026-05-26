pub mod app_state;

pub mod app_state_impl;

//------------------------Re-calling.
pub use app_state::AppState;

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_app_increment_counter() {
//         let mut app = AppState::default();
//         app.increment_counter();
//         assert_eq!(app.counter, 1)
//     }
//
//     #[test]
//     fn test_app_decrement_counter() {
//         let mut app = AppState::default();
//         app.decrement_counter();
//         assert_eq!(app.counter, 0);
//     }
// }
//

// /// > Tick event handler.
// pub fn tick(&self) {}
//
// /// > Quit application.
// pub fn quit(&mut self) {
//     self.should_quit = true;
// }
//
// /// # incrementing counter with default function
// ///  - function present on self.counter.checked_add(1)
// pub fn increment_counter(&mut self) {
//     if let Some(res) = self.counter.checked_add(1) {
//         self.counter = res;
//     }
// }
//
// /// # decrementing counter with default function
// ///  - function present on self.counter.checked_sub(1)
// pub fn decrement_counter(&mut self) {
//     if let Some(res) = self.counter.checked_sub(1) {
//         self.counter = res;
//     }
// }
