use super::{AppState, Color};

pub fn handle_change_color(
    color: &String,
    current_state: AppState,
) -> AppState {
    if color == "BLUE" {
        AppState { color: Color::Blue, ..current_state }

    } else if color == "RED" {
        AppState { color: Color::Red, ..current_state }

    } else if color == "YELLOW" {
        AppState { color: Color::Yellow, ..current_state }

    } else {
        println!("SOMETHING_DEFAULTED!!!");
        AppState { ..current_state }
    }
}
